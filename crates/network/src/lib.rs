use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod topic;
#[cfg(feature = "transports")]
pub mod transports;

pub trait PartyIdentifier: std::fmt::Display + Clone + Debug + Eq + PartialEq {}

impl<T> PartyIdentifier for T where T: std::fmt::Display + Clone + Debug + Eq + PartialEq {}

/// Recipient of a message.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Recipient<I: PartyIdentifier> {
    All,
    AllIncludingSelf,
    Single(I),
}

/// Type of message, broadcast or direct (i.e., a point to point message).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Broadcast,
    Direct,
}

impl<I: PartyIdentifier> From<Recipient<I>> for MessageType {
    fn from(value: Recipient<I>) -> Self {
        match value {
            Recipient::All | Recipient::AllIncludingSelf => MessageType::Broadcast,
            Recipient::Single(_) => MessageType::Direct,
        }
    }
}

/// Message received from a sender.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceivedMessage<I: PartyIdentifier, M = Vec<u8>> {
    pub sender: I,
    pub message_type: MessageType,
    pub content: M,
}

impl<I, M> ReceivedMessage<I, M>
where
    I: PartyIdentifier,
{
    pub fn new(sender: I, content: M, message_type: MessageType) -> Self {
        Self {
            sender,
            content,
            message_type,
        }
    }

    pub fn new_broadcast(sender: I, content: M) -> Self {
        Self {
            content,
            sender,
            message_type: MessageType::Broadcast,
        }
    }

    pub fn new_direct(sender: I, content: M) -> Self {
        Self {
            content,
            sender,
            message_type: MessageType::Direct,
        }
    }
}

/// A transport trait that can be used to obtain senders and incoming message streams.
pub trait Transport {
    type Error: std::error::Error + Send + Sync + 'static;
    type Identity: PartyIdentifier + Send + Sync + 'static;

    type ReceiveMessageStream: futures_util::Stream<Item = Result<ReceivedMessage<Self::Identity>, Self::Error>>
        + Send
        + Unpin
        + 'static;

    type Sender: TransportSender<Identity = Self::Identity> + Clone + Send + Sync + 'static;

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

    fn broadcast(&self, msg: Vec<u8>) -> impl Future<Output = Result<(), Self::Error>> + Send {
        self.send(msg, Recipient::All)
    }

    fn broadcast_echo_self(
        &self,
        msg: Vec<u8>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        self.send(msg, Recipient::AllIncludingSelf)
    }

    fn send_single(
        &self,
        msg: Vec<u8>,
        to: Self::Identity,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        self.send(msg, Recipient::Single(to))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn do_something() {}
}
