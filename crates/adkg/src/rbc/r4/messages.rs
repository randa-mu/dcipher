//! Messages used by the 4 rounds rbc protocol

use serde::{Deserialize, Serialize};

/// Proposal ⟨PROPOSE, 𝑀⟩ sent by the broadcast leader to the rest of the parties.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Propose {
    #[serde(with = "serde_bytes")]
    pub(crate) m: Vec<u8>,
}

/// Echo ⟨ECHO, 𝑚𝑗, ℎ⟩ sent by participants upon accepting the proposal.
/// mj is the vector of codewords owned by party j, while h is the hash of the dealer's message.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Echo {
    #[serde(with = "serde_bytes")]
    pub(crate) m: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub(crate) h: Vec<u8>,
}

/// Ready ⟨READY, 𝑚𝑖, ℎ⟩ sent by participants upon receiving 2t + 1 echo messages with matching mi, h.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ready {
    #[serde(with = "serde_bytes")]
    pub(crate) m: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub(crate) h: Vec<u8>,
}

/// Messages sent during the 4 round RBC protocol.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Message {
    Propose(Propose),
    Echo(Echo),
    Ready(Ready),
}
