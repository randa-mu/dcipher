//! Module with structs used for the signature sender contract.

mod async_signer;
pub mod contracts;

use crate::fulfiller::Identifier;
use crate::signature_sender::contracts::SignatureSender;
use alloy::primitives::{Bytes, U256};
use serde::{Deserialize, Serialize};

/// Pending signature request.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct SignatureRequest {
    pub(crate) id: U256,
    pub(crate) message_to_sign: Bytes,
    pub(crate) condition: Bytes,
}

#[derive(Clone, Debug)]
pub struct SignedSignatureRequest {
    pub(crate) id: U256,
    pub(crate) signature: Bytes,
}

impl Identifier for SignatureRequest {
    type Id = U256;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Identifier for SignedSignatureRequest {
    type Id = U256;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl From<SignatureSender::SignatureRequested> for SignatureRequest {
    fn from(value: SignatureSender::SignatureRequested) -> Self {
        Self {
            id: value.requestID,
            condition: value.condition,
            message_to_sign: value.message,
        }
    }
}
