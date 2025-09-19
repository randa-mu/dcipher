/// Implementation of the four-round RBC protocol for long messages of https://eprint.iacr.org/2021/777.pdf, Algorithm 4
mod handlers;
pub(crate) mod messages;

use super::{AlwaysTruePredicate, RbcPredicate, ReliableBroadcast, ReliableBroadcastConfig};
use crate::helpers::PartyId;
use crate::network::{RetryStrategy, broadcast_with_self};
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::{ReceivedMessage, Transport};
use futures::StreamExt;
use handlers::{rbc_receive_echo, rbc_receive_proposal, rbc_receive_ready};
use messages::*;
use std::sync::Arc;
use std::{
    collections::{BTreeMap, HashMap, btree_map::Entry},
    hash::Hash,
};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

const TOPIC: &str = "rbcr4";

/// Configuration for the four-round RBC protocol for long messages of https://eprint.iacr.org/2021/777.pdf, Algorithm 4
#[derive(Clone, Debug)]
pub struct Rbc4RoundsConfig {
    n: usize,
    t: usize,
    id: PartyId,
    retry_strategy: RetryStrategy,
}

/// Implementation of the four-round RBC protocol for long messages of https://eprint.iacr.org/2021/777.pdf, Algorithm 4
#[derive(Debug)]
struct Rbc4Rounds<T>
where
    T: Transport,
{
    config: Arc<Rbc4RoundsConfig>,
    sender: T::Sender,
    receiver: T::ReceiveMessageStream,
}

/// Empty error type since the DKG cannot crash currently.
#[derive(Error, Debug)]
#[error("rbc error")]
pub enum RbcError {
    #[error("RBC cancelled early")]
    CancelledEarly,

    #[error("failed to send & serialize message during RBC")]
    SendSerialize(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to initialize transport")]
    TransportInit,
}

impl Rbc4RoundsConfig {
    pub fn new(id: PartyId, n: usize, t: usize, retry_strategy: &RetryStrategy) -> Arc<Self> {
        Arc::new(Self {
            n,
            t,
            id,
            retry_strategy: *retry_strategy,
        })
    }

    fn get_topic(prefix: &str) -> String {
        if prefix.is_empty() {
            TOPIC.to_owned()
        } else {
            format!("{prefix}/{TOPIC}")
        }
    }
}

impl<'a> ReliableBroadcastConfig<'a, PartyId> for Rbc4RoundsConfig {
    type Error = RbcError;

    fn new_instance_with_prefix<T>(
        self: &Arc<Self>,
        topic_prefix: String,
        transport: T,
    ) -> Result<impl ReliableBroadcast<Identity = PartyId, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        // Create own transport instance for the specified topic
        let mut transport = transport
            .get_transport_for(Self::get_topic(&topic_prefix))
            .ok_or(Self::Error::TransportInit)?;
        let receiver = transport
            .receiver_stream()
            .ok_or(Self::Error::TransportInit)?;
        let sender = transport.sender().ok_or(Self::Error::TransportInit)?;
        Ok(Rbc4Rounds::<T::Transport> {
            config: self.clone(),
            sender,
            receiver,
        })
    }
}

#[async_trait::async_trait]
impl<T> ReliableBroadcast for Rbc4Rounds<T>
where
    T: Transport<Identity = PartyId>,
{
    type Identity = PartyId;
    type Error = RbcError;

    async fn start(self, m: &[u8], cancel: CancellationToken) -> Result<Vec<u8>, Self::Error> {
        tokio::select! {
            _ = cancel.cancelled_owned() => {
                info!("Leader `{}` aborting RBC due to cancellation token", self.config.id);
                Err(RbcError::CancelledEarly)?
            }

            res = rbc_leader::<T>(
                m,
                self.config.id,
                self.config.n,
                self.config.t,
                &self.config.retry_strategy,
                self.sender,
                self.receiver,
            ) => {
                res
            }
        }
    }

    async fn listen<P>(
        self,
        predicate: &P,
        expected_sender: Self::Identity,
        cancel: CancellationToken,
    ) -> Result<Vec<u8>, Self::Error>
    where
        P: RbcPredicate,
    {
        tokio::select! {
            _ = cancel.cancelled_owned() => {
                info!("Node `{}` aborting RBC due to cancellation token", self.config.id);
                Err(RbcError::CancelledEarly)?
            }

            res = rbc::<T>(
                self.config.id,
                self.config.n,
                self.config.t,
                &self.config.retry_strategy,
                self.sender,
                self.receiver,
                predicate,
                expected_sender,
            ) => {
                res
            }
        }
    }
}

/// The following transitions are made:
///     1) WaitingForProposal => WaitingForEchos. The proposal has been received, we now need other parties to echo. The party has sent their echo messages.
///     2a) WaitingForEchos => WaitingForReadys. The party has sent their ready message.
///     2b) WaitingForProposal => WaitingForReadys. The proposal has been received through other parties by receiving either (1) 2t + 1 echos, or (2) t + 1 readys and t + 1 echos.
///     3) WaitingForReadys => Complete. A valid broadcast message has been received after decoding 2t + 1 ready codewords with Reed-Solomon.
#[derive(Clone, Debug, PartialEq, Eq)]
enum RbcStatus {
    WaitingForProposal, // Proposal has not been received
    WaitingForEchos,    // Not enough echos have been received to become ready
    ReadySent(Vec<u8>), // A ready was sent for a specific hash
    Complete(Vec<u8>),  // A message was recovered, Rbc about to exit
}

/// State machine used by the RBC protocol
struct StateMachine<'a, T> {
    n: usize,
    t: usize,
    id: PartyId,
    status: RbcStatus, // Current status of the RBC protocol

    count_echo_messages: PerPartyStorage<Echo, bool>, // count the number of echo messages
    ready_h_messages: PerPartyStorage<Vec<u8>, Vec<u8>>, // store the unique message sent by each party for each hash when ready, i.e., PerPartyStorage<h, m>
    h_message_crossed_threshold: HashMap<Vec<u8>, Vec<u8>>, // HashMap to store messages that were echoed at least t + 1 times per hash, i.e., HashMap<h, m>

    retry_strategy: &'a RetryStrategy,
    sender: &'a T,
}

/// Beginning of the protocol executed by the leader
async fn rbc_leader<T>(
    m: &[u8],
    i: PartyId,
    n: usize,
    t: usize,
    retry_strategy: &RetryStrategy,
    sender: T::Sender,
    receiver: T::ReceiveMessageStream,
) -> Result<Vec<u8>, RbcError>
where
    T: Transport<Identity = PartyId>,
{
    info!("Leader `{i}` starting RBC with parameters (n = `{n}`, t = `{t}`)");
    debug!("Leader `{i}` starting RBC for message `{m:?}`");

    // only broadcaster node
    // input M
    // send \langle PROPOSE, M \rangle to all
    let msg = Message::Propose(Propose { m: m.to_vec() });
    if let Err(e) = broadcast_with_self(&msg, retry_strategy, &sender).await {
        error!("Leader `{i}` failed to send proposal... Aborting RBC.");
        Err(RbcError::SendSerialize(e.into()))? // rewrap error
    }

    // Start the RBC protocol as a standard node
    rbc::<T>(
        i,
        n,
        t,
        retry_strategy,
        sender,
        receiver,
        &AlwaysTruePredicate,
        i,
    )
    .await
}

/// Protocol executed by the leader and the nodes.
#[allow(clippy::too_many_arguments)]
async fn rbc<T>(
    i: PartyId,
    n: usize,
    t: usize,
    retry_strategy: &RetryStrategy,
    sender: T::Sender,
    mut receiver: T::ReceiveMessageStream,
    predicate: &impl RbcPredicate,
    expected_sender: PartyId,
) -> Result<Vec<u8>, RbcError>
where
    T: Transport<Identity = PartyId>,
{
    info!("Node `{i}` listening for RBC with parameters (n = `{n}`, t = `{t}`)");

    // Only the state machine should be mutable
    let mut state_machine = StateMachine {
        status: RbcStatus::WaitingForProposal,
        count_echo_messages: PerPartyStorage::new(),
        ready_h_messages: PerPartyStorage::new(),
        h_message_crossed_threshold: HashMap::new(),
        n,
        t,
        id: i,
        retry_strategy,
        sender: &sender,
    };

    loop {
        let ReceivedMessage {
            sender, content, ..
        } = match receiver.next().await {
            Some(Ok(m)) => m,
            Some(Err(e)) => {
                warn!("Node `{}` failed to recv: {e:?}", state_machine.id);
                continue;
            }
            None => {
                error!(
                    "Node `{}` failed to recv: no more items in stream",
                    state_machine.id
                );
                return Err(RbcError::CancelledEarly);
            }
        };
        let msg: Message = match bson::from_slice(&content) {
            Ok(m) => m,
            Err(e) => {
                error!(error = ?e, "Node `{}` failed to deserialize message", state_machine.id);
                continue;
            }
        };

        // Process the message
        match msg {
            Message::Propose(p) => {
                rbc_receive_proposal(sender, p, predicate, &expected_sender, &mut state_machine)
                    .await
            }

            Message::Echo(e) => rbc_receive_echo(sender, e, &mut state_machine).await,

            Message::Ready(r) => rbc_receive_ready(sender, r, &mut state_machine).await,
        };

        // RBC completed, output message
        if let RbcStatus::Complete(m) = state_machine.status {
            info!("Node `{i}` completed RBC");

            return Ok(m);
        }
    }
}

/// Helper struct used for per key, per party storage.
/// Used to quickly insert a value for a specific key and party, and , get/count all values belonging to a key, independently of the parties.
struct PerPartyStorage<K, V> {
    db: HashMap<K, BTreeMap<PartyId, V>>, // BTreeMap to preserve ordering
}

impl<K, V> PerPartyStorage<K, V>
where
    K: Eq + PartialEq + Hash,
{
    fn new() -> Self {
        PerPartyStorage { db: HashMap::new() }
    }

    /// Only insert if the key is not already present
    fn insert_once(&mut self, k: K, party: PartyId, v: V) {
        let storage = self.db.entry(k).or_default();
        match storage.entry(party) {
            Entry::Occupied(_) => {} // nop
            Entry::Vacant(e) => _ = e.insert(v),
        }
    }

    /// Returns the values stored for key k by all of the parties, sorted on the party id.
    fn get_all(&self, k: &K) -> Option<(Vec<PartyId>, Vec<&V>)> {
        let storage = self.db.get(k)?;
        Some(storage.iter().map(|(k, v)| (*k, v)).unzip())
    }

    /// Returns the number of values stored for key k amongst all parties.
    fn get_count(&self, k: &K) -> usize {
        let Some(storage) = self.db.get(k) else {
            return 0;
        };

        storage.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::PartyId;
    use crate::rbc::ReliableBroadcastConfig;
    use crate::rbc::r4::Rbc4RoundsConfig;
    use crate::{
        network::RetryStrategy,
        rbc::{AlwaysTruePredicate, ReliableBroadcast},
    };
    use dcipher_network::topic::dispatcher::TopicDispatcher;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use tokio::task::JoinSet;
    use tokio_util::sync::CancellationToken;

    /// Execute the RBC in parallel
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_rbc_all_parties() {
        _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .try_init();

        let topic = "test_rbc_all_parties";
        let m = b"Hello World!";
        let t = 2;
        let n = 3 * t + 1;

        let (_dispatchers, mut transports): (Vec<_>, VecDeque<_>) =
            MemoryNetwork::get_transports(PartyId::iter_all(n))
                .into_iter()
                .map(|t| {
                    let mut dispatcher = TopicDispatcher::new();
                    let tbt = dispatcher.start(t);
                    (dispatcher, tbt)
                })
                .collect();
        let mut rbcs: VecDeque<_> = PartyId::iter_all(n)
            .map(|i| Rbc4RoundsConfig::new(i, n, t, &RetryStrategy::None))
            .collect();

        let mut tasks = JoinSet::new();
        tasks.spawn({
            let transport = transports.pop_front().unwrap();
            let rbc_config = rbcs.pop_front().unwrap();
            let broadcast = rbc_config
                .new_instance_with_prefix(topic.to_owned(), Arc::new(transport))
                .expect("failed to create rbc instance");

            async move { broadcast.start(m, CancellationToken::new()).await }
        });

        for _ in 2..=n {
            tasks.spawn({
                let transport = transports.pop_front().unwrap();
                let broadcast = rbcs
                    .pop_front()
                    .unwrap()
                    .new_instance_with_prefix(topic.to_owned(), Arc::new(transport))
                    .expect("failed to create rbc instance");

                async move {
                    broadcast
                        .listen(&AlwaysTruePredicate, PartyId(1), CancellationToken::new())
                        .await
                }
            });
        }

        while let Some(res) = tasks.join_next().await {
            assert!(res.is_ok());

            let v = res.unwrap();
            assert!(v.is_ok());
            assert_eq!(v.unwrap(), m);
        }
    }
}
