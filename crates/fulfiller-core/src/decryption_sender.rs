//! Module with signers / tx fulfillers used by the decryption sender contract.

pub mod async_signer;
pub mod single_call_fulfiller;
pub mod single_party_signer;

// Re-export contracts from contracts_core
pub mod contracts {
    pub use contracts_core::blocklock::decryption_sender::DecryptionSender;
}

use contracts_core::ser::IbeIdentityOnBn254G1CiphertextError;
use contracts_core::ser::EvmDeserialize;
use crate::decryption_sender::async_signer::DecryptionSenderAsyncSigner;
pub use crate::decryption_sender::contracts::DecryptionSender;
use crate::fulfiller::RetryStrategy;
use crate::fulfiller::ticker::TickerFulfiller;
use crate::fulfiller::{Identifier, TransactionFulfiller};
use crate::signer::AsynchronousSigner;
use alloy::primitives::{Bytes, U256};
use contracts_core::ibe_helper::{IbeIdentityOnBn254G1Ciphertext, PairingIbeCipherSuite};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::marker::PhantomData;

/// Helper struct used to instantiate a [`DecryptionSenderFulfiller`], an implementation of [`TickerFulfiller`].
pub struct DecryptionSenderFulfillerConfig<CS, S, TF>(
    PhantomData<fn(CS) -> CS>,
    PhantomData<fn(S) -> S>,
    PhantomData<fn(TF) -> TF>,
);

/// Type alias of [`TickerFulfiller`] used to fulfil decryption requests.
pub type DecryptionSenderFulfiller<RS, TF> =
    TickerFulfiller<DecryptionRequest, SignedDecryptionRequest<'static>, RS, TF>;

impl<CS, S, TF> DecryptionSenderFulfillerConfig<CS, S, TF>
where
    CS: PairingIbeCipherSuite,
    S: AsynchronousSigner<Bytes>,
    TF: TransactionFulfiller<SignedRequest = SignedDecryptionRequest<'static>>,
    DecryptionSenderAsyncSigner<CS, S>:
        AsynchronousSigner<DecryptionRequest, Signature = SignedDecryptionRequest<'static>>,
{
    /// Instantiate a [`DecryptionSenderFulfiller<RS, TF>`](DecryptionSenderFulfiller).
    pub fn new_fulfiller(
        cs: CS,
        signer: S,
        transaction_fulfiller: TF,
        max_fulfillment_per_tick: usize,
        retry_strategy: RetryStrategy,
    ) -> DecryptionSenderFulfiller<DecryptionSenderAsyncSigner<CS, S>, TF> {
        let signer = DecryptionSenderAsyncSigner::new(cs, signer);
        TickerFulfiller::new(
            signer,
            transaction_fulfiller,
            max_fulfillment_per_tick,
            retry_strategy,
        )
    }
}

/// Pending decryption request.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct DecryptionRequest {
    pub id: U256,
    pub condition: Bytes,
    pub ciphertext: Bytes,
}

/// Decryption request that has been signed and is ready to be fulfilled.
#[derive(Clone, Debug)]
pub struct SignedDecryptionRequest<'lt_cow> {
    pub id: U256,
    pub decryption_key: Bytes,
    pub signature: Cow<'lt_cow, Bytes>,
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

impl From<DecryptionSender::DecryptionRequested> for DecryptionRequest {
    fn from(value: DecryptionSender::DecryptionRequested) -> Self {
        Self {
            id: value.requestId,
            condition: value.condition,
            ciphertext: value.ciphertext,
        }
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
