//! Module with signers / tx fulfillers used by the decryption sender contract.

pub mod async_signer;
pub mod single_call_fulfiller;
pub mod single_party_signer;

// Re-export contracts from contracts_core
pub mod contracts {
    pub use contracts_core::blocklock::decryption_sender::DecryptionSender;
}

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

#[derive(thiserror::Error, Debug)]
pub enum DecryptionRequestConversionError {
    #[error("Invalid ciphertext format: {0}")]
    InvalidCiphertext(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

impl TryInto<IbeIdentityOnBn254G1Ciphertext> for DecryptionRequest {
    type Error = DecryptionRequestConversionError;

    fn try_into(self) -> Result<IbeIdentityOnBn254G1Ciphertext, Self::Error> {
        // Parse the ciphertext bytes as a G2 point
        // The ciphertext should contain the ephemeral public key as a G2 point
        if self.ciphertext.len() != 128 {
            return Err(DecryptionRequestConversionError::InvalidCiphertext(
                format!("Expected 128 bytes, got {}", self.ciphertext.len()),
            ));
        }

        // Split into x and y coordinates (64 bytes each)
        let x_bytes = &self.ciphertext[0..64];
        let y_bytes = &self.ciphertext[64..128];

        // Parse x coordinate (c0, c1)
        let x_c0_bytes = &x_bytes[0..32];
        let x_c1_bytes = &x_bytes[32..64];

        // Parse y coordinate (c0, c1)
        let y_c0_bytes = &y_bytes[0..32];
        let y_c1_bytes = &y_bytes[32..64];

        use ark_bn254::{Fq2, G2Affine};
        use ark_ff::PrimeField;

        // Convert bytes to field elements
        let x_c0 = ark_bn254::Fq::from_be_bytes_mod_order(x_c0_bytes);
        let x_c1 = ark_bn254::Fq::from_be_bytes_mod_order(x_c1_bytes);
        let y_c0 = ark_bn254::Fq::from_be_bytes_mod_order(y_c0_bytes);
        let y_c1 = ark_bn254::Fq::from_be_bytes_mod_order(y_c1_bytes);

        let x = Fq2::new(x_c0, x_c1);
        let y = Fq2::new(y_c0, y_c1);

        let eph_pk = G2Affine::new(x, y);

        if !eph_pk.is_on_curve() {
            return Err(DecryptionRequestConversionError::InvalidCiphertext(
                "Point is not on curve".to_string(),
            ));
        }

        Ok(IbeIdentityOnBn254G1Ciphertext::new(eph_pk))
    }
}
