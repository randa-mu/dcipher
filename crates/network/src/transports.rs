use crate::{PartyIdentifier, Recipient};

#[cfg(feature = "libp2p")]
pub mod libp2p;

#[derive(Clone, Debug)]
struct SendMessage<I: PartyIdentifier> {
    to: Recipient<I>,
    msg: Vec<u8>,
}
