pub mod agents;
pub mod decryption_sender;
pub mod fulfiller;
pub mod ibe_helper;
pub(crate) mod ser;
pub mod signature_sender;
pub mod signer;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize, Debug,
)]
pub struct RequestId(pub alloy::primitives::U256);

impl From<RequestId> for alloy::primitives::U256 {
    fn from(value: RequestId) -> Self {
        value.0
    }
}

impl From<alloy::primitives::U256> for RequestId {
    fn from(value: alloy::primitives::U256) -> Self {
        Self(value)
    }
}

impl AsRef<alloy::primitives::U256> for RequestId {
    fn as_ref(&self) -> &alloy::primitives::U256 {
        &self.0
    }
}
