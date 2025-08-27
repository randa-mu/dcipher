//! Concrete implementation of [`DSignerScheme`].

use crate::bls::filter::BlsFilter;
use crate::bls::{BlsSignatureRequest, SharedSignatureCache, StoredSignatureRequest};
use crate::bls::{BlsVerifier, G1Affine, G2Affine};
use crate::dsigner::{
    ApplicationArgs, DSignerScheme, DSignerSchemeError, DSignerSchemeSigner, SchemeDetails,
    SignatureAlgorithm, SignatureRequest, VerificationParameters,
};
use bytes::Bytes;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use itertools::Either;
use serde::{Deserialize, Serialize};
use utils::serialize::point::{
    PointDeserializeCompressed, PointSerializeCompressed, PointSerializeUncompressed,
};

pub struct AsyncThresholdSigner<BLS: BlsVerifier> {
    scheme_details: SchemeDetails,
    signatures_cache: SharedSignatureCache<BLS>,
    new_sig_request: tokio::sync::mpsc::UnboundedSender<BlsSignatureRequest>,
    filter: BlsFilter,
}

impl<BLS: BlsVerifier> AsyncThresholdSigner<BLS> {
    pub(super) fn new(
        scheme_details: SchemeDetails,
        signatures_cache: SharedSignatureCache<BLS>,
        new_sig_request: tokio::sync::mpsc::UnboundedSender<BlsSignatureRequest>,
        filter: BlsFilter,
    ) -> Self {
        Self {
            scheme_details,
            signatures_cache,
            new_sig_request,
            filter,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct PartialSignature<G> {
    id: u16,
    sig: G,
}

#[derive(thiserror::Error, Debug)]
pub enum AsyncThresholdSignerError {
    #[error("the specified application is not supported by the signer")]
    ApplicationNotSupported,

    #[error("the specified algorithm is not supported by the signer")]
    AlgorithmNotSupported,

    #[error("the message to sign has been dropped from cache")]
    DroppedFromCache,

    #[error("the watch sender has been dropped")]
    WatchSenderDropped,

    #[error("the channel used to request signatures has been closed")]
    CannotRequestNewSignatures,

    #[error("failed to serialized signature to bytes")]
    PointSerializationError(utils::serialize::SerializationError),
}

impl From<AsyncThresholdSignerError> for DSignerSchemeError {
    fn from(error: AsyncThresholdSignerError) -> Self {
        match error {
            AsyncThresholdSignerError::ApplicationNotSupported => {
                DSignerSchemeError::ApplicationNotSupported
            }
            AsyncThresholdSignerError::AlgorithmNotSupported => {
                DSignerSchemeError::AlgorithmNotSupported
            }
            _ => DSignerSchemeError::Other(error.into()),
        }
    }
}

impl<BLS: BlsVerifier> DSignerScheme for AsyncThresholdSigner<BLS>
where
    G1Affine<BLS>:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
    G2Affine<BLS>:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
{
    fn details(&self) -> SchemeDetails {
        self.scheme_details.clone()
    }

    fn verification_parameters(
        &self,
        alg: &SignatureAlgorithm,
        args: &ApplicationArgs,
    ) -> Result<VerificationParameters, DSignerSchemeError> {
        // Get dst
        let SignatureAlgorithm::Bls(bls_alg) = alg else {
            Err(AsyncThresholdSignerError::AlgorithmNotSupported)?
        };
        let Some(dst) = self.filter.get_rfc9380_dst_if_supported(args, bls_alg) else {
            Err(AsyncThresholdSignerError::ApplicationNotSupported)?
        };

        // Find compatible public key
        let public_key = self
            .scheme_details
            .scheme_algs
            .iter()
            .find(|scheme| scheme.algs.contains(alg)) // tiny number of algs
            .ok_or(AsyncThresholdSignerError::AlgorithmNotSupported)?
            .public_key
            .clone();

        Ok(VerificationParameters { dst, public_key })
    }
}

impl<BLS: BlsVerifier> DSignerSchemeSigner for AsyncThresholdSigner<BLS>
where
    G1Affine<BLS>:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
    G2Affine<BLS>:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
{
    fn async_sign(&self, req: SignatureRequest) -> BoxFuture<Result<Bytes, DSignerSchemeError>> {
        async move {
            let SignatureAlgorithm::Bls(alg) = req.alg else {
                Err(AsyncThresholdSignerError::AlgorithmNotSupported)?
            };

            // Obtain dst to try and fetch existing signature
            let Some(dst) = self.filter.get_rfc9380_dst_if_supported(&req.args, &alg) else {
                Err(AsyncThresholdSignerError::ApplicationNotSupported)?
            };
            let stored_req = StoredSignatureRequest {
                m: req.m.clone(),
                dst,
            };

            // We have three possibilities here:
            //  1. The request is not yet present in the map
            //      => a. insert a watch sender in the map,
            //         b. we notify of a new request,
            //         c. return a future awaiting the signature through the watch receiver.
            //  2. The request is in the map
            //    2a. it contains a signature
            //         => return a future that resolves immediately with the signature
            //    2b. it contains a watch sender
            //         => do 1.b. and 1.c.
            let signature_or_receiver = {
                let mut signatures_cache = self
                    .signatures_cache
                    .lock()
                    .expect("a thread panicked with the mutex");

                // This may drop the LRU entry from the map, which results in the
                // future owning the corresponding receiver resolving in an error.
                let signature_or_sender =
                    signatures_cache.get_or_insert(stored_req.clone(), || {
                        let (tx, _) = tokio::sync::watch::channel(None);
                        Either::Right(tx)
                    });

                match signature_or_sender {
                    Either::Left(signature) => {
                        // 2a. The message is in the map and contains a signature
                        Result::<_, AsyncThresholdSignerError>::Ok(Either::Left(
                            signature.to_owned(),
                        ))
                    }

                    Either::Right(tx) => {
                        let rx = tx.subscribe();

                        // Notify of the new message to sign
                        self.new_sig_request
                            .send(BlsSignatureRequest {
                                m: req.m,
                                args: req.args,
                                alg,
                            })
                            .map_err(|_| AsyncThresholdSignerError::CannotRequestNewSignatures)?;

                        Ok(Either::Right(rx))
                    }
                }
            }?;

            // If the signature was cached, return immediately
            let sig: Either<G1Affine<BLS>, G2Affine<BLS>> = match signature_or_receiver {
                Either::Left(signature) => signature,
                Either::Right(mut rx) => {
                    // A signature may already be in the channel, borrow it and mark it as seen
                    let signature = rx.borrow_and_update().to_owned();

                    if let Some(sig) = signature {
                        // If it contains a signature, simply return
                        sig
                    } else {
                        // Does not yet contain a signature, await for a change and return
                        match rx.changed().await {
                            Ok(()) => rx
                                    .borrow_and_update()
                                    .to_owned()
                                    .expect("watch channel updated but sig is None"),
                            Err(_) => Err(AsyncThresholdSignerError::WatchSenderDropped)?,
                        }
                    }
                }
            };

            let sig = either::for_both!(sig, sig => if alg.compression { sig.ser_compressed() } else { sig.ser_uncompressed() })
                .map_err(AsyncThresholdSignerError::PointSerializationError)?;
            Ok(sig.into())
        }
        .boxed()
    }
}
