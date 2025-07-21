//! Messages used by the 4 rounds rbc protocol

/// Proposal ⟨PROPOSE, 𝑀⟩ sent by the broadcast leader to the rest of the parties.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Propose {
    pub(crate) m: Vec<u8>,
}

/// Echo ⟨ECHO, 𝑚𝑗, ℎ⟩ sent by participants upon accepting the proposal.
/// mj is the vector of codewords owned by party j, while h is the hash of the dealer's message.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Echo {
    pub(crate) m: Vec<u8>,
    pub(crate) h: Vec<u8>,
}

/// Ready ⟨READY, 𝑚𝑖, ℎ⟩ sent by participants upon receiving 2t + 1 echo messages with matching mi, h.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ready {
    pub(crate) m: Vec<u8>,
    pub(crate) h: Vec<u8>,
}

/// Messages sent during the 4 round RBC protocol.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Message {
    Propose(Propose),
    Echo(Echo),
    Ready(Ready),
}
