//! Implementation of the Tyler Crain's Asynchronous Byzantine Agreement described in https://arxiv.org/pdf/2002.08765.
//! We specifically implement the Good-Case-Coin-Free variant described in https://eprint.iacr.org/2021/1591.pdf, Appendix B.

use futures::StreamExt;
mod ecdh_coin_toss;
pub mod messages;

use crate::aba::{Aba, AbaConfig, Estimate};
use crate::helpers::{PartyId, SessionId};
use crate::network::{RetryStrategy, broadcast_with_self};
use ark_ec::CurveGroup;
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::{ReceivedMessage, Transport, TransportSender};
use digest::core_api::BlockSizeUser;
use digest::crypto_common::rand_core::CryptoRng;
use digest::{DynDigest, FixedOutputReset};
use ecdh_coin_toss::{Coin, EcdhCoinTossError, EcdhCoinTossEval};
use futures::future::Either;
use messages::{AbaMessage, AuxStage, CoinEvalMessage, View};
use messages::{AuxiliaryMessage, AuxiliarySetMessage, EstimateMessage};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::pin::pin;
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap, btree_map::Entry},
    marker::PhantomData,
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::{Mutex, Notify, oneshot};
use tokio::task::JoinError;
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, Level, debug, error, event, info, trace, warn};
use utils::hash_to_curve::HashToCurve;
use utils::serialize::fq::FqSerialize;
use utils::serialize::point::PointSerializeCompressed;

const TOPIC: &str = "abacrain20";

/// Inputs required to start actively executing the ABA protocol
pub struct AbaInput<CK> {
    pub v: Estimate,
    pub coin_keys_receiver: oneshot::Receiver<CK>,
}

/// Keys required to obtain a common coin during the Crain20 ABA
#[derive(Clone, Debug)]
pub struct CoinKeys<CG: CurveGroup> {
    pub sk: CG::ScalarField,
    pub vks: Vec<CG>,
    pub combined_vk: CG,
}

/// Errors output by the Crain20 ABA
#[derive(Error, Debug)]
#[error("aba error")]
pub enum AbaError {
    #[error("received cancellation before obtaining estimate")]
    #[non_exhaustive]
    CancelledEarly,

    #[error("failed to receive common coin keys")]
    #[non_exhaustive]
    CoinKeysRecv,

    #[error("oneshot receive error: `{1}`")]
    Receiver(#[source] RecvError, &'static str),

    #[error("coin toss error: `{1}`")]
    CoinToss(#[source] EcdhCoinTossError, &'static str),

    #[error("failed to initialize transport")]
    TransportInit,

    #[error("transport error")]
    Transport(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to join thread: `{1}`")]
    Join(#[source] JoinError, &'static str),

    #[error("failed to serialize bson: `{1}`")]
    BsonSer(#[source] bson::ser::Error, &'static str),
}

/// Structure used to specify various parameters required by the Crain20 ABA
pub struct AbaCrain20Config<CG, CK, H> {
    id: PartyId,
    n: usize,
    t: usize,
    g: CG,
    retry_strategy: RetryStrategy,
    _ck: PhantomData<fn() -> CK>,
    _h: PhantomData<fn() -> H>,
}

impl<CG, CK, H> AbaCrain20Config<CG, CK, H> {
    pub fn new(id: PartyId, n: usize, t: usize, g: CG, retry_strategy: RetryStrategy) -> Arc<Self> {
        Self {
            id,
            n,
            t,
            g,
            retry_strategy,
            _ck: PhantomData,
            _h: PhantomData,
        }
        .into()
    }

    fn get_topic(prefix: &str) -> String {
        if prefix.is_empty() {
            TOPIC.to_owned()
        } else {
            format!("{prefix}/{TOPIC}")
        }
    }
}

impl<'a, CG, CK, H> AbaConfig<'a, PartyId> for AbaCrain20Config<CG, CK, H>
where
    CG: CurveGroup + Copy + HashToCurve + PointSerializeCompressed,
    CG::ScalarField: FqSerialize,
    EcdhCoinTossEval<CG, H>: for<'de> Deserialize<'de>,
    CK: Send + Into<CoinKeys<CG>> + 'static,
    H: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
{
    type Input = AbaInput<CK>;
    type Error = Box<AbaError>;

    fn new_instance_with_prefix<T>(
        self: &Arc<Self>,
        sid: SessionId,
        topic_prefix: String,
        transport: T,
    ) -> Result<impl Aba<Input = Self::Input, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        // Create own transport instance for the specified topic
        let mut transport = transport
            .get_transport_for(Self::get_topic(&topic_prefix))
            .ok_or(AbaError::TransportInit)?;
        let sender = transport.sender().ok_or(AbaError::TransportInit)?;
        let receiver = transport.receiver_stream().ok_or(AbaError::TransportInit)?;
        Ok(AbaCrain20::<_, _, _, T::Transport> {
            config: self.clone(),
            sid,
            sender,
            receiver,
        })
    }
}

struct AbaState<CG: CurveGroup, H> {
    notify_bin_values: NotifyMap<(u8, AuxStage)>, // notify upon receiving 2t + 1 binary estimates (Algorithm 3, Line 7)
    bin_values: Mutex<HashMap<u8, BinValues>>,

    notify_count_aux: NotifyMap<(u8, AuxStage)>, // notify upon receiving n - t aux agreements (Algorithm 4, Line 4)
    aux_views: Mutex<PerPartyStorage<(u8, AuxStage), View>>, // store each views sent through aux messages

    notify_count_auxset: NotifyMap<u8>, // notify upon receiving at least n - t auxset agreements (Algorithm 5, Line 7)
    auxset_views: Mutex<PerPartyStorage<u8, View>>, // store each views sent through auxset messages

    notify_enough_coin_evals: NotifyMap<u8>,
    coin_evals: Mutex<PerPartyStorage<u8, EcdhCoinTossEval<CG, H>>>,
}

struct AbaCrain20<CG, CK, H, T>
where
    T: Transport,
{
    config: Arc<AbaCrain20Config<CG, CK, H>>,
    sid: SessionId,
    sender: T::Sender,
    receiver: T::ReceiveMessageStream,
}

struct AbaCrain20Instance<CG, CK, H, TS>
where
    TS: TransportSender,
{
    config: Arc<AbaCrain20Config<CG, CK, H>>,
    sid: SessionId,
    sender: TS,
}

impl<CG, CK, H, T> Aba for AbaCrain20<CG, CK, H, T>
where
    CG: CurveGroup + Copy + HashToCurve + PointSerializeCompressed,
    CG::ScalarField: FqSerialize,
    EcdhCoinTossEval<CG, H>: for<'de> Deserialize<'de>,
    CK: Send + Into<CoinKeys<CG>> + 'static,
    H: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
    T: Transport<Identity = PartyId> + 'static,
    T::Sender: Clone,
{
    type Input = AbaInput<CK>;
    type Error = Box<AbaError>;

    #[tracing::instrument(skip_all, fields(sid = ?self.sid))]
    async fn propose<RNG>(
        self,
        inputs: oneshot::Receiver<Self::Input>,
        output: oneshot::Sender<Estimate>,
        cancellation_token: CancellationToken,
        rng: &mut RNG,
    ) -> Result<(), Self::Error>
    where
        RNG: RngCore + CryptoRng + Send + Sync,
    {
        let Self {
            sender,
            receiver,
            sid,
            config,
        } = self;

        let aba20 = AbaCrain20Instance {
            config: config.clone(),
            sid,
            sender: sender.clone(),
        };

        debug!("Node `{}` started ABA with sid `{sid}`", config.id);

        // Initialize the ABA state machine
        let state = Arc::new(AbaState::<CG, H> {
            aux_views: Mutex::new(PerPartyStorage::new()),
            notify_count_aux: NotifyMap::new(),
            auxset_views: Mutex::new(PerPartyStorage::new()),
            notify_count_auxset: NotifyMap::new(),
            bin_values: Mutex::new(HashMap::new()),
            notify_bin_values: NotifyMap::new(),
            notify_enough_coin_evals: NotifyMap::new(),
            coin_evals: Mutex::new(PerPartyStorage::new()),
        });

        // Start the recv thread
        let recv_cancel_token = cancellation_token.child_token();
        let recv = tokio::task::spawn({
            let config = config.clone();
            let state = state.clone();
            let cancel = recv_cancel_token.clone();

            async move {
                Self::recv_thread(sid, config, receiver, sender, cancel, state).await;
            }
        });

        // Abort upon receiving a notification through the cancellation token
        let aba_input = tokio::select! {
            _ = cancellation_token.cancelled() => {
                error!("Node `{}` aborting ABA with sid `{sid}`, cause: cancellation token", config.id);
                Err(AbaError::CancelledEarly)
            }

            res = inputs => {
                // Wait for an estimate to be sent through the oneshot receiver channel
                match res {
                    Ok(aba_input) => Ok(aba_input),
                    Err(e) => {
                        error!("Node `{}` failed to receive estimate: {e:?}.", config.id);
                        cancellation_token.cancel();
                        Err(AbaError::Receiver(e, "failed to receive estimate"))
                    }
                }
            }
        };

        // We got inputs, execute until cancelled
        let res = match aba_input {
            Ok(aba_input) => {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        info!("Node `{}` aborting ABA with sid `{sid}`, cause: cancellation token", config.id);
                        Ok(())
                    }

                    res = aba20.propose_internal(state, aba_input, output, rng) => {
                        match res {
                            Ok(_) => unreachable!("propose internal never ends"),
                            Err(_) => res // forward error
                        }
                    }
                }
            }

            Err(e) => Err(e.into()), // rewrap error
        };

        // Wait for the recv thread to end
        match recv.await {
            Ok(_) => res, // return previous result
            Err(e) => {
                error!("Node `{}` failed to abort recv thread: {e:?}.", config.id);
                Err(AbaError::Join(e, "failed to join recv thread").into())
            }
        }
    }
}

impl<CG, CK, H, T> AbaCrain20<CG, CK, H, T>
where
    CG: CurveGroup + Copy + HashToCurve + PointSerializeCompressed,
    CG::ScalarField: FqSerialize,
    EcdhCoinTossEval<CG, H>: for<'de> Deserialize<'de>,
    CK: Send + Into<CoinKeys<CG>> + 'static,
    H: Default + DynDigest + BlockSizeUser + Clone + Send + Sync + 'static,
    T: Transport<Identity = PartyId>,
    T::Sender: Clone,
{
    /// Thread responsible for receiving all types of ABA messages and transmitting notifications.
    async fn recv_thread(
        sid: SessionId,
        config: Arc<AbaCrain20Config<CG, CK, H>>,
        receiver: T::ReceiveMessageStream,
        sender: T::Sender,
        cancel: CancellationToken,
        state: Arc<AbaState<CG, H>>,
    ) {
        let id = config.id;
        // Stop the thread upon receiving a signal from the cancellation token
        tokio::select! {
            _ = cancel.cancelled() => {
                info!("Node `{id}` in ABA with sid `{sid}` stopping recv_thread");
            }

            _ = Self::recv_loop(config, receiver, sender, state).instrument(tracing::info_span!("recv_loop", ?sid)) => {}
        }
    }

    /// Infinite loop listening for ABA messages and sending notifications.
    async fn recv_loop(
        config: Arc<AbaCrain20Config<CG, CK, H>>,
        mut receiver: T::ReceiveMessageStream,
        sender: T::Sender,
        state: Arc<AbaState<CG, H>>,
    ) {
        // Local variables
        let mut count_est = PerPartyStorage::new();
        let mut sent_estimate: HashSet<EstimateMessage> = HashSet::new();

        loop {
            let ReceivedMessage {
                sender: sender_id,
                content,
                ..
            } = match receiver.next().await {
                Some(Ok(m)) => m,
                Some(Err(e)) => {
                    warn!("Node `{}` failed to recv: {e:?}", config.id);
                    continue;
                }
                None => {
                    error!(
                        "Node `{}` failed to recv: no more items in stream",
                        config.id
                    );
                    return;
                }
            };

            let m: AbaMessage = match bson::from_slice(&content) {
                Ok(m) => m,
                Err(e) => {
                    error!(error = ?e, "Node `{}` failed to deserialize message", config.id);
                    continue;
                }
            };
            trace!(
                "Node `{}` received message {m:?} from {sender_id}",
                config.id,
            );

            match m {
                // 4: upon receiving BVAL(v) do
                AbaMessage::Estimate(est) => {
                    count_est.insert_once(est, sender_id, true);
                    let count = count_est.get_count(&est);

                    #[allow(clippy::int_plus_one)]
                    if count >= 2 * config.t + 1 {
                        // 7: if BVAL(V) received from 2t + 1 different nodes then
                        // 8: bin_values := bin_values \cup {v}
                        // add the estimate to the binary values
                        let mut r_bin_values = state.bin_values.lock().await;
                        let bin_values = &mut r_bin_values.entry(est.round).or_default()[est.stage];
                        if bin_values.contains(&est.estimate) {
                            drop(r_bin_values);
                        } else {
                            bin_values.push(est.estimate);
                            drop(r_bin_values);

                            // notify of update to bin_values
                            debug!(
                                "Node {} notifying bin values for round {}",
                                config.id, est.round
                            );
                            state.notify_bin_values.notify_one((est.round, est.stage));
                        };
                    } else if count >= config.t + 1 && !sent_estimate.contains(&est) {
                        // 5: if BVAL(v) received from t + 1 different nodes AND BVAL(v) was not sent, then
                        // 6: Send BVAL(v) to all nodes
                        let msg_est = AbaMessage::Estimate(est);
                        if let Err(e) =
                            broadcast_with_self(&msg_est, &config.retry_strategy, &sender).await
                        {
                            error!(
                                "Node `{}` failed to broadcast estimate message: {e:?}",
                                config.id
                            )
                        }
                        sent_estimate.insert(est);
                    }
                }

                AbaMessage::Auxiliary(aux) => {
                    let mut aux_views = state.aux_views.lock().await;
                    // Add the new estimate to the current view
                    aux_views
                        .entry((aux.round, aux.stage), sender_id)
                        .or_default()
                        .insert(aux.estimate);

                    // notify once we got at least n - t aux messages
                    if aux_views.get_count(&(aux.round, aux.stage)) >= config.n - config.t {
                        state.notify_count_aux.notify_one((aux.round, aux.stage));
                    }
                }

                AbaMessage::AuxiliarySet(aux_set) => {
                    // Insert auxset view, at most once per sender_id
                    let mut auxset_views = state.auxset_views.lock().await;
                    auxset_views.insert_once(aux_set.round, sender_id, aux_set.view);

                    // notify once we got at least n - t auxset messages
                    if auxset_views.get_count(&aux_set.round) >= config.n - config.t {
                        state.notify_count_auxset.notify_one(aux_set.round);
                    }
                }

                AbaMessage::CoinEval(msg_eval) => {
                    // Deserialize eval
                    let Ok(eval): Result<EcdhCoinTossEval<CG, _>, _> = msg_eval.borrow().try_into()
                    else {
                        warn!("Failed to deserialize CoinEvalMessage");
                        continue;
                    };

                    // Store one eval per party, per round. We cannot verify it here
                    // since the node may not be ready to check evaluations yet.
                    let mut coin_evals = state.coin_evals.lock().await;
                    coin_evals.insert_once(msg_eval.round, sender_id, eval);
                    let count = coin_evals.get_count(&msg_eval.round);
                    drop(coin_evals); // drop lock

                    // Notify if we have t + 1 evals
                    if count > config.t {
                        state.notify_enough_coin_evals.notify_one(msg_eval.round);
                    }
                }
            };
        }
    }
}

impl<CG, CK, H, TS> AbaCrain20Instance<CG, CK, H, TS>
where
    CG: CurveGroup + Copy + HashToCurve + PointSerializeCompressed,
    CG::ScalarField: FqSerialize,
    EcdhCoinTossEval<CG, H>: for<'de> Deserialize<'de>,
    CK: Send + Into<CoinKeys<CG>> + 'static,
    H: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
    TS: TransportSender<Identity = PartyId> + Clone,
{
    #[tracing::instrument(skip_all, fields(aba_input = ?aba_input.v))]
    async fn propose_internal<RNG>(
        self,
        state: Arc<AbaState<CG, H>>,
        aba_input: AbaInput<CK>,
        sender: oneshot::Sender<Estimate>,
        rng: &mut RNG,
    ) -> Result<(), Box<AbaError>>
    where
        RNG: RngCore + CryptoRng,
    {
        let AbaInput {
            v,
            coin_keys_receiver,
        } = aba_input;
        info!(
            "Node `{}` executing ABA with sid `{}` and estimate {v:?}",
            self.config.id, self.sid
        );
        let mut sender = Some(sender);
        let mut coin_keys_receiver = Some(coin_keys_receiver);
        let mut coin_keys = None;
        let mut est = v;
        let mut r = 0;

        loop {
            // 4: r \gets r + 1
            r += 1;
            info!(estimate = ?est, "Node `{}` started ABA round {r}", self.config.id);
            // 5: (view[r, 0], bin_values[r]) \gets SBV_Broadcast(est)
            let view_r_0 = self.sbv_broadcast(r, AuxStage::Stage1, est, &state).await;

            // 6: send AUXSET[r](view[r, 0]) to all
            let msg_auxset = AbaMessage::AuxiliarySet(AuxiliarySetMessage {
                round: r,
                view: view_r_0,
            });
            if let Err(e) =
                broadcast_with_self(&msg_auxset, &self.config.retry_strategy, &self.sender).await
            {
                error!(
                    "Node `{}` failed to broadcast auxset message: {e:?}",
                    self.config.id
                )
            }

            // 7: wait until \exists a set view[r, 1] such that
            //     - (1) view[r, 1] \subseteq bin_values; and
            //     - (2) contained in AUXSET(.) messages received from n - t nodes
            event!(
                Level::DEBUG,
                "Node `{}` at round `{r}` waiting for valid view",
                self.config.id
            );
            let view_r_1 = loop {
                // wake up each time after having received n - t auxset, or on bin_values update
                future_select_pin(
                    state.notify_count_auxset.notified(r),
                    state.notify_bin_values.notified((r, AuxStage::Stage1)),
                )
                .await;

                let auxset_views = state.auxset_views.lock().await;
                let bin_values = state.bin_values.lock().await; // warn: two locks
                let auxset_views = auxset_views.get(&r).to_owned().unwrap_or_default();
                let bin_values = &bin_values.get(&r).cloned().unwrap_or_default()[AuxStage::Stage1];
                if let Some(view) = self.construct_view(bin_values, &auxset_views) {
                    event!(
                        Level::DEBUG,
                        "Node `{}` at round `{r}` obtained valid view `{view:?}`",
                        self.config.id
                    );

                    break view;
                }
            };

            // 8: if view[r, 1] = {w} then est \gets w (i.e., if there is a single element w)
            // 9: else est \gets \bot
            est = view_r_1.into(); // Converts view into a single estimate, bot otherwise

            // 10: view[r, 2] \gets SBV_Broadcast(est)
            let view_r_2 = self.sbv_broadcast(r, AuxStage::Stage2, est, &state).await;

            // 11: if view[r, 2] = {v}, v \neq \bot then
            let v = Estimate::from(view_r_2.clone());
            if v != Estimate::Bot {
                // est \gets v
                est = v;
                info!(
                    "Node {} sid `{}` decided on estimate `{est:?}`",
                    self.config.id, self.sid
                );

                // Inform the parent thread through the channel once
                // decide(v)
                if let Some(sender) = sender.take()
                    && sender.send(est).is_err()
                {
                    error!(
                        "Node `{}` failed to send decided upon estimate through sender channel: receiver dropped",
                        self.config.id
                    );
                }
            } else {
                event!(
                    Level::DEBUG,
                    "Node `{}` at round `{r}` failed to obtain a valid estimate: generating common coin",
                    self.config.id
                );

                // Get a reference from coin_keys, or use the receiver if None
                let coin_keys = if let Some(ck) = coin_keys.as_ref() {
                    ck
                } else {
                    // Try to get keys, abort ABA if it returns an error
                    coin_keys = Some(
                        self.get_coin_keys(r, coin_keys_receiver.take().unwrap())
                            .await?
                            .into(),
                    );
                    coin_keys.as_ref().unwrap()
                };

                // 12: else Coin() = sign(pk, #ABA||#round)
                if let Err(e) = self.send_coin_eval(r, coin_keys, rng).await {
                    event!(
                        Level::ERROR,
                        "Node `{}` at round `{r}` failed to generate and send coin eval: {e:?}",
                        self.config.id
                    );
                }
            }

            // 13: if view[r, 2] = {v, \bot} then est \gets v
            if view_r_2 == View::bot_zero() {
                est = Estimate::Zero;
            } else if view_r_2 == View::bot_one() {
                est = Estimate::One;
            }

            // 14: if view[r, 2] = {\bot} then est \gets Coin()
            if view_r_2.is_bot() {
                let coin_keys = coin_keys
                    .as_ref()
                    .expect("coin_keys cannot be None at this point");

                // est \gets Coin()
                let coin = self.get_coin(r, &state, coin_keys).await?; // get coin or abort ABA with error
                event!(
                    Level::INFO,
                    "Node `{}` at round `{r}` successfully obtained a common coin, new estimate: {:?}",
                    self.config.id,
                    Estimate::from(coin)
                );

                est = coin.into();
            }
        }
    }

    /// Binary-value broadcast described in https://dl.acm.org/doi/10.1145/2785953, Figure 1
    /// Send the current party's estimate to all other nodes with an Estimate message.
    #[tracing::instrument(skip(self))]
    async fn bv_broadcast(&self, r: u8, stage: AuxStage, v: Estimate) {
        // 1: broadcast B_VAL(v) to all
        let msg_est = AbaMessage::Estimate(EstimateMessage {
            round: r,
            stage,
            estimate: v,
        });

        event!(
            Level::DEBUG,
            "Node `{}` at round `{r}` sending {:?} to all",
            self.config.id,
            msg_est
        );
        if let Err(e) =
            broadcast_with_self(&msg_est, &self.config.retry_strategy, &self.sender).await
        {
            error!(
                "Node `{}` failed to broadcast estimate message: {e:?}",
                self.config.id
            )
        }
    }

    /// Synchronized binary-value broadcast described in https://dl.acm.org/doi/10.1145/2785953, Figure 2
    /// Send the current party's estimate to all other nodes with an Estimate message.
    #[tracing::instrument(skip(self, state))]
    async fn sbv_broadcast(
        &self,
        r: u8,
        stage: AuxStage,
        v: Estimate,
        state: &Arc<AbaState<CG, H>>,
    ) -> View {
        // 1: BV_Broadcast(v)
        self.bv_broadcast(r, stage, v).await;

        event!(
            Level::DEBUG,
            "Node `{}` waiting for bin values",
            self.config.id
        );
        let bin_values = loop {
            // 2: wait until bin_values \neq \emptyset
            state.notify_bin_values.notified((r, stage)).await;

            let bin_values = state.bin_values.lock().await;
            let bin_values = &bin_values.get(&r).cloned().unwrap_or_default()[stage];
            if !bin_values.is_empty() {
                event!(
                    Level::DEBUG,
                    "Node `{}` obtained bin_values = `{bin_values:?}`",
                    self.config.id
                );
                break bin_values.clone();
            }
        };

        // 3: Send AUX(w) for w \in bin_values to all
        for w in bin_values.iter() {
            let msg_aux = AbaMessage::Auxiliary(AuxiliaryMessage {
                round: r,
                stage,
                estimate: *w,
            });
            event!(
                Level::DEBUG,
                "Node `{}` sending {:?} to all",
                self.config.id,
                msg_aux
            );

            if let Err(e) =
                broadcast_with_self(&msg_aux, &self.config.retry_strategy, &self.sender).await
            {
                error!(
                    "Node `{}` failed to broadcast aux message: {e:?}",
                    self.config.id
                )
            }
        }

        // 4: wait until \exists a set view s.t.
        //  (1) view \subseteq bin_values, and
        //  (2) contained in AUX(.) messages received from n - t nodes
        let view = loop {
            event!(
                Level::DEBUG,
                "Node `{}` waiting for count_aux notification",
                self.config.id
            );

            // wake up each time after having received n - t aux, or on bin_values update
            future_select_pin(
                state.notify_count_aux.notified((r, stage)),
                state.notify_bin_values.notified((r, stage)),
            )
            .await;

            let aux_views = state.aux_views.lock().await;
            let bin_values = state.bin_values.lock().await; // warn: two locks, could deadlock
            let aux_views = aux_views.get(&(r, stage)).to_owned().unwrap_or_default();
            let bin_values = &bin_values.get(&r).cloned().unwrap_or_default()[stage];
            let view = self.construct_view(bin_values, &aux_views);
            if let Some(view) = view {
                event!(
                    Level::DEBUG,
                    "Node {} obtained view = `{view:?}`",
                    self.config.id
                );
                break view;
            } else {
                event!(
                    Level::DEBUG,
                    "Node {} received notify_count_aux notification while having no binary estimates / not enough aux",
                    self.config.id
                );
            }
        };
        // 5: return view
        #[allow(clippy::let_and_return)] // for clarity
        view
    }

    /// Try to get the output from the coin keys receiver, return an error otherwise.
    async fn get_coin_keys(
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
    async fn send_coin_eval<RNG>(
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
    async fn get_coin(
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

    /// Try to build a view from the union of views sent by other nodes, filtered by local binary values
    /// obtained through the BV_broadcast algorithm, Figure 1 of <https://arxiv.org/pdf/2002.08765>.
    /// Implements filtering of line (05), Figure 3 of <https://arxiv.org/pdf/2002.08765>:
    /// \exists a view such that its values (i) belong to bin values and comes from views sent by
    /// (n − t) distinct processes.
    fn construct_view(&self, bin_values: &[Estimate], views: &[&View]) -> Option<View> {
        assert!(bin_values.len() <= 2);

        let bin_values = BTreeSet::from_iter(bin_values.iter().copied());

        // Form a view such that its values (i) belong to bin values and comes from views sent by
        // (n − t) distinct processes
        let mut count = 0;
        let mut view_union = View::default();
        for &view in views {
            if !view.is_subset(&bin_values) {
                // not a subset of bin_values, ignore
                continue;
            }

            view_union.extend(view.iter()); // equivalent to union
            count += 1;
            if count >= self.config.n - self.config.t {
                return Some(view_union);
            }
        }

        None
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

/// Helper struct used for per key, per party storage.
/// Used to quickly insert a value for a specific key and party, and , get/count all values belonging to a key, independently of the parties.
struct PerPartyStorage<K, V> {
    db: HashMap<K, BTreeMap<PartyId, V>>, // BTreeMap to preserve ordering
}

impl<K, V> PerPartyStorage<K, V>
where
    K: Eq + PartialEq + std::hash::Hash,
{
    fn new() -> Self {
        PerPartyStorage { db: HashMap::new() }
    }

    /// Get the entry to a key
    fn entry(&mut self, k: K, party: PartyId) -> Entry<'_, PartyId, V> {
        let storage = self.db.entry(k).or_default();
        storage.entry(party)
    }

    /// Only insert if the key is not already present
    fn insert_once(&mut self, k: K, party: PartyId, v: V) {
        let storage = self.db.entry(k).or_default();
        match storage.entry(party) {
            Entry::Occupied(_) => {} // nop
            Entry::Vacant(e) => {
                let _ = e.insert(v);
            }
        }
    }

    /// Returns the values stored for key k by all of the parties, sorted on the party id.
    fn get_all(&self, k: &K) -> Option<(Vec<PartyId>, Vec<&V>)> {
        let storage = self.db.get(k)?;
        Some(storage.iter().map(|(k, v)| (*k, v)).unzip())
    }

    /// Returns the values stored for key k
    fn get(&self, k: &K) -> Option<Vec<&V>> {
        let storage = self.db.get(k)?;
        Some(storage.values().collect())
    }

    /// Returns the number of values stored for key k amongst all parties.
    fn get_count(&self, k: &K) -> usize {
        let Some(storage) = self.db.get(k) else {
            return 0;
        };

        storage.len()
    }
}

/// Helper struct to store and maintain a map of notifications
/// todo: Ideally, we would clear the map once the round has passed
struct NotifyMap<K> {
    notifies: std::sync::Mutex<HashMap<K, Arc<Notify>>>,
}

impl<K> NotifyMap<K>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        Self {
            notifies: std::sync::Mutex::new(HashMap::new()),
        }
    }

    fn get_notify(&self, k: K) -> Arc<Notify> {
        let mut notifies = self
            .notifies
            .lock()
            .expect("a thread holding the notifies mutex panicked");

        notifies.entry(k).or_insert(Arc::new(Notify::new())).clone()
    }

    async fn notified(&self, k: K) {
        self.get_notify(k).notified().await
    }

    fn notify_one(&self, k: K) {
        self.get_notify(k).notify_one()
    }
}

#[derive(Default, Clone)]
struct BinValues([Vec<Estimate>; 2]);

impl Index<AuxStage> for BinValues {
    type Output = Vec<Estimate>;

    fn index(&self, index: AuxStage) -> &Self::Output {
        match index {
            AuxStage::Stage1 => &self.0[0],
            AuxStage::Stage2 => &self.0[1],
        }
    }
}

impl IndexMut<AuxStage> for BinValues {
    fn index_mut(&mut self, index: AuxStage) -> &mut Self::Output {
        match index {
            AuxStage::Stage1 => &mut self.0[0],
            AuxStage::Stage2 => &mut self.0[1],
        }
    }
}

impl Deref for View {
    type Target = BTreeSet<Estimate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for View {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl View {
    pub(crate) fn bot_zero() -> Self {
        Self(BTreeSet::from_iter([Estimate::Bot, Estimate::Zero]))
    }

    pub(crate) fn bot_one() -> Self {
        Self(BTreeSet::from_iter([Estimate::Bot, Estimate::One]))
    }

    pub(crate) fn is_bot(&self) -> bool {
        self.0.first().is_some_and(|est| est == &Estimate::Bot)
    }
}

impl<CG, CK, H, T> From<AbaCrain20<CG, CK, H, T>> for AbaCrain20Instance<CG, CK, H, T::Sender>
where
    T: Transport,
{
    fn from(aba20: AbaCrain20<CG, CK, H, T>) -> Self {
        Self {
            sid: aba20.sid,
            config: aba20.config,
            sender: aba20.sender,
        }
    }
}

async fn future_select_pin<Out>(a: impl Future<Output = Out>, b: impl Future<Output = Out>) -> Out {
    let a = pin!(a);
    let b = pin!(b);
    match futures::future::select(a, b).await {
        Either::Left((o, _)) | Either::Right((o, _)) => o,
    }
}

#[cfg(test)]
mod tests {
    use crate::aba::AbaConfig;
    use crate::aba::crain20::{AbaCrain20Config, AbaInput, CoinKeys};
    use crate::aba::{Aba, Estimate};
    use crate::helpers::PartyId;
    use crate::network::RetryStrategy;
    use ark_bn254::Bn254;
    use ark_ec::{PrimeGroup, pairing::Pairing};
    use dcipher_network::topic::dispatcher::TopicDispatcher;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use rand::rngs::OsRng;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use tokio::sync::oneshot;
    use tokio::task;
    use tokio::task::JoinSet;
    use tokio_util::sync::CancellationToken;

    type G = <Bn254 as Pairing>::G1;

    #[tokio::test]
    async fn test_aba_all_parties_est_one() {
        _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        let t = 2;
        let n = 3 * t + 1;
        let g = G::generator();

        let (dispatchers, mut tbts): (Vec<_>, VecDeque<_>) =
            MemoryNetwork::get_transports(PartyId::iter_all(n))
                .into_iter()
                .map(|t| {
                    let mut dispatcher = TopicDispatcher::new();
                    let tbt = dispatcher.start(t);
                    (dispatcher, tbt)
                })
                .collect();
        let mut abas: VecDeque<_> = PartyId::iter_all(n)
            .map(|i| AbaCrain20Config::<_, _, sha3::Sha3_256>::new(i, n, t, g, RetryStrategy::None))
            .collect();

        let mut tasks = JoinSet::new();
        for i in PartyId::iter_all(n) {
            tasks.spawn({
                let transport = tbts.pop_front().unwrap();
                let aba_config = abas.pop_front().unwrap();

                async move {
                    let (isender, ireceiver) = oneshot::channel();
                    let (osender, oreceiver) = oneshot::channel();
                    let cancel = CancellationToken::new();
                    let cancel_cloned = cancel.clone();

                    // Create input with One estimate
                    let (coin_keys_sender, coin_keys_receiver) = oneshot::channel::<CoinKeys<G>>();
                    drop(coin_keys_sender); // all nodes input the same estimate, coin must not be used
                    let est = AbaInput {
                        v: Estimate::One,
                        coin_keys_receiver,
                    };

                    // Spawn aba task
                    let aba_task = task::spawn(async move {
                        let aba = aba_config
                            .new_instance(0.into(), Arc::new(transport))
                            .expect("failed to create aba instance");
                        aba.propose(ireceiver, osender, cancel_cloned, &mut OsRng)
                            .await
                    });

                    // Send the estimate through the channel
                    isender
                        .send(est)
                        .unwrap_or_else(|_| panic!("failed to send estimate"));

                    // Wait for the estimate
                    let estimate = oreceiver.await.expect("aba thread dropped oneshot sender");
                    cancel.cancel();

                    // Wait for aba to complete
                    aba_task
                        .await
                        .expect("failed to join thread")
                        .expect("aba failed");
                    (i, estimate)
                }
            });
        }

        while let Some(res) = tasks.join_next().await {
            assert!(res.is_ok());
            let (_, est) = res.unwrap();
            assert_eq!(est, Estimate::One);
        }

        for d in dispatchers {
            d.stop().await;
        }
    }
}
