//! Module with signers / tx fulfillers used by the decryption sender contract.

pub mod async_signer;
pub mod contracts;
pub mod single_call_fulfiller;
pub mod single_party_signer;

use crate::decryption_sender::contracts::DecryptionSender;
use crate::fulfiller::RetryStrategy;
use crate::fulfiller::ticker::TickerFulfiller;
use crate::fulfiller::{Identifier, TransactionFulfiller};
use crate::ibe_helper::IbeIdentityOnBn254G1Ciphertext;
use crate::ser::{EvmDeserialize, IbeIdentityOnBn254G1CiphertextError};
use crate::signer::RequestSigningRegistry;
use alloy::primitives::{Bytes, U256};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::marker::PhantomData;

/// Helper struct used to instantiate a [`DecryptionSenderFulfiller`], an implementation of [`TickerFulfiller`].
pub struct DecryptionSenderFulfillerConfig<RS, TF>(
    PhantomData<fn(RS) -> RS>,
    PhantomData<fn(TF) -> TF>,
);

/// Type alias of [`TickerFulfiller`] used to fulfil decryption requests.
pub type DecryptionSenderFulfiller<RS, TF> =
    TickerFulfiller<DecryptionRequest, SignedDecryptionRequest<'static>, RS, TF>;

impl<RS, TF> DecryptionSenderFulfillerConfig<RS, TF>
where
    RS: RequestSigningRegistry<
            Request = DecryptionRequest,
            SignedRequest = SignedDecryptionRequest<'static>,
        >,
    TF: TransactionFulfiller<SignedRequest = SignedDecryptionRequest<'static>>,
{
    /// Instantiate a [`DecryptionSenderFulfiller<RS, TF>`](DecryptionSenderFulfiller).
    pub fn new_fulfiller(
        signing_registry: RS,
        transaction_fulfiller: TF,
        max_fulfillment_per_tick: usize,
        retry_strategy: RetryStrategy,
    ) -> DecryptionSenderFulfiller<RS, TF> {
        TickerFulfiller::new(
            signing_registry,
            transaction_fulfiller,
            max_fulfillment_per_tick,
            retry_strategy,
        )
    }
}

/// Pending decryption request.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct DecryptionRequest {
    pub(crate) id: U256,
    pub(crate) condition: Bytes,
    pub(crate) ciphertext: Bytes,
}

/// Decryption request that has been signed and is ready to be fulfilled.
#[derive(Clone, Debug)]
pub struct SignedDecryptionRequest<'lt_cow> {
    pub(crate) id: U256,
    pub(crate) decryption_key: Bytes,
    pub(crate) signature: Cow<'lt_cow, Bytes>,
}

impl Identifier for DecryptionRequest {
    type Id = U256;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<'lt_cow> SignedDecryptionRequest<'lt_cow> {
    pub(crate) fn new(id: U256, decryption_key: Bytes, signature: Cow<'lt_cow, Bytes>) -> Self {
        Self {
            id,
            decryption_key,
            signature,
        }
    }
}

impl Identifier for SignedDecryptionRequest<'_> {
    type Id = U256;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl TryFrom<&DecryptionRequest> for IbeIdentityOnBn254G1Ciphertext {
    type Error = IbeIdentityOnBn254G1CiphertextError;

    fn try_from(value: &DecryptionRequest) -> Result<Self, Self::Error> {
        EvmDeserialize::deser(&value.ciphertext)
    }
}

impl TryFrom<DecryptionRequest> for IbeIdentityOnBn254G1Ciphertext {
    type Error = IbeIdentityOnBn254G1CiphertextError;

    fn try_from(value: DecryptionRequest) -> Result<Self, Self::Error> {
        EvmDeserialize::deser(&value.ciphertext)
    }
}

impl From<DecryptionSender::DecryptionRequested> for DecryptionRequest {
    fn from(value: DecryptionSender::DecryptionRequested) -> Self {
        Self {
            id: value.requestID,
            condition: value.condition,
            ciphertext: value.ciphertext,
        }
    }
}
