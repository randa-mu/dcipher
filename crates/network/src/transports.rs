use crate::PartyIdentifier;

#[cfg(feature = "in_memory")]
pub mod in_memory;
#[cfg(feature = "libp2p")]
pub mod libp2p;
#[cfg(feature = "replayable")]
pub mod replayable;

#[derive(Clone, Debug)]
pub enum TransportAction<I: PartyIdentifier> {
    SendDirectMessage(SendDirectMessage<I>),
    SendBroadcastMessage(SendBroadcastMessage),
}

#[derive(Clone, Debug)]
pub struct SendDirectMessage<I: PartyIdentifier> {
    pub to: I,
    pub msg: Vec<u8>,
}

impl<I: PartyIdentifier> SendDirectMessage<I> {
    pub fn new(to: I, msg: Vec<u8>) -> Self {
        Self { to, msg }
    }
}
#[derive(Clone, Debug)]
pub struct SendBroadcastMessage {
    pub msg: Vec<u8>,
    pub broadcast_self: bool,
}

impl SendBroadcastMessage {
    pub fn new(msg: Vec<u8>, broadcast_self: bool) -> Self {
        Self {
            msg,
            broadcast_self,
        }
    }
}

impl<I: PartyIdentifier> From<SendDirectMessage<I>> for TransportAction<I> {
    fn from(msg: SendDirectMessage<I>) -> TransportAction<I> {
        TransportAction::SendDirectMessage(msg)
    }
}

impl<I: PartyIdentifier> From<SendBroadcastMessage> for TransportAction<I> {
    fn from(msg: SendBroadcastMessage) -> TransportAction<I> {
        TransportAction::SendBroadcastMessage(msg)
    }
}
