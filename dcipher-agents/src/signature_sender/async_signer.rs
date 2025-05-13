//! [`AsynchronousSigner`] for signature requests.

use crate::ser::EvmSerialize;
use crate::signature_sender::{SignatureRequest, SignedSignatureRequest};
use crate::signer::AsynchronousSigner;
use alloy::primitives::Bytes;

pub struct SignatureSenderAsyncSigner<AsyncSigner>(AsyncSigner);

impl<AsyncSigner> SignatureSenderAsyncSigner<AsyncSigner> {
    pub fn new(signer: AsyncSigner) -> Self {
        Self(signer)
    }
}

impl<AsyncSigner> AsynchronousSigner<SignatureRequest> for SignatureSenderAsyncSigner<AsyncSigner>
where
    AsyncSigner: AsynchronousSigner<Bytes> + Send + Sync,
    AsyncSigner::Signature: EvmSerialize,
{
    type Error = AsyncSigner::Error;
    type Signature = SignedSignatureRequest;

    async fn async_sign(&self, req: SignatureRequest) -> Result<Self::Signature, Self::Error> {
        let sig = self.0.async_sign(req.message_to_sign).await?;
        Ok(SignedSignatureRequest {
            id: req.id,
            signature: EvmSerialize::ser_bytes(&sig),
        })
    }
}
