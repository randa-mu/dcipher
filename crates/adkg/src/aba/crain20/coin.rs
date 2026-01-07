//! Functions used for the coin toss protocol

use crate::aba::crain20::ecdh_coin_toss::Coin;
use crate::aba::crain20::messages::{AbaMessage, CoinEvalMessage};
use crate::aba::crain20::{AbaCrain20Instance, AbaError, AbaState, CoinKeys};
use crate::helpers::PartyId;
use crate::network::broadcast_with_self;
use dcipher_network::TransportSender;
use digest::crypto_common::rand_core::CryptoRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{Level, error, event};

pub trait CoinToss: 'static + Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    type SecretKey: Send + Sync;
    type PublicParams: Send + Sync;

    type Eval: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    fn eval(
        sk: &Self::SecretKey,
        params: &Self::PublicParams,
        sid: usize,
        round: u8,
        rng: &mut (impl CryptoRng + RngCore),
    ) -> Result<Self::Eval, Self::Error>;

    fn get_coin<'a, I>(
        evals: I,
        params: &Self::PublicParams,
        sid: usize,
        round: u8,
    ) -> Result<Coin, Self::Error>
    where
        I: IntoIterator<Item = (PartyId, &'a Self::Eval)> + 'a,
        Self::Eval: 'a;
}

impl<CT, CK, TS> AbaCrain20Instance<CT, CK, TS>
where
    CT: CoinToss,
    CK: Send + Into<CoinKeys<CT>> + 'static,
    TS: TransportSender<Identity = PartyId>,
{
    /// Try to get the output from the coin keys receiver, return an error otherwise.
    pub(super) async fn get_coin_keys(
        &self,
        r: u8,
        coin_keys_receiver: oneshot::Receiver<CK>,
    ) -> Result<CK, AbaError> {
        event!(
            Level::DEBUG,
            "Node `{}` at round `{r}` has not yet obtained keys for common coin protocol, waiting.",
            self.config.id
        );

        // Return coin_keys if sender not dropped, err otherwise
        match coin_keys_receiver.await {
            Ok(coin_keys) => {
                event!(
                    Level::DEBUG,
                    "Node `{}` at round `{r}` obtained keys for common coin protocol",
                    self.config.id
                );

                Ok(coin_keys)
            }
            Err(_) => {
                error!(
                    "Node `{}` at round `{r}` failed to obtain common coin input through channel: sender dropper. Aborting ABA.",
                    self.config.id
                );
                Err(AbaError::CoinKeysRecv)
            }
        }
    }

    /// Try to generate and send a partial coin evaluation, or return an error otherwise.
    pub(super) async fn send_coin_eval<RNG>(
        &self,
        r: u8,
        coin_keys: &CoinKeys<CT>,
        rng: &mut RNG,
    ) -> Result<(), Box<AbaError>>
    where
        RNG: RngCore + CryptoRng,
    {
        let eval = CT::eval(
            &coin_keys.sk,
            &coin_keys.params,
            usize::from(self.sid),
            r,
            rng,
        )
        .map_err(|e| {
            AbaError::CoinToss(e.into(), "failed to generate coin toss evaluation: {e}")
        })?;

        let msg_coin_eval = AbaMessage::CoinEval(CoinEvalMessage::new(eval, r).unwrap());

        if let Err(e) =
            broadcast_with_self(&msg_coin_eval, &self.config.retry_strategy, &self.sender).await
        {
            error!(
                "Node `{}` failed to broadcast coin eval message: {e:?}",
                self.config.id
            );
        }

        Ok(())
    }

    /// Wait for enough evaluations and try to recover a common coin. Returns an error if too many evaluations are invalid.
    pub(super) async fn get_coin(
        &self,
        r: u8,
        state: &Arc<AbaState<CT>>,
        coin_keys: &CoinKeys<CT>,
    ) -> Result<Coin, Box<AbaError>> {
        loop {
            // Wait until we have enough valid partial coins evals for the current round
            event!(
                Level::DEBUG,
                "Node `{}` at round `{r}` waiting for coin evaluations",
                self.config.id
            );
            state.notify_enough_coin_evals.notified(r).await;

            // mutex locked for the entire duration, either that or cloning evals
            let coin_evals = state.coin_evals.lock().await;
            let Some((senders, evals)) = coin_evals.get_all(&r) else {
                event!(
                    Level::DEBUG,
                    "Node `{}` at round `{r}` received coin evals notifications while not having evals",
                    self.config.id
                );
                continue;
            };

            let evals_len = evals.len();
            if evals_len < self.config.t + 1 {
                event!(
                    Level::DEBUG,
                    "Node `{}` at round `{r}` does not have enough evals: {} < {}",
                    self.config.id,
                    evals.len(),
                    self.config.t
                );
                continue; // not enough evals for this round yet
            };

            // Try to get and return the common coin
            match CT::get_coin(
                senders.into_iter().zip(evals),
                &coin_keys.params,
                self.sid.into(),
                r,
            ) {
                Ok(coin) => return Ok(coin),
                Err(e) => {
                    // Failed to obtain the common coin, we either continue if we don't have all evals yet, or we abort
                    event!(
                        Level::WARN,
                        "Node `{}` at round `{r}` failed to obtain a common coin due to invalid eval(s): {e:?}",
                        self.config.id
                    );

                    if evals_len < self.config.n {
                        continue;
                    } else {
                        event!(
                            Level::ERROR,
                            "Node `{}` at round `{r}` failed to obtain a common coin with n evals: {e:?}. Aborting ABA with error.",
                            self.config.id
                        );
                        Err(AbaError::CoinToss(
                            e.into(),
                            "failed to obtain common coin with all evals",
                        ))?
                    }
                }
            }
        }
    }
}
