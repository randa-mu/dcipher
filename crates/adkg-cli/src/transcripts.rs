use adkg::helpers::PartyId;
use dcipher_network::transports::writer::InMemoryEntry;
use serde::{Deserialize, Serialize};

/// Vec<u8> serialized as base64 or bytes.
#[serde_with::serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerializedBytes(#[serde_as(as = "utils::Base64OrBytes")] Vec<u8>);

/// A list of broadcast messages
#[derive(Clone, Serialize, Deserialize)]
pub struct BroadcastMessages(pub Vec<InMemoryEntry<PartyId, SerializedBytes>>);

/// A list of direct messages
#[derive(Clone, Serialize, Deserialize)]
pub struct DirectMessages {
    pub recipient: PartyId,
    pub messages: Vec<InMemoryEntry<PartyId, SerializedBytes>>,
}

/// An encrypted adkg transcript that can be stored and sent to nodes.
pub type EncryptedAdkgTranscript = Vec<u8>;

impl From<Vec<u8>> for SerializedBytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
