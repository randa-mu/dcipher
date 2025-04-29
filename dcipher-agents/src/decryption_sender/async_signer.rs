//! [`AsynchronousSigner`] for decryption requests. Unlike [`AsyncThresholdSigner`](crate::signer::threshold_signer::AsyncThresholdSigner),
//! this signer allows to sign identical conditions as is often the case with the decryption sender contract.

use crate::decryption_sender::{DecryptionRequest, SignedDecryptionRequest};
use crate::ibe_helper::{IbeCiphertext, PairingIbeCipherSuite};
use crate::ser::EvmSerialize;
use crate::signer::AsynchronousSigner;
use alloy::primitives::Bytes;
use itertools::Either;
use lru::LruCache;
use std::borrow::Cow;
use std::num::NonZeroUsize;
use std::sync::Arc;

/// Maximum number of parallel signature requests allowed before dropping old requests.
const MAX_PARALLEL_REQUESTS: usize = 64;

pub struct DecryptionSenderAsyncSigner<CS, AsyncSigner, M>
where
    CS: PairingIbeCipherSuite,
    AsyncSigner: AsynchronousSigner<M>,
{
    cs: CS,
    signer: AsyncSigner,
    requests: tokio::sync::Mutex<
        LruCache<Bytes, Arc<tokio::sync::RwLock<Option<SignatureData<CS, AsyncSigner::Error>>>>>,
    >,
}

impl<CS, AsyncSigner, M> DecryptionSenderAsyncSigner<CS, AsyncSigner, M>
where
    CS: PairingIbeCipherSuite,
    AsyncSigner: AsynchronousSigner<M>,
{
    pub fn new(cs: CS, signer: AsyncSigner) -> Self {
        Self {
            cs,
            signer,
            requests: tokio::sync::Mutex::new(LruCache::new(
                const { NonZeroUsize::new(MAX_PARALLEL_REQUESTS).unwrap() }, // cache with 64 messages
            )),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DecryptionSenderAsyncSignerError<AsyncSignerError>
where
    AsyncSignerError: std::error::Error + Send + Sync + 'static,
{
    #[error("failed to parse decryption request into a valid ciphertext")]
    ParseCiphertext(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to request signature from async signer")]
    UnderlyingAsyncSigner(#[source] AsyncSignerError),
}

type SignatureData<CS, E> = Result<
    (
        Cow<'static, Bytes>,
        <CS as PairingIbeCipherSuite>::IdentityGroup,
    ),
    E,
>;

impl<CS, AsyncSigner> DecryptionSenderAsyncSigner<CS, AsyncSigner, Bytes>
where
    CS: PairingIbeCipherSuite + Send + Sync,
    CS::IdentityGroup: EvmSerialize,
    AsyncSigner: AsynchronousSigner<Bytes, Signature = CS::IdentityGroup> + Send + Sync,
    AsyncSigner::Error: Clone,
{
    pub async fn await_signature(
        &self,
        m: Bytes,
    ) -> SignatureData<CS, DecryptionSenderAsyncSignerError<AsyncSigner::Error>> {
        let write_or_read_lock = {
            let mut new_request = false;
            let mut requests_lock_guard = self.requests.lock().await;
            let signature_data = requests_lock_guard
                .get_or_insert(m.clone(), || {
                    // If the FnOnce is called, we insert a new entry
                    new_request = true;
                    Arc::new(tokio::sync::RwLock::new(None))
                })
                .to_owned();

            if new_request {
                // New request => we have a newly created RwLock. This future will be
                // responsible to request the signature, and write it back.
                // We still hold a lock to self.requests here, so nobody can get a hold of
                // the rw lock before us.
                let signature_data_lock = signature_data.write_owned().await;
                Either::Left(signature_data_lock)
            } else {
                // Otherwise, we create a future that will get resolved upon releasing the write lock
                Either::Right(signature_data.read_owned()) // cannot wait here - still holding another lock
            }
        };

        match write_or_read_lock {
            Either::Left(mut write_lock) => {
                // Wait for a signature from the async signer
                let sig_data = match self.signer.async_sign(m.clone()).await {
                    Ok(sig) => {
                        let sig_bytes = Cow::Owned(EvmSerialize::ser_bytes(&sig));
                        Ok((sig_bytes, sig))
                    }
                    Err(e) => Err(e),
                };

                // Save the data, and drop write lock
                //  => read lock will be acquired by all awaiting futures
                *write_lock = Some(sig_data.clone());
                sig_data
            }
            Either::Right(read_lock) => read_lock
                // await the release of the write lock
                .await
                .as_ref()
                .cloned()
                .expect("ready lock should only resolve when signature data is some"),
        }
        .map_err(|e| DecryptionSenderAsyncSignerError::UnderlyingAsyncSigner(e))
    }
}

impl<CS, AsyncSigner> AsynchronousSigner<DecryptionRequest>
    for DecryptionSenderAsyncSigner<CS, AsyncSigner, Bytes>
where
    CS: PairingIbeCipherSuite + Send + Sync,
    CS::IdentityGroup: EvmSerialize,
    AsyncSigner: AsynchronousSigner<Bytes, Signature = CS::IdentityGroup> + Send + Sync,
    AsyncSigner::Error: Clone,
    DecryptionRequest: TryInto<CS::Ciphertext>,
    <DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    type Error = DecryptionSenderAsyncSignerError<AsyncSigner::Error>;
    type Signature = SignedDecryptionRequest<'static>;

    fn async_sign(
        &self,
        req: DecryptionRequest,
    ) -> impl Future<Output = Result<Self::Signature, Self::Error>> + Send {
        async move {
            // Await signature
            let (sig_bytes, sig) = self.await_signature(req.condition.clone()).await?;

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
                sig_bytes.clone(),
            ))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::decryption_sender::DecryptionRequest;
    use crate::ibe_helper::{IbeIdentityOnBn254G1Suite, PairingIbeCipherSuite};
    use crate::ser::EvmSerialize;
    use crate::ser::tests::bn254::encode_ciphertext;
    use crate::signer::{AsynchronousSigner, BlsSigner};
    use alloy::primitives::{Bytes, U256};
    use ark_bn254::Fr;
    use ark_ec::{AffineRepr, CurveGroup};
    use ark_ff::{BigInteger, MontFp, PrimeField};
    use lru::LruCache;
    use std::collections::HashMap;
    use std::num::NonZeroUsize;
    use std::sync::Arc;
    use tokio::sync::oneshot;

    pub(crate) fn create_ciphertext(eph_pk: ark_bn254::G2Affine) -> Bytes {
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

    #[derive(Clone)]
    struct MockAsyncSigner {
        receivers: Arc<
            tokio::sync::Mutex<
                HashMap<Bytes, oneshot::Receiver<Result<ark_bn254::G1Affine, MockSignerError>>>,
            >,
        >,
        senders: Arc<
            std::sync::Mutex<
                HashMap<Bytes, oneshot::Sender<Result<ark_bn254::G1Affine, MockSignerError>>>,
            >,
        >,
    }

    impl MockAsyncSigner {
        fn new(conditions: impl IntoIterator<Item = Bytes>) -> Self {
            let (txs, rxs): (Vec<_>, Vec<_>) = conditions
                .into_iter()
                .map(|c| {
                    let (tx, rx) = oneshot::channel();
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
        fn set_response(
            &self,
            condition: &Bytes,
            result: Result<ark_bn254::G1Affine, MockSignerError>,
        ) {
            let tx = self
                .senders
                .lock()
                .expect("task holding mutex panicked")
                .remove(condition)
                .expect("condition not found");
            tx.send(result).expect("failed to set response");
        }
    }

    impl AsynchronousSigner<Bytes> for MockAsyncSigner {
        type Error = MockSignerError;
        type Signature = ark_bn254::G1Affine;

        fn async_sign(
            &self,
            m: Bytes,
        ) -> impl Future<Output = Result<Self::Signature, Self::Error>> + Send {
            async move {
                let rx = self
                    .receivers
                    .lock()
                    .await
                    .remove(&m)
                    .expect("no receiver found for condition");
                rx.await.unwrap_or(Err(MockSignerError))
            }
        }
    }

    #[tokio::test]
    async fn test_different_conditions_concurrent_requests() {
        let global_timeout = std::time::Duration::from_millis(200);

        // Create two different conditions
        let condition1 = Bytes::from(vec![1, 3, 5, 7]);
        let exp_sig1 = ark_bn254::G1Affine::generator();
        let condition2 = Bytes::from(vec![2, 4, 6, 8]);
        let exp_sig2 = (ark_bn254::G1Affine::generator() * ark_bn254::Fr::from(2u64)).into_affine();

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let mock_signer = MockAsyncSigner::new(vec![condition1.clone(), condition2.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 1);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs,
            signer: mock_signer.clone(),
            requests: tokio::sync::Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())),
        });

        // Spawn two background tasks that request sigs and send it back through a channel
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition1.clone();
            let tx = tx.clone();
            async move {
                let res = decryption_sender.await_signature(condition).await;
                tx.send(res).await.expect("failed to send response");
            }
        });
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition2.clone();
            async move {
                let res = decryption_sender.await_signature(condition).await;
                tx.send(res).await.expect("failed to send response");
            }
        });

        // Set the response for the second request
        mock_signer.set_response(&condition2, Ok(exp_sig2));

        // Wait for a signature to be sent through the rx channel
        let sig2_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig2_result.is_some(), "Second signature should be resolved");
        let sig2 = sig2_result.unwrap();
        assert!(sig2.is_ok(), "Second signature should succeed");
        assert_eq!(sig2.unwrap().1, exp_sig2);

        // Set the response for the first request
        mock_signer.set_response(&condition1, Ok(exp_sig1));

        // Wait for a signature to be sent through the rx channel
        let sig1_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig1_result.is_some(), "First signature should be resolved");
        let sig1 = sig1_result.unwrap();
        assert!(sig1.is_ok(), "First signature should succeed");
        assert_eq!(sig1.unwrap().1, exp_sig1);
    }

    #[tokio::test]
    async fn test_same_condition_concurrent_requests() {
        let global_timeout = std::time::Duration::from_millis(200);

        // Create two different conditions
        let condition = Bytes::from(vec![1, 3, 5, 7]);
        let exp_sig = ark_bn254::G1Affine::generator();

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let mock_signer = MockAsyncSigner::new(vec![condition.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 1);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs,
            signer: mock_signer.clone(),
            requests: tokio::sync::Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())),
        });

        // Spawn two background tasks that request sigs and send it back through a channel
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            let tx = tx.clone();
            async move {
                let res = decryption_sender.await_signature(condition).await;
                tx.send(res).await.expect("failed to send response");
            }
        });
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            async move {
                let res = decryption_sender.await_signature(condition).await;
                tx.send(res).await.expect("failed to send response");
            }
        });

        // Set the response only once
        mock_signer.set_response(&condition, Ok(exp_sig));

        // Wait for the first signature to be sent through the rx channel
        let sig_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig_result.is_some(), "sig should be resolved");
        let sig = sig_result.unwrap();
        assert!(sig.is_ok(), "sig should be ok");
        assert_eq!(sig.unwrap().1, exp_sig);

        // Wait for a second signature to be sent through the rx channel
        let sig_result = tokio::time::timeout(global_timeout, rx.recv())
            .await
            .expect("failed to obtain signature: timed out");
        assert!(sig_result.is_some(), "sig should be resolved");
        let sig = sig_result.unwrap();
        assert!(sig.is_ok(), "sig should be ok");
        assert_eq!(sig.unwrap().1, exp_sig);
    }

    #[tokio::test]
    async fn test_same_condition_concurrent_requests_err() {
        let global_timeout = std::time::Duration::from_millis(200);

        // Create two different conditions
        let condition = Bytes::from(vec![1, 3, 5, 7]);

        // Set up the mock signer and DecryptionSenderAsyncSigner
        let mock_signer = MockAsyncSigner::new(vec![condition.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 1);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs,
            signer: mock_signer.clone(),
            requests: tokio::sync::Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())),
        });

        // Spawn two background tasks that request sigs and send it back through a channel
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            let tx = tx.clone();
            async move {
                let res = decryption_sender.await_signature(condition).await;
                tx.send(res).await.expect("failed to send response");
            }
        });
        tokio::task::spawn({
            let decryption_sender = decryption_sender.clone();
            let condition = condition.clone();
            async move {
                let res = decryption_sender.await_signature(condition).await;
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
        let mock_signer = MockAsyncSigner::new(vec![condition.clone()]);
        let cs = IbeIdentityOnBn254G1Suite::new_signer(b"TEST", 1, sk);
        let decryption_sender = Arc::new(DecryptionSenderAsyncSigner {
            cs: cs.clone(),
            signer: mock_signer.clone(),
            requests: tokio::sync::Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())),
        });

        // Setup the request and response
        let eph_pk = ark_bn254::G2Affine::generator();
        let req = DecryptionRequest {
            id: U256::from(1u64),
            condition: condition.clone(),
            ciphertext: create_ciphertext(eph_pk),
        };
        let exp_sig = cs.sign(&condition).unwrap();
        let exp_preprocessed_key = cs.preprocess_decryption_key(exp_sig, eph_pk);

        // Set the response
        mock_signer.set_response(&condition, Ok(exp_sig));

        let fut_sig = decryption_sender.async_sign(req.clone());

        // Wait for the first signature to be sent through the rx channel
        let signed_req = tokio::time::timeout(global_timeout, fut_sig)
            .await
            .expect("failed to obtain signature: timed out")
            .expect("sig should be resolved");
        assert_eq!(signed_req.id, req.id);
        assert_eq!(
            signed_req.signature.into_owned(),
            EvmSerialize::ser_bytes(&exp_sig)
        );
        assert_eq!(signed_req.decryption_key.as_ref(), exp_preprocessed_key);
    }
}
