//! Module for reliable broadcast protocols.
pub(crate) mod multi_rbc;
pub mod r4;
mod reed_solomon;

use crate::helpers::PartyId;
use async_trait::async_trait;
use dcipher_network::topic::TopicBasedTransport;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Trait used to store the configuration required by a Reliable Broadcast protocol and create
/// new instances to broadcast and receive messages.
pub trait ReliableBroadcastConfig<'a, ID>: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    // We need an explicit lifetime for the output to have a different lifetime than self
    fn new_instance<T>(
        self: &Arc<Self>,
        transport: T,
    ) -> Result<impl ReliableBroadcast<Identity = ID, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = ID>,
    {
        self.new_instance_with_prefix("".to_owned(), transport)
    }

    // We need an explicit lifetime for the output to have a different lifetime than self
    fn new_instance_with_prefix<T>(
        self: &Arc<Self>,
        topic_prefix: String,
        transport: T,
    ) -> Result<impl ReliableBroadcast<Identity = ID, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = ID>;
}

/// Reliably broadcast and receive a message.
#[async_trait::async_trait]
pub trait ReliableBroadcast: Send {
    type Identity;
    type Error: std::error::Error + Send + Sync + 'static;

    /// Start the reliable broadcast by sending a proposal for message m.
    async fn start(self, m: &[u8], cancel: CancellationToken) -> Result<Vec<u8>, Self::Error>;

    /// Listen for a reliable broadcast proposal and interact with other nodes to output a message.
    async fn listen<P>(
        self,
        predicate: &P,
        expected_sender: Self::Identity,
        cancel: CancellationToken,
    ) -> Result<Vec<u8>, Self::Error>
    where
        P: RbcPredicate;
}

/// Predicate to determine the validity of the message sent during the reliable broadcast.
#[async_trait]
pub trait RbcPredicate: Send + Sync {
    async fn predicate(&self, sender: PartyId, m: &[u8]) -> bool;
}

/// This predicate can be used for basic RBC broadcast where any message is accepted.
#[derive(Clone)]
pub(crate) struct AlwaysTruePredicate;

#[async_trait]
impl RbcPredicate for AlwaysTruePredicate {
    async fn predicate(&self, _sender: PartyId, _m: &[u8]) -> bool {
        true
    }
}
