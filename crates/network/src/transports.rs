use crate::{PartyIdentifier, Recipient};

#[cfg(feature = "in_memory")]
pub mod in_memory;
#[cfg(feature = "libp2p")]
pub mod libp2p;

#[derive(Clone, Debug)]
pub enum TransportAction<I: PartyIdentifier> {
    SendMessage(SendMessage<I>),
}

#[derive(Clone, Debug)]
pub struct SendMessage<I: PartyIdentifier> {
    pub to: Recipient<I>,
    pub msg: Vec<u8>,
}
