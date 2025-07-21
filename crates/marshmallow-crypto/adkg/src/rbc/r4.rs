/// Implementation of the four-round RBC protocol for long messages of https://eprint.iacr.org/2021/777.pdf, Algorithm 4
mod handlers;
pub(crate) mod messages;

use super::{AlwaysTruePredicate, RbcPredicate, ReliableBroadcast};
use crate::helpers::PartyId;
use crate::network::{AuthenticatedTransport, Recipient, RetryStrategy};
use handlers::{rbc_receive_echo, rbc_receive_proposal, rbc_receive_ready};
use messages::*;
use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap},
    hash::Hash,
};
use thiserror::Error;
use tracing::{debug, error, info, warn};

type SendMessage<M> = crate::network::SendMessage<M, PartyId>;
type ReceiveMessage<M> = crate::network::ReceivedMessage<M, PartyId>;

/// Implementation of the four-round RBC protocol for long messages of https://eprint.iacr.org/2021/777.pdf, Algorithm 4
pub struct Rbc4Rounds {
    n: usize,
    t: usize,
    id: usize,
    retry_strategy: RetryStrategy,
}

impl Rbc4Rounds {
    pub fn new(id: usize, n: usize, t: usize, retry_strategy: &RetryStrategy) -> Self {
        Rbc4Rounds {
            n,
            t,
            id,
            retry_strategy: *retry_strategy,
        }
    }
}

/// Empty error type since the DKG cannot crash currently.
#[derive(Error, Debug)]
#[error("rbc error")]
pub struct RbcError;

#[async_trait::async_trait]
impl ReliableBroadcast for Rbc4Rounds {
    type Message = Message;
    type Identity = PartyId;
    type Error = RbcError;

    async fn start<T>(&mut self, m: &[u8], transport: &mut T) -> Result<Vec<u8>, Self::Error>
    where
        T: AuthenticatedTransport<Message = Self::Message, Identity = Self::Identity>,
    {
        rbc_leader(m, self.id, self.n, self.t, &self.retry_strategy, transport).await
    }

    async fn listen<P, T>(
        &mut self,
        predicate: &P,
        transport: &mut T,
    ) -> Result<Vec<u8>, Self::Error>
    where
        P: RbcPredicate + Send + Sync,
        T: AuthenticatedTransport<Message = Self::Message, Identity = Self::Identity>,
    {
        rbc(
            self.id,
            self.n,
            self.t,
            &self.retry_strategy,
            transport,
            predicate,
        )
        .await
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
    id: usize,
    status: RbcStatus, // Current status of the RBC protocol

    count_echo_messages: PerPartyStorage<Echo, bool>, // count the number of echo messages
    ready_h_messages: PerPartyStorage<Vec<u8>, Vec<u8>>, // store the unique message sent by each party for each hash when ready, i.e., PerPartyStorage<h, m>
    h_message_crossed_threshold: HashMap<Vec<u8>, Vec<u8>>, // HashMap to store messages that were echoed at least t + 1 times per hash, i.e., HashMap<h, m>

    retry_strategy: &'a RetryStrategy,
    transport: &'a mut T,
}

/// Try to send a message to other nodes.
async fn send_helper<T>(
    m: &SendMessage<Message>,
    retry_strategy: &RetryStrategy,
    transport: &mut T,
) -> Result<(), RbcError>
where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    debug!("Attempting to send message `{m:?}`");
    match transport.send_retry(m, retry_strategy).await {
        Ok(_) => {
            debug!("Message `{m:?}` sent");
            Ok(())
        }
        Err(e) => {
            error!("Failed to send message to node(s): {e:?}");
            Err(RbcError)? // fix error handling, we need to wrap transport errors.
        }
    }
}

/// Beginning of the protocol executed by the leader
async fn rbc_leader<T>(
    m: &[u8],
    i: usize,
    n: usize,
    t: usize,
    retry_strategy: &RetryStrategy,
    transport: &mut T,
) -> Result<Vec<u8>, RbcError>
where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    info!("Leader `{i}` starting RBC with parameters (n = `{n}`, t = `{t}`)");
    debug!("Leader `{i}` starting RBC for message `{m:?}`");

    // only broadcaster node
    // input M
    // send \langle PROPOSE, M \rangle to all
    let msg = SendMessage {
        recipient: Recipient::All,
        content: Message::Propose(Propose { m: m.to_vec() }),
    };

    if let Err(e) = send_helper(&msg, retry_strategy, transport).await {
        error!("Leader `{i}` failed to send proposal... Aborting RBC.");
        Err(e)? // rewrap error
    }

    // Start the RBC protocol as a standard node
    rbc(i, n, t, retry_strategy, transport, &AlwaysTruePredicate).await
}

/// Protocol executed by the leader and the nodes.
async fn rbc<T>(
    i: usize,
    n: usize,
    t: usize,
    retry_strategy: &RetryStrategy,
    transport: &mut T,
    predicate: &impl RbcPredicate,
) -> Result<Vec<u8>, RbcError>
where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    info!("Node `{i}` starting RBC with parameters (n = `{n}`, t = `{t}`)");

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
        transport,
    };

    loop {
        let ReceiveMessage {
            content: msg,
            sender,
            ..
        } = match state_machine.transport.recv().await {
            Ok(m) => m,
            Err(e) => {
                warn!(
                    "Node `{}` failed to receive message with error {e:?}",
                    state_machine.id
                );
                continue;
            }
        };

        // Process the message
        match msg {
            Message::Propose(p) => {
                rbc_receive_proposal(sender, p, predicate, &mut state_machine).await
            }

            Message::Echo(e) => rbc_receive_echo(sender, e, &mut state_machine).await,

            Message::Ready(r) => rbc_receive_ready(sender, r, &mut state_machine).await,
        };

        // RBC completed, output message
        if let RbcStatus::Complete(m) = state_machine.status {
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
    use crate::{
        network::{mock_network::MockNetwork, RetryStrategy},
        rbc::{
            r4::{Message, Rbc4Rounds},
            AlwaysTruePredicate, ReliableBroadcast,
        },
    };
    use std::collections::VecDeque;
    use tokio::task::JoinSet;

    /// Execute the RBC in parallel
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_rbc_all_parties() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let m = b"Hello World!";
        let t = 3;
        let n = 3 * t + 1;
        /// Pkcs7 padding
        let mut transports = MockNetwork::<Message>::get_instances(n);
        let mut rbcs: VecDeque<Rbc4Rounds> = (1..=n)
            .map(|i| Rbc4Rounds::new(i, n, t, &RetryStrategy::None))
            .collect();

        let mut tasks = JoinSet::new();
        tasks.spawn({
            let mut transport = transports.pop_front().unwrap();
            let mut broadcast = rbcs.pop_front().unwrap();
            async move { broadcast.start(m, &mut transport).await }
        });

        for _ in 2..=n {
            tasks.spawn({
                let mut transport = transports.pop_front().unwrap();
                let mut broadcast = rbcs.pop_front().unwrap();
                async move { broadcast.listen(&AlwaysTruePredicate, &mut transport).await }
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
