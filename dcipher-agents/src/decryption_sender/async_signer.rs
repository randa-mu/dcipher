//! [`AsynchronousSigner`] for decryption requests. Unlike [`AsyncThresholdSigner`](crate::signer::bls::AsyncThresholdSigner),
//! this signer allows to sign identical conditions as is often the case with the decryption sender contract.

use crate::decryption_sender::{DecryptionRequest, SignedDecryptionRequest};
use crate::ibe_helper::{IbeCiphertext, PairingIbeCipherSuite};
use crate::signer::AsynchronousSigner;
use alloy::primitives::Bytes;
use dcipher_signer::dsigner::{
    ApplicationArgs, DSignerSchemeError, DSignerSchemeSigner, SignatureAlgorithm, SignatureRequest,
};
use std::borrow::Cow;
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeUncompressed};

pub struct DecryptionSenderAsyncSigner<CS, Signer> {
    cs: CS,
    signer: Signer,
    algorithm: SignatureAlgorithm,
    application_args: ApplicationArgs,
}

impl<CS, Signer> DecryptionSenderAsyncSigner<CS, Signer> {
    pub fn new(
        cs: CS,
        signer: Signer,
        algorithm: SignatureAlgorithm,
        application_args: ApplicationArgs,
    ) -> Self {
        Self {
            cs,
            signer,
            algorithm,
            application_args,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DecryptionSenderAsyncSignerError {
    #[error("failed to parse decryption request into a valid ciphertext")]
    ParseCiphertext(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to parse signature from signer")]
    ParseSignature(#[from] utils::serialize::SerializationError),

    #[error("failed to request signature from async signer")]
    UnderlyingAsyncSigner(#[from] DSignerSchemeError),
}

impl<CS, Signer> AsynchronousSigner<DecryptionRequest> for DecryptionSenderAsyncSigner<CS, Signer>
where
    CS: PairingIbeCipherSuite + Send + Sync,
    CS::IdentityGroup: PointDeserializeCompressed + PointSerializeUncompressed,
    Signer: DSignerSchemeSigner + Sync,
    DecryptionRequest: TryInto<CS::Ciphertext>,
    <DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    type Error = DecryptionSenderAsyncSignerError;
    type Signature = SignedDecryptionRequest<'static>;

    async fn async_sign(&self, req: DecryptionRequest) -> Result<Self::Signature, Self::Error> {
        let signature_req = SignatureRequest {
            alg: self.algorithm,
            args: self.application_args.clone(),
            m: req.condition.clone().into(),
        };

        // Await signature
        let sig = self
            .signer
            .async_sign(signature_req)
            .await
            .map_err(DecryptionSenderAsyncSignerError::UnderlyingAsyncSigner)?;

        let sig = CS::IdentityGroup::deser_compressed(&sig)?;
        let sig_bytes = Cow::Owned(sig.ser_uncompressed()?.into());
        // Preprocess decryption keys using the signature and the ciphertext's ephemeral public key
        let request_id = req.id;
        let ct: CS::Ciphertext = match req.try_into() {
            Ok(ct) => ct,
            Err(e) => {
                // If we fail to generate keys, it is likely due to an invalid ephemeral public key / ciphertext,
                // not much we can do here.
                tracing::error!(error = %e, %request_id, "Failed to generate decryption keys / signature... ignoring request");
                Err(DecryptionSenderAsyncSignerError::ParseCiphertext(e.into()))?
            }
        };
        let preprocessed_key = self.cs.preprocess_decryption_key(sig, ct.ephemeral_pk());

        Ok(SignedDecryptionRequest::new(
            request_id,
            Bytes::from(preprocessed_key.as_ref().to_vec()),
            sig_bytes,
        ))
    }
}

#[cfg(test)]
#[cfg(all(feature = "blocklock", feature = "bn254"))] // need blocklock types for ibe
pub(crate) mod tests {
    use super::*;
    use crate::decryption_sender::DecryptionRequest;
    use crate::ibe_helper::{IbeIdentityOnBn254G1Suite, PairingIbeCipherSuite, PairingIbeSigner};
    use crate::ser::tests::bn254::encode_ciphertext;
    use crate::signer::AsynchronousSigner;
    use alloy::primitives::U256;
    use ark_bn254::Fr;
    use ark_ec::{AffineRepr, CurveGroup};
    use ark_ff::{BigInteger, MontFp, PrimeField};
    use bytes::Bytes;
    use dcipher_signer::dsigner::{
        ApplicationAnyArgs, BlsSignatureAlgorithm, BlsSignatureCurve, BlsSignatureHash,
    };
    use futures_util::FutureExt;
    use futures_util::future::BoxFuture;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::watch;
    use utils::serialize::point::PointSerializeCompressed;

    pub(crate) fn create_ciphertext(eph_pk: ark_bn254::G2Affine) -> alloy::primitives::Bytes {
        let (x, y) = eph_pk.xy().unwrap();
        let (x, y) = (*x, *y);
        let x0 = x.c0.into_bigint().to_bytes_be();
        let x1 = x.c1.into_bigint().to_bytes_be();
        let y0 = y.c0.into_bigint().to_bytes_be();
        let y1 = y.c1.into_bigint().to_bytes_be();

        encode_ciphertext(&x0, &x1, &y0, &y1)
    }

    #[derive(Clone, Debug, thiserror::Error)]
    #[error("mock signer error")]
    struct MockSignerError;

    type SignatureResult = Option<Result<Bytes, MockSignerError>>;

    #[derive(Clone)]
    struct MockSigner {
        receivers: Arc<tokio::sync::Mutex<HashMap<Bytes, watch::Receiver<SignatureResult>>>>,
        senders: Arc<std::sync::Mutex<HashMap<Bytes, watch::Sender<SignatureResult>>>>,
    }

    impl MockSigner {
        fn new(conditions: impl IntoIterator<Item = Bytes>) -> Self {
            let (txs, rxs): (Vec<_>, Vec<_>) = conditions
                .into_iter()
                .map(|c| {
                    let (tx, rx) = watch::channel(None);
                    ((c.clone(), tx), (c, rx))
                })
                .collect();

            let receivers = HashMap::from_iter(rxs);
            let senders = HashMap::from_iter(txs);
            Self {
                receivers: Arc::new(tokio::sync::Mutex::new(receivers)),
                senders: Arc::new(std::sync::Mutex::new(senders)),
            }
        }

        // Set the response for a specific request
        fn set_response(&self, condition: &Bytes, result: Result<Bytes, MockSignerError>) {
            let tx = self
                .senders
                .lock()
                .expect("task holding mutex panicked")
                .remove(condition)
                .expect("condition not found");
            tx.send_replace(Some(result));
        }
    }

    impl DSignerSchemeSigner for MockSigner {
        fn async_sign(
            &self,
            req: SignatureRequest,
        ) -> BoxFuture<'_, Result<Bytes, DSignerSchemeError>> {
            let receivers = self.receivers.clone().lock_owned();
            async move {
                let mut rx = receivers
                    .await
                    .get(&req.m)
                    .expect("no receiver found for condition")
                    .clone();
                rx.changed().await.expect("failed to await has_changed");

                rx.borrow_and_update()
                    .clone()
                    .unwrap()
                    .map_err(|e| DSignerSchemeError::Other(e.into()))
            }
            .boxed()
        }
    }

    fn new_decryption_request(condition: Bytes) -> DecryptionRequest {
        let ct = create_ciphertext(ark_bn254::G2Affine::generator());
        DecryptionRequest {
            id: U256::ZERO,
            ciphertext: ct,
            condition: condition.into(),
        }
    }

    #[tokio::test]
    async fn test_different_conditions_concurrent_requests() {
        let global_timeout = std::time::Duration::from_millis(2000);

        // Create two different conditions
        let condition1 = Bytes::from(vec![1, 3, 5, 7]);
        let exp_sig1 = ark_bn254::G1Affine::generator();
        let condition2 = Bytes::from(vec![2, 4, 6, 8]);
        let exp_sig2 = (ark_bn254::G1Affine::generator() * ark_bn254::Fr::from(2u64)).into_affine();

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let mock_signer = MockSigner::new(vec![condition1.clone(), condition2.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 1);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs,
            signer: mock_signer.clone(),
            algorithm: SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bn254G1,
                hash: BlsSignatureHash::Keccak256,
                compression: false,
            }),
            application_args: ApplicationArgs::Any(ApplicationAnyArgs {
                dst_suffix: "test".to_owned(),
            }),
        });

        // Spawn two background tasks that request sigs and send it back through a channel
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition1.clone();
            let tx = tx.clone();
            async move {
                let res = decryption_sender
                    .async_sign(new_decryption_request(condition))
                    .await;
                tx.send(res).await.expect("failed to send response");
            }
        });
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition2.clone();
            async move {
                let res = decryption_sender
                    .async_sign(new_decryption_request(condition))
                    .await;
                tx.send(res).await.expect("failed to send response");
            }
        });

        // Set the response for the second request
        mock_signer.set_response(&condition2, Ok(exp_sig2.ser_compressed().unwrap().into()));

        // Wait for a signature to be sent through the rx channel
        let sig2_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig2_result.is_some(), "Second signature should be resolved");
        let sig2 = sig2_result.unwrap();
        assert!(sig2.is_ok(), "Second signature should succeed");
        assert_eq!(
            sig2.unwrap().signature.into_owned(),
            exp_sig2.ser_uncompressed().unwrap(),
        );

        // Set the response for the first request
        mock_signer.set_response(&condition1, Ok(exp_sig1.ser_compressed().unwrap().into()));

        // Wait for a signature to be sent through the rx channel
        let sig1_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig1_result.is_some(), "First signature should be resolved");
        let sig1 = sig1_result.unwrap();
        assert!(sig1.is_ok(), "First signature should succeed");
        assert_eq!(
            sig1.unwrap().signature.into_owned(),
            exp_sig1.ser_uncompressed().unwrap(),
        );
    }

    #[tokio::test]
    async fn test_same_condition_concurrent_requests() {
        let global_timeout = std::time::Duration::from_millis(200);

        // Create two different conditions
        let condition = Bytes::from(vec![1, 3, 5, 7]);
        let exp_sig = ark_bn254::G1Affine::generator();

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let mock_signer = MockSigner::new(vec![condition.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 1);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs,
            signer: mock_signer.clone(),
            algorithm: SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bn254G1,
                hash: BlsSignatureHash::Keccak256,
                compression: true,
            }),
            application_args: ApplicationArgs::Any(ApplicationAnyArgs {
                dst_suffix: "test".to_owned(),
            }),
        });

        // Spawn two background tasks that request sigs and send it back through a channel
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            let tx = tx.clone();
            async move {
                let res = decryption_sender
                    .async_sign(new_decryption_request(condition))
                    .await;
                tx.send(res).await.expect("failed to send response");
            }
        });
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            async move {
                let res = decryption_sender
                    .async_sign(new_decryption_request(condition))
                    .await;
                tx.send(res).await.expect("failed to send response");
            }
        });

        // Set the response only once
        mock_signer.set_response(&condition, Ok(exp_sig.ser_compressed().unwrap().into()));

        // Wait for the first signature to be sent through the rx channel
        let sig_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig_result.is_some(), "sig should be resolved");
        let sig = sig_result.unwrap();
        assert!(sig.is_ok(), "sig should be ok");
        assert_eq!(
            sig.unwrap().signature.into_owned(),
            exp_sig.ser_uncompressed().unwrap(),
        );

        // Wait for a second signature to be sent through the rx channel
        let sig_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig_result.is_some(), "sig should be resolved");
        let sig = sig_result.unwrap();
        assert!(sig.is_ok(), "sig should be ok");
        assert_eq!(
            sig.unwrap().signature.into_owned(),
            exp_sig.ser_uncompressed().unwrap(),
        );
    }

    #[tokio::test]
    async fn test_same_condition_concurrent_requests_err() {
        let global_timeout = std::time::Duration::from_millis(200);

        // Create two different conditions
        let condition = Bytes::from(vec![1, 3, 5, 7]);

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let mock_signer = MockSigner::new(vec![condition.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 1);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs,
            signer: mock_signer.clone(),
            algorithm: SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bn254G1,
                hash: BlsSignatureHash::Keccak256,
                compression: true,
            }),
            application_args: ApplicationArgs::Any(ApplicationAnyArgs {
                dst_suffix: "test".to_owned(),
            }),
        });

        // Spawn two background tasks that request sigs and send it back through a channel
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            let tx = tx.clone();
            async move {
                let res = decryption_sender
                    .async_sign(new_decryption_request(condition))
                    .await;
                tx.send(res).await.expect("failed to send response");
            }
        });
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            async move {
                let res = decryption_sender
                    .async_sign(new_decryption_request(condition))
                    .await;
                tx.send(res).await.expect("failed to send response");
            }
        });

        // Set the response only once
        mock_signer.set_response(&condition, Err(MockSignerError));

        // Wait for the first signature to be sent through the rx channel
        let sig_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig_result.is_some(), "sig should be resolved");
        let sig = sig_result.unwrap();
        assert!(sig.is_err(), "sig should be err");

        // Wait for a second signature to be sent through the rx channel
        let sig_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig_result.is_some(), "sig should be resolved");
        let sig = sig_result.unwrap();
        assert!(sig.is_err(), "sig should be err");
    }

    #[tokio::test]
    async fn test_with_decryption_request() {
        let global_timeout = std::time::Duration::from_millis(200);

        // Create two different conditions
        let condition = Bytes::from(vec![1, 3, 5, 7]);

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let sk: Fr = MontFp!("0102030405060708091011121314151617181920");
        let mock_signer = MockSigner::new(vec![condition.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new_signer(b"TEST", 1, sk);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs: cs.clone(),
            signer: mock_signer.clone(),
            algorithm: SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bn254G1,
                hash: BlsSignatureHash::Keccak256,
                compression: true,
            }),
            application_args: ApplicationArgs::Any(ApplicationAnyArgs {
                dst_suffix: "test".to_owned(),
            }),
        });

        // Setup the request and response
        let eph_pk = ark_bn254::G2Affine::generator();
        let req = DecryptionRequest {
            id: U256::from(1u64),
            condition: condition.clone().into(),
            ciphertext: create_ciphertext(eph_pk),
        };
        let exp_sig = cs.decryption_key(cs.h1(condition.as_ref()));
        let exp_preprocessed_key = cs.preprocess_decryption_key(exp_sig, eph_pk);

        // Set the response
        mock_signer.set_response(&condition, Ok(exp_sig.ser_compressed().unwrap().into()));

        let fut_sig = decryption_sender.async_sign(req.clone());

        // Wait for the first signature to be sent through the rx channel
        let signed_req = tokio::time::timeout(global_timeout, fut_sig)
            .await
            .expect("failed to obtain signature: timed out")
            .expect("sig should be resolved");
        assert_eq!(signed_req.id, req.id);
        assert_eq!(
            signed_req.signature.into_owned(),
            exp_sig.ser_uncompressed().unwrap(),
        );
        assert_eq!(signed_req.decryption_key.as_ref(), exp_preprocessed_key);
    }
}
