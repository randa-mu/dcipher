/// Module for reliable broadcast protocols.
pub(crate) mod r4;
mod reed_solomon;

use crate::helpers::PartyId;
use crate::network::AuthenticatedTransport;

/// Reliably broadcast and receive a message.
#[async_trait::async_trait]
pub trait ReliableBroadcast {
    type Message;
    type Identity;
    type Error: std::error::Error;

    /// Start the reliable broadcast by sending a proposal for message m.
    async fn start<T>(&mut self, m: &[u8], transport: &mut T) -> Result<Vec<u8>, Self::Error>
    where
        T: AuthenticatedTransport<Message = Self::Message, Identity = Self::Identity>;

    /// Listen for a reliable broadcast proposal and interact with other nodes to output a message.
    async fn listen<P, T>(
        &mut self,
        predicate: &P,
        transport: &mut T,
    ) -> Result<Vec<u8>, Self::Error>
    where
        P: RbcPredicate + Send + Sync,
        T: AuthenticatedTransport<Message = Self::Message, Identity = Self::Identity>;
}

/// Predicate to determine the validity of the message sent during the reliable broadcast.
pub trait RbcPredicate {
    fn predicate(&self, sender: PartyId, m: &[u8]) -> bool;
}

/// This predicate can be used for basic RBC broadcast where any message is accepted.
pub(crate) struct AlwaysTruePredicate;

impl RbcPredicate for AlwaysTruePredicate {
    fn predicate(&self, _sender: PartyId, _m: &[u8]) -> bool {
        true
    }
}
