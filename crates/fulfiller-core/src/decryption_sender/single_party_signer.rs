//! Concrete implementation of a [`AsynchronousSigner`] for the single-party setting where
//! signatures and decryption keys are issued immediately.

use contracts_core::blocklock::decryption_sender::{DecryptionRequest, SignedDecryptionRequest};
use crate::ibe_helper::{IbeCiphertext, PairingIbeSigner};
use crate::ser::EvmSerialize;
use crate::signer::AsynchronousSigner;
use alloy::primitives::Bytes;
use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum StandaloneSignerError {
    #[error("failed to parse decryption requests")]
    ParseDecryptionRequest(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[derive(Clone)]
pub struct StandaloneSigner<CS: PairingIbeSigner>(CS);

impl<CS> StandaloneSigner<CS>
where
    CS: PairingIbeSigner,
{
    pub fn new(cs: CS) -> Self {
        Self(cs)
    }
}

impl<CS> StandaloneSigner<CS>
where
    CS: PairingIbeSigner + Send + Sync + 'static,
    for<'a> &'a DecryptionRequest: TryInto<CS::Ciphertext>,
    for<'a> <&'a DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
    CS::IdentityGroup: EvmSerialize,
{
    fn process_request(
        &self,
        req: &DecryptionRequest,
    ) -> Result<SignedDecryptionRequest<'static>, StandaloneSignerError> {
        // Generate a signature (also a decryption key in this context) for each condition
        let identity = self.0.h1(&req.condition);
        let sig = self.0.decryption_key(identity);
        let sig_bytes = Cow::<'_, Bytes>::Owned(EvmSerialize::ser_bytes(&sig));
        // Preprocess decryption keys using the signature and the ciphertext's ephemeral public key
        let ct: CS::Ciphertext = match req.try_into() {
            Ok(ct) => ct,
            Err(e) => {
                // If we fail to generate keys, it is likely due to an invalid ephemeral public key / ciphertext,
                // not much we can do here.
                tracing::error!(error = %e, request_id = %req.id, "Failed to generate decryption keys / signature... ignoring request");
                Err(StandaloneSignerError::ParseDecryptionRequest(e.into()))?
            }
        };
        let preprocessed_key = self.0.preprocess_decryption_key(sig, ct.ephemeral_pk());

        Ok(SignedDecryptionRequest::new(
            req.id,
            Bytes::from(preprocessed_key.as_ref().to_vec()),
            sig_bytes.clone(),
        ))
    }
}

impl<CS> AsynchronousSigner<DecryptionRequest> for StandaloneSigner<CS>
where
    CS: PairingIbeSigner + Send + Sync + 'static,
    for<'a> &'a DecryptionRequest: TryInto<CS::Ciphertext>,
    for<'a> <&'a DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
    CS::IdentityGroup: EvmSerialize,
{
    type Error = StandaloneSignerError;
    type Signature = SignedDecryptionRequest<'static>;

    async fn async_sign(&self, req: DecryptionRequest) -> Result<Self::Signature, Self::Error> {
        self.process_request(&req)
    }
}
