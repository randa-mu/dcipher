//! Module with structs used for the signature sender contract.

mod async_signer;
pub mod contracts;
pub mod fulfiller;

use crate::fulfiller::ticker::TickerFulfiller;
use crate::fulfiller::{Identifier, RetryStrategy, TransactionFulfiller};
use crate::signature_sender::async_signer::SignatureSenderAsyncSigner;
use crate::signature_sender::contracts::SignatureSender;
use crate::signer::AsynchronousSigner;
use alloy::primitives::{Bytes, U256};
use dcipher_signer::dsigner::{ApplicationArgs, DSignerSchemeSigner, SignatureAlgorithm};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Helper struct used to instantiate a [`SignatureSenderFulfiller`], an implementation of [`TickerFulfiller`].
pub struct SignatureSenderFulfillerConfig<CG, S, TF>(
    PhantomData<fn(CG) -> CG>,
    PhantomData<fn(S) -> S>,
    PhantomData<fn(TF) -> TF>,
);

/// Type alias of [`TickerFulfiller`] used to fulfil signature requests.
pub type SignatureSenderFulfiller<RS, TF> =
    TickerFulfiller<SignatureRequest, SignedSignatureRequest, RS, TF>;

impl<CG, S, TF> SignatureSenderFulfillerConfig<CG, S, TF>
where
    S: DSignerSchemeSigner,
    TF: TransactionFulfiller<SignedRequest = SignedSignatureRequest>,
    SignatureSenderAsyncSigner<CG, S>:
        AsynchronousSigner<SignatureRequest, Signature = SignedSignatureRequest>,
{
    /// Instantiate a [`SignatureSenderFulfiller<RS, TF>`](SignatureSenderFulfiller).
    pub fn new_fulfiller(
        signer: S,
        algorithm: SignatureAlgorithm,
        application_args: ApplicationArgs,
        transaction_fulfiller: TF,
        max_fulfillment_per_tick: usize,
        retry_strategy: RetryStrategy,
    ) -> SignatureSenderFulfiller<SignatureSenderAsyncSigner<CG, S>, TF> {
        let signer = SignatureSenderAsyncSigner::new(signer, algorithm, application_args);
        TickerFulfiller::new(
            signer,
            transaction_fulfiller,
            max_fulfillment_per_tick,
            retry_strategy,
        )
    }
}

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
    #[allow(unused)]
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
