//! Concrete implementation of a [`RequestSigningRegistry`] for the single-party setting where
//! signatures and decryption keys are issued immediately.

use crate::decryption_sender::{DecryptionRequest, SignedDecryptionRequest};
use crate::ibe_helper::{IbeCipherSuite, IbeCiphertext};
use crate::ser::EvmSerialize;
use crate::signer::RequestSigningRegistry;
use alloy::primitives::Bytes;
use ark_ec::AffineRepr;
use std::borrow::Cow;

pub struct StandaloneRegistry<CS>
where
    CS: IbeCipherSuite,
{
    // Ciphersuite and secret key
    cs: CS,
    sk: <CS::IdentityGroup as AffineRepr>::ScalarField,
}

#[derive(thiserror::Error, Debug)]
pub enum StandaloneRegistryError {
    #[error("failed to parse decryption requests")]
    ParseDecryptionRequest(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

pub struct StandaloneSigner<CS: IbeCipherSuite>(StandaloneRegistry<CS>);

impl<CS> StandaloneSigner<CS>
where
    CS: IbeCipherSuite,
    StandaloneRegistry<CS>: RequestSigningRegistry,
{
    pub fn new(cs: CS, sk: <CS::IdentityGroup as AffineRepr>::ScalarField) -> Self {
        Self(StandaloneRegistry { cs, sk })
    }

    pub fn registry(self) -> StandaloneRegistry<CS> {
        self.0
    }
}

impl<CS> StandaloneRegistry<CS>
where
    CS: IbeCipherSuite + Send + Sync + 'static,
    for<'a> &'a DecryptionRequest: TryInto<CS::Ciphertext>,
    for<'a> <&'a DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
    CS::IdentityGroup: EvmSerialize,
{
    fn process_request(
        &self,
        req: &DecryptionRequest,
    ) -> Result<SignedDecryptionRequest<'static>, StandaloneRegistryError> {
        // Generate a signature (also a decryption key in this context) for each condition
        let identity = self.cs.h1(&req.condition);
        let sig = self.cs.decryption_key(&self.sk, identity);
        let sig_bytes = Cow::<'_, Bytes>::Owned(EvmSerialize::ser_bytes(&sig));
        // Preprocess decryption keys using the signature and the ciphertext's ephemeral public key
        let ct: CS::Ciphertext = match req.try_into() {
            Ok(ct) => ct,
            Err(e) => {
                // If we fail to generate keys, it is likely due to an invalid ephemeral public key / ciphertext,
                // not much we can do here.
                tracing::error!(error = %e, request_id = %req.id, "Failed to generate decryption keys / signature... ignoring request");
                Err(StandaloneRegistryError::ParseDecryptionRequest(e.into()))?
            }
        };
        let preprocessed_key = self.cs.preprocess_decryption_key(sig, ct.ephemeral_pk());

        Ok(SignedDecryptionRequest::new(
            req.id,
            Bytes::from(preprocessed_key.as_ref().to_vec()),
            sig_bytes.clone(),
        ))
    }
}

impl<CS> RequestSigningRegistry for StandaloneRegistry<CS>
where
    CS: IbeCipherSuite + Send + Sync + 'static,
    for<'a> &'a DecryptionRequest: TryInto<CS::Ciphertext>,
    for<'a> <&'a DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
    CS::IdentityGroup: EvmSerialize,
{
    type Request = DecryptionRequest;
    type SignedRequest = SignedDecryptionRequest<'static>;

    fn try_fetch_signed_requests<'lt_self, 'lt_r, 'lt_rr>(
        &'lt_self self,
        inputs: impl IntoIterator<Item = &'lt_r Self::Request> + 'lt_self,
    ) -> impl Iterator<Item = Option<Self::SignedRequest>> + 'lt_self
    where
        'lt_r: 'lt_self,
    {
        inputs.into_iter().map(|req| self.process_request(req).ok())
    }
}
