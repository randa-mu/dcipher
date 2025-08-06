use adkg::helpers::PartyId;
use dcipher_network::transports::replayable::writer::InMemoryEntry;
use serde::{Deserialize, Serialize};

/// Vec<u8> serialized as base64 or bytes.
#[serde_with::serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerializedBytes(#[serde_as(as = "utils::Base64OrBytes")] Vec<u8>);

/// A list of broadcast messages
#[derive(Clone, Serialize, Deserialize)]
pub struct BroadcastMessages<M = SerializedBytes>(pub Vec<InMemoryEntry<PartyId, M>>);

/// A list of direct messages
#[derive(Clone, Serialize, Deserialize)]
pub struct DirectMessages<M = SerializedBytes> {
    pub recipient: PartyId,
    pub messages: Vec<InMemoryEntry<PartyId, M>>,
}

/// An encrypted adkg transcript that can be stored and sent to nodes.
pub type EncryptedAdkgTranscript = Vec<u8>;

impl From<Vec<u8>> for SerializedBytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<SerializedBytes> for Vec<u8> {
    fn from(value: SerializedBytes) -> Self {
        value.0
    }
}

impl From<DirectMessages> for DirectMessages<Vec<u8>> {
    fn from(value: DirectMessages) -> Self {
        Self {
            recipient: value.recipient,
            messages: value.messages.into_iter().map(|e| e.into_new_m()).collect(),
        }
    }
}

impl From<BroadcastMessages> for BroadcastMessages<Vec<u8>> {
    fn from(value: BroadcastMessages) -> Self {
        Self(value.0.into_iter().map(|e| e.into_new_m()).collect())
    }
}
