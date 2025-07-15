use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[cfg(feature = "libp2p")]
pub mod libp2p;

pub trait PartyIdentifier: std::fmt::Display + Clone + Debug + Eq + PartialEq {}

impl<T> PartyIdentifier for T where T: std::fmt::Display + Clone + Debug + Eq + PartialEq {}

/// Recipient of a message.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Recipient<I: PartyIdentifier> {
    All,
    Single(I),
}

/// Message received from a sender.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceivedMessage<I: PartyIdentifier, M = Vec<u8>> {
    pub sender: I,
    pub recipient: Recipient<I>,
    pub content: M,
}

impl<I, M> ReceivedMessage<I, M>
where
    I: PartyIdentifier,
{
    pub fn new_broadcast(sender: I, content: M) -> Self {
        Self {
            content,
            sender,
            recipient: Recipient::All,
        }
    }

    pub fn new(sender: I, content: M, recipient: Recipient<I>) -> Self {
        Self {
            content,
            sender,
            recipient,
        }
    }
}

#[derive(Clone, Debug)]
struct SendMessage<I: PartyIdentifier> {
    to: Recipient<I>,
    msg: Vec<u8>,
}

/// A transport trait that can be used to obtain senders and incoming message streams.
pub trait Transport {
    type Identity: PartyIdentifier + Send + Sync + 'static;

    type ReceiveMessageStream: futures_util::Stream<Item = ReceivedMessage<Self::Identity>>
        + Send
        + Unpin
        + 'static;

    type Sender: TransportSender<Identity = Self::Identity> + Send + Sync + 'static;

    /// Obtain a [`TransportSender`] that can be used to send messages.
    fn sender(&mut self) -> Option<Self::Sender>;

    /// Obtain a [`Self::ReceiveMessageStream`] used to receive messages.
    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream>;
}

/// A sender for (broadcast) messages.
pub trait TransportSender {
    type Identity: PartyIdentifier + Send + Sync + 'static;
    type Error: std::error::Error + Send + Sync + 'static;

    fn send(
        &self,
        msg: Vec<u8>,
        to: Recipient<Self::Identity>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[cfg(test)]
mod tests {
    #[test]
    fn do_something() {}
}
