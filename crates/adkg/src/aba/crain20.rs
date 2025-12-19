//! Implementation of the Tyler Crain's Asynchronous Byzantine Agreement described in https://arxiv.org/pdf/2002.08765.
//! We specifically implement the Good-Case-Coin-Free variant described in https://eprint.iacr.org/2021/1591.pdf, Appendix B.

mod broadcast;
mod coin;
mod ecdh_coin_toss;
pub mod messages;
mod recv_handler;

pub use crate::aba::crain20::coin::CoinToss;
pub use ecdh_coin_toss::{EcdhCoinToss, EcdhCoinTossParams};

use crate::aba::{Aba, AbaConfig, Estimate};
use crate::helpers::{PartyId, SessionId};
use crate::network::{RetryStrategy, broadcast_with_self};
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::{Transport, TransportSender};
use digest::crypto_common::rand_core::CryptoRng;
use futures::future::Either;
use messages::AuxiliarySetMessage;
use messages::{AbaMessage, AuxStage, View};
use rand::RngCore;
use std::collections::BTreeSet;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::pin::pin;
use std::{
    collections::{BTreeMap, HashMap, btree_map::Entry},
    marker::PhantomData,
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::{Mutex, Notify, oneshot};
use tokio::task::JoinError;
use tokio_util::sync::CancellationToken;
use tracing::{Level, debug, error, event, info};

const TOPIC: &str = "abacrain20";

/// Inputs required to start actively executing the ABA protocol
pub struct AbaInput<CK> {
    pub v: Estimate,
    pub coin_keys_receiver: oneshot::Receiver<CK>,
}

/// Keys required to obtain a common coin during the Crain20 ABA
#[derive(Clone, Debug)]
pub struct CoinKeys<CT: CoinToss> {
    pub sk: CT::SecretKey,
    pub params: CT::PublicParams,
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
    CoinToss(
        #[source] Box<dyn std::error::Error + Send + Sync + 'static>,
        &'static str,
    ),

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
pub struct AbaCrain20Config<CT, CK> {
    id: PartyId,
    n: usize,
    t: usize,
    retry_strategy: RetryStrategy,
    _ct: PhantomData<fn() -> CT>,
    _ck: PhantomData<fn() -> CK>,
}

impl<CT, CK> AbaCrain20Config<CT, CK> {
    pub fn new(id: PartyId, n: usize, t: usize, retry_strategy: RetryStrategy) -> Arc<Self> {
        Self {
            id,
            n,
            t,
            retry_strategy,
            _ct: PhantomData,
            _ck: PhantomData,
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

impl<'a, CT, CK> AbaConfig<'a, PartyId> for AbaCrain20Config<CT, CK>
where
    CT: CoinToss,
    CK: Send + Into<CoinKeys<CT>> + 'static,
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
        Ok(AbaCrain20::<_, _, T::Transport> {
            config: self.clone(),
            sid,
            sender,
            receiver,
        })
    }
}

struct AbaState<CT: CoinToss> {
    notify_bin_values: NotifyMap<(u8, AuxStage)>, // notify upon receiving 2t + 1 binary estimates (Algorithm 3, Line 7)
    bin_values: Mutex<HashMap<u8, BinValues>>,

    notify_count_aux: NotifyMap<(u8, AuxStage)>, // notify upon receiving n - t aux agreements (Algorithm 4, Line 4)
    aux_views: Mutex<PerPartyStorage<(u8, AuxStage), View>>, // store each views sent through aux messages

    notify_count_auxset: NotifyMap<u8>, // notify upon receiving at least n - t auxset agreements (Algorithm 5, Line 7)
    auxset_views: Mutex<PerPartyStorage<u8, View>>, // store each views sent through auxset messages

    notify_enough_coin_evals: NotifyMap<u8>,
    coin_evals: Mutex<PerPartyStorage<u8, CT::Eval>>,
}

struct AbaCrain20<CT, CK, T>
where
    T: Transport,
{
    config: Arc<AbaCrain20Config<CT, CK>>,
    sid: SessionId,
    sender: T::Sender,
    receiver: T::ReceiveMessageStream,
}

struct AbaCrain20Instance<CT, CK, TS> {
    config: Arc<AbaCrain20Config<CT, CK>>,
    sid: SessionId,
    sender: TS,
}

impl<CT, CK, T> Aba for AbaCrain20<CT, CK, T>
where
    CT: CoinToss,
    CK: Send + Into<CoinKeys<CT>> + 'static,
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
        let state = Arc::new(AbaState::<CT> {
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

impl<CT, CK, TS> AbaCrain20Instance<CT, CK, TS>
where
    CT: CoinToss,
    CK: Send + Into<CoinKeys<CT>> + 'static,
    TS: TransportSender<Identity = PartyId> + Clone,
{
    #[tracing::instrument(skip_all, fields(aba_input = ?aba_input.v))]
    async fn propose_internal<RNG>(
        self,
        state: Arc<AbaState<CT>>,
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
}

impl<CG, CK, TS> AbaCrain20Instance<CG, CK, TS> {
    /// Try to build a view from the union of views sent by other nodes, filtered by local binary values
    /// obtained through the BV_broadcast algorithm, Figure 1 of <https://arxiv.org/pdf/2002.08765>.
    /// Implements filtering of line (05), Figure 3 of <https://arxiv.org/pdf/2002.08765>:
    /// \exists a view such that its values (i) belong to bin values and come from views sent by
    /// (n − t) distinct processes. Returns `None` if coming from less than that.
    fn construct_view(&self, bin_values: &[Estimate], views: &[&View]) -> Option<View> {
        assert!(bin_values.len() <= 2);

        let bin_values = BTreeSet::from_iter(bin_values.iter().copied());

        // Form a view such that its values (i) belong to bin values and come from views sent by
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

impl<CT, CK, T> From<AbaCrain20<CT, CK, T>> for AbaCrain20Instance<CT, CK, T::Sender>
where
    T: Transport,
{
    fn from(aba20: AbaCrain20<CT, CK, T>) -> Self {
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
    use crate::aba::crain20::{AbaCrain20, AbaCrain20Config, AbaInput, CoinKeys};
    use crate::aba::{Aba, Estimate};
    use crate::helpers::{PartyId, SessionId};
    use crate::network::RetryStrategy;
    use ark_bn254::{Bn254, Fr};
    use ark_ec::{PrimeGroup, pairing::Pairing};
    use ark_poly::univariate::DensePolynomial;
    use ark_poly::{DenseUVPolynomial, Polynomial};
    use ark_std::UniformRand;
    use dcipher_network::Transport;
    use dcipher_network::transports::in_memory::{BusMemoryTransport, MemoryNetwork};
    use itertools::Itertools;
    use rand::rngs::OsRng;
    use std::collections::VecDeque;
    use tokio::sync::oneshot;
    use tokio::task;
    use tokio::task::JoinSet;
    use tokio_util::sync::CancellationToken;

    type G = <Bn254 as Pairing>::G1;

    fn gen_keys(n: u16, t: u16, g: G) -> (Vec<Fr>, G, Vec<G>) {
        // Build polynomial from coefficients
        let poly_coeffs = (0..t)
            .map(|_| <G as PrimeGroup>::ScalarField::rand(&mut OsRng))
            .collect::<Vec<_>>();
        let p = DensePolynomial::from_coefficients_slice(&poly_coeffs);

        let sk = p.evaluate(&0.into());
        let pk = g * sk;

        let sks = (1..=n).map(|i| p.evaluate(&i.into())).collect::<Vec<_>>();
        let pks = sks.iter().map(|ski| g * ski).collect::<Vec<_>>();

        (sks, pk, pks)
    }

    #[tokio::test]
    async fn test_aba_agreement() {
        let t = 2;
        let n = 3 * t + 1;
        let g = G::generator();
        let sid = SessionId::const_from(0);
        let est = Estimate::One;

        let (sks, pk, pks) = gen_keys(n as u16, t as u16, g);
        let estimates: Vec<_> = vec![est; n];

        let final_est = run(n, t, sks, pks, pk, g, estimates, sid).await;
        assert_eq!(est, final_est);
    }

    #[tokio::test]
    async fn test_aba_disagreement() {
        let t = 2;
        let n = 3 * t + 1;
        let g = G::generator();
        let sid = SessionId::const_from(0);

        let (sks, pk, pks) = gen_keys(n as u16, t as u16, g);
        let estimates: Vec<_> = PartyId::iter_all(n)
            .map(|i| {
                //
                // let est = if thread_rng().gen_bool(0.5) {
                //     Estimate::One
                // } else {
                //     Estimate::Zero
                // };
                // 50-50 split or so
                if i.as_usize() <= n / 2 {
                    Estimate::One
                } else {
                    Estimate::Zero
                }
            })
            .collect();

        run(n, t, sks, pks, pk, g, estimates, sid).await;
    }

    #[allow(clippy::too_many_arguments)]
    async fn run(
        n: usize,
        t: usize,
        sks: Vec<Fr>,
        pks: Vec<G>,
        pk: G,
        g: G,
        estimates: Vec<Estimate>,
        sid: SessionId,
    ) -> Estimate {
        let mut coin_keys: VecDeque<_> = sks
            .into_iter()
            .map(|sk| CoinKeys {
                sk,
                vks: pks.clone(),
                combined_vk: pk,
            })
            .collect();

        let mut transports: VecDeque<_> = MemoryNetwork::get_transports(PartyId::iter_all(n));
        let mut abas: VecDeque<_> = PartyId::iter_all(n)
            .map(|i| AbaCrain20Config::<_, _, sha3::Sha3_256>::new(i, n, t, g, RetryStrategy::None))
            .collect();

        let mut tasks = JoinSet::new();
        for i in PartyId::iter_all(n) {
            tasks.spawn({
                let mut transport = transports.pop_front().unwrap();
                let aba_config = abas.pop_front().unwrap();
                let coin_keys = coin_keys.pop_front().unwrap();
                let v = estimates[i];

                async move {
                    let (isender, ireceiver) = oneshot::channel();
                    let (osender, oreceiver) = oneshot::channel();
                    let cancel = CancellationToken::new();
                    let cancel_cloned = cancel.clone();

                    // Create input with One estimate
                    let (coin_keys_sender, coin_keys_receiver) = oneshot::channel::<CoinKeys<G>>();
                    coin_keys_sender.send(coin_keys).unwrap();

                    let est = AbaInput {
                        v,
                        coin_keys_receiver,
                    };

                    // Spawn aba task
                    let aba_task = task::spawn(async move {
                        let aba = AbaCrain20::<_, _, _, BusMemoryTransport<_>> {
                            config: aba_config,
                            receiver: transport.receiver_stream().unwrap(),
                            sender: transport.sender().unwrap(),
                            sid,
                        };
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

        let mut ests = vec![];
        while let Some(res) = tasks.join_next().await {
            assert!(res.is_ok());
            let (_, est) = res.unwrap();
            assert!([Estimate::Zero, Estimate::One].contains(&est));
            ests.push(est);
        }
        assert!(ests.iter().all_equal());

        *ests.first().unwrap()
    }
}
