//! Functions used for the coin toss protocol

use crate::aba::crain20::ecdh_coin_toss::{Coin, EcdhCoinTossEval};
use crate::aba::crain20::messages::{AbaMessage, CoinEvalMessage};
use crate::aba::crain20::{AbaCrain20Instance, AbaError, AbaState, CoinKeys};
use crate::helpers::PartyId;
use crate::network::broadcast_with_self;
use ark_ec::CurveGroup;
use dcipher_network::TransportSender;
use digest::core_api::BlockSizeUser;
use digest::crypto_common::rand_core::CryptoRng;
use digest::{DynDigest, FixedOutputReset};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{Level, error, event};
use utils::hash_to_curve::HashToCurve;
use utils::serialize::fq::FqSerialize;
use utils::serialize::point::PointSerializeCompressed;

impl<CG, CK, H, TS> AbaCrain20Instance<CG, CK, H, TS>
where
    CG: CurveGroup + Copy + HashToCurve + PointSerializeCompressed,
    CG::ScalarField: FqSerialize,
    EcdhCoinTossEval<CG, H>: for<'de> Deserialize<'de>,
    CK: Send + Into<CoinKeys<CG>> + 'static,
    H: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
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
        coin_keys: &CoinKeys<CG>,
        rng: &mut RNG,
    ) -> Result<(), Box<AbaError>>
    where
        RNG: RngCore + CryptoRng,
    {
        let eval = EcdhCoinTossEval::<CG, H>::eval(
            &coin_keys.sk,
            &Self::coin_input(usize::from(self.sid), &coin_keys.combined_vk, r)?,
            &self.config.g,
            rng,
        )
        .map_err(|e| AbaError::CoinToss(e, "failed to generate coin toss evaluation: {e}"))?;

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
        state: &Arc<AbaState<CG, H>>,
        coin_keys: &CoinKeys<CG>,
    ) -> Result<Coin, Box<AbaError>> {
        // Get the input of the common coin protocol
        let coin_input = Self::coin_input(
            usize::from(self.sid),
            &coin_keys.combined_vk.into_affine().into(),
            r,
        )?;

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

            if evals.len() < self.config.t + 1 {
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
            let coin_vks: Vec<_> = senders.iter().map(|&j| coin_keys.vks[j]).collect();
            match EcdhCoinTossEval::get_coin(
                &evals,
                &senders,
                &coin_vks,
                &coin_input,
                &self.config.g,
                self.config.t + 1,
            ) {
                Ok(coin) => return Ok(coin),
                Err(e) => {
                    // Failed to obtain the common coin, we either continue if we don't have all evals yet, or we abort
                    event!(
                        Level::WARN,
                        "Node `{}` at round `{r}` failed to obtain a common coin due to invalid eval(s): {e:?}",
                        self.config.id
                    );

                    if evals.len() < self.config.n {
                        continue;
                    } else {
                        event!(
                            Level::ERROR,
                            "Node `{}` at round `{r}` failed to obtain a common coin with n evals: {e:?}. Aborting ABA with error.",
                            self.config.id
                        );
                        Err(AbaError::CoinToss(
                            e,
                            "failed to obtain common coin with all evals",
                        ))?
                    }
                }
            }
        }
    }

    /// Get the input to the common coin.
    fn coin_input(sid: usize, combined_vk: &CG, round: u8) -> Result<Vec<u8>, Box<AbaError>> {
        CoinInput {
            combined_vk: *combined_vk,
            sid,
            round,
        }
        .serialize()
    }
}

/// Structure used to serialize the input of the coin
#[derive(Serialize)]
#[serde(bound(serialize = "CG: PointSerializeCompressed",))]
struct CoinInput<CG> {
    #[serde(with = "utils::serialize::point::base64")]
    combined_vk: CG,
    sid: usize,
    round: u8,
}

impl<CG> CoinInput<CG>
where
    CG: PointSerializeCompressed,
{
    fn serialize(&self) -> Result<Vec<u8>, Box<AbaError>> {
        bson::to_vec(&self)
            .map_err(|e| AbaError::BsonSer(e, "failed to serialize CoinInput to bson").into())
    }
}
