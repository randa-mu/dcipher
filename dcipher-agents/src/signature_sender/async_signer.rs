//! [`AsynchronousSigner`] for signature requests.

use crate::ser::EvmSerialize;
use crate::signature_sender::{SignatureRequest, SignedSignatureRequest};
use crate::signer::AsynchronousSigner;
use ark_ec::CurveGroup;
use dcipher_signer::dsigner;
use dcipher_signer::dsigner::{
    ApplicationArgs, DSignerSchemeError, DSignerSchemeSigner, SignatureAlgorithm,
};
use std::marker::PhantomData;
use utils::serialize::point::PointDeserializeCompressed;

pub struct SignatureSenderAsyncSigner<CG, Signer> {
    signer: Signer,
    algorithm: SignatureAlgorithm,
    application_args: ApplicationArgs,
    _cg: PhantomData<fn(CG)>,
}

impl<CG, Signer> SignatureSenderAsyncSigner<CG, Signer> {
    pub fn new(
        signer: Signer,
        algorithm: SignatureAlgorithm,
        application_args: ApplicationArgs,
    ) -> Self {
        Self {
            signer,
            application_args,
            algorithm,
            _cg: PhantomData,
        }
    }
}

impl<CG, Signer> AsynchronousSigner<SignatureRequest> for SignatureSenderAsyncSigner<CG, Signer>
where
    CG: CurveGroup<Affine: PointDeserializeCompressed + EvmSerialize>,
    Signer: DSignerSchemeSigner + Send + Sync,
{
    type Error = DSignerSchemeError;
    type Signature = SignedSignatureRequest;

    async fn async_sign(&self, req: SignatureRequest) -> Result<Self::Signature, Self::Error> {
        let dsigner_req = dsigner::SignatureRequest {
            m: req.message_to_sign.into(),
            args: self.application_args.clone(),
            alg: self.algorithm,
        };
        let sig = self.signer.async_sign(dsigner_req).await?;
        let sig_cg = CG::Affine::deser(&sig).map_err(|e| Self::Error::Other(e.into()))?;

        Ok(SignedSignatureRequest {
            id: req.id,
            signature: EvmSerialize::ser_bytes(&sig_cg),
        })
    }
}
