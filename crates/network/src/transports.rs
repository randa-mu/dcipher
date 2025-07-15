use crate::{PartyIdentifier, Recipient};

pub mod in_memory;
#[cfg(feature = "libp2p")]
pub mod libp2p;

#[derive(Clone, Debug)]
enum TransportAction<I: PartyIdentifier> {
    SendMessage(SendMessage<I>),
}

#[derive(Clone, Debug)]
struct SendMessage<I: PartyIdentifier> {
    to: Recipient<I>,
    msg: Vec<u8>,
}
