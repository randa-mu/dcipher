//! Concrete implementation of a [`AsynchronousSigner`] for a (t, n) threshold network of
//! participants.

mod aggregation;
mod libp2p;
pub mod metrics;

pub use aggregation::lagrange_points_interpolate_at;

use crate::ser::EvmSerialize;
use crate::signer::threshold_signer::libp2p::LibP2PNode;
use crate::signer::threshold_signer::metrics::Metrics;
use crate::signer::{AsynchronousSigner, BlsSigner, BlsVerifier};
use ark_ec::{AffineRepr, CurveGroup};
use itertools::Either;
use lru::LruCache;
use pairing_utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

type SignatureGroup<BLS> = <BLS as BlsVerifier>::SignatureGroup;

type SignatureOrChannel<BLS> =
    Either<SignatureGroup<BLS>, tokio::sync::watch::Sender<Option<SignatureGroup<BLS>>>>;

type SharedSignatureCache<BLS> = Arc<std::sync::Mutex<LruCache<Vec<u8>, SignatureOrChannel<BLS>>>>;

type SharedPartialsCache<BLS> =
    Arc<std::sync::Mutex<LruCache<Vec<u8>, HashMap<u16, PartialSignature<SignatureGroup<BLS>>>>>>;

pub struct AsyncThresholdSigner<BLS>
where
    BLS: BlsSigner,
{
    signatures_cache: SharedSignatureCache<BLS>,
    new_message_to_sign: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct PartialSignature<G> {
    id: u16,
    sig: G,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "G: PointSerializeCompressed",
    deserialize = "G: PointDeserializeCompressed"
))]
struct PartialSignatureWithMessage<G> {
    m: Vec<u8>,
    #[serde(with = "pairing_utils::serialize::point::base64")]
    sig: G,
}

/// Threshold signer that relies on libp2p to exchange partial signatures.
pub struct ThresholdSigner<BLS>
where
    BLS: BlsSigner,
{
    // Signatures cache
    signatures_cache: SharedSignatureCache<BLS>,

    // Map from messages to partials
    partials_cache: SharedPartialsCache<BLS>,

    // Ciphersuite + Signer
    signer: BLS,

    // Threshold parameters
    n: u16,
    t: u16,
    id: u16,
    pks: Vec<BLS::PublicKeyGroup>,

    // Enable the node to broadcast a partial signature upon receiving a valid partial.
    eager_signing: bool,
}

impl<BLS> ThresholdSigner<BLS>
where
    BLS: BlsSigner + Clone + Send + Sync + 'static,
    SignatureGroup<BLS>: EvmSerialize + PointSerializeCompressed + PointDeserializeCompressed,
{
    /// Create a new threshold signer by specifying the various threshold scheme parameters.
    pub fn new(cs: BLS, n: u16, t: u16, id: u16, pks: Vec<BLS::PublicKeyGroup>) -> Self {
        Self {
            signatures_cache: Arc::new(std::sync::Mutex::new(LruCache::new(
                const { NonZeroUsize::new(64).unwrap() }, // cache with 64 messages
            ))),
            partials_cache: Arc::new(std::sync::Mutex::new(LruCache::new(
                const { NonZeroUsize::new(64).unwrap() }, // cache with 64 messages
            ))),
            signer: cs,
            n,
            t,
            id,
            pks,
            // disable eager signing by default, i.e., automatically submitting a partial
            // signature upon receiving a valid partial from another node.
            eager_signing: false,
        }
    }

    /// New threshold signer with a custom LRU cache size.
    pub fn new_with_cache_size(
        cs: BLS,
        n: u16,
        t: u16,
        id: u16,
        pks: Vec<BLS::PublicKeyGroup>,
        lru_cache_size: NonZeroUsize,
    ) -> Self {
        Self {
            signatures_cache: Arc::new(std::sync::Mutex::new(LruCache::new(lru_cache_size))),
            partials_cache: Arc::new(std::sync::Mutex::new(LruCache::new(lru_cache_size))),
            signer: cs,
            n,
            t,
            id,
            pks,
            eager_signing: false,
        }
    }

    /// Enable eager signing by automatically submitting a partial signature upon receiving
    /// a valid partial from another node.
    pub fn with_eager_signing(mut self) -> Self {
        self.eager_signing = true;
        self
    }

    /// Runs the threshold signer in a background task and obtain a cancellation token and a registry.
    pub fn run(
        self,
        libp2p_keypair: ::libp2p::identity::Keypair,
        libp2p_listenaddr: ::libp2p::Multiaddr,
        libp2p_peer_addresses: Vec<::libp2p::Multiaddr>,
        libp2p_peer_ids: Vec<::libp2p::PeerId>,
        short_peer_ids: Vec<u16>,
    ) -> (CancellationToken, AsyncThresholdSigner<BLS>) {
        if libp2p_peer_addresses.len() != usize::from(self.n - 1)
            || libp2p_peer_ids.len() != usize::from(self.n - 1)
            || short_peer_ids.len() != usize::from(self.n - 1)
        {
            panic!("run requires all inputs array to be of length n - 1");
        }

        let arc_self = Arc::new(self);
        let cancellation_token = CancellationToken::new();
        let (tx_registry_to_signer, rx_signer_to_registry) = tokio::sync::mpsc::unbounded_channel();
        let (tx_signer_to_libp2p, rx_libp2p_from_signer) = tokio::sync::mpsc::unbounded_channel();
        let (tx_libp2p_to_signer, rx_signer_from_libp2p) = tokio::sync::mpsc::unbounded_channel();

        // Create a registry
        let registry = AsyncThresholdSigner {
            signatures_cache: arc_self.signatures_cache.clone(),
            new_message_to_sign: tx_registry_to_signer.clone(),
        };

        // Create a libp2p instance
        let libp2p = LibP2PNode::new(
            libp2p_keypair,
            libp2p_peer_addresses,
            libp2p_peer_ids,
            short_peer_ids,
        );

        // Run the libp2p instance in a new task
        libp2p
            .run(
                libp2p_listenaddr,
                tx_libp2p_to_signer,
                rx_libp2p_from_signer,
                cancellation_token.child_token(),
            )
            .expect("failed to run libp2p node");

        // Spawn task that handles signing requests from registry
        tokio::task::spawn(arc_self.clone().recv_new_messages(
            rx_signer_to_registry,
            tx_signer_to_libp2p,
            cancellation_token.child_token(),
        ));

        // Spawn task that handles messages from other nodes
        tokio::task::spawn(arc_self.clone().recv_new_signatures(
            rx_signer_from_libp2p,
            tx_registry_to_signer,
            cancellation_token.child_token(),
        ));

        (cancellation_token, registry)
    }

    async fn recv_new_messages(
        self: Arc<Self>,
        mut rx_messages: tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>,
        tx_to_libp2p: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
        cancellation_token: CancellationToken,
    ) {
        #[cfg(feature = "rayon")]
        use rayon::prelude::*;

        const MAX_BATCH_SIZE: usize = 256;

        let inner_fn = async move {
            let mut messages = Vec::with_capacity(MAX_BATCH_SIZE);

            loop {
                let count = rx_messages.recv_many(&mut messages, MAX_BATCH_SIZE).await;
                if count == 0 {
                    tracing::warn!("Registry has dropped message sender, exiting recv loop");
                    break;
                };

                // Remove messages with partial already issued
                let messages: Vec<_> = {
                    let mut partials_cache = self
                        .partials_cache
                        .lock()
                        .expect("a thread panicked holding the mutex");

                    messages
                        .drain(..)
                        .filter(|m| {
                            let Some(partials_map) = partials_cache.get(m) else {
                                return true; // not yet signed
                            };

                            if partials_map.contains_key(&self.id) {
                                tracing::debug!(msg = ?m, "Received message signing request, but message was already signed");
                                false
                            } else {
                                true
                            }
                        })
                        .collect()
                };

                let span =
                    tracing::debug_span!("threshold_signer_batch", batch_size = count).entered();
                #[cfg(feature = "rayon")]
                tracing::debug!(
                    messages_count = messages.len(),
                    "Signing messages in parallel"
                );
                #[cfg(not(feature = "rayon"))]
                tracing::debug!(
                    messages_count = messages.len(),
                    "Signing messages sequentially"
                );
                let span = span.exit();

                // Create signatures in parallel if rayon is enabled, otherwise use a standard iter
                #[cfg(feature = "rayon")]
                let iter = messages.into_par_iter();
                #[cfg(not(feature = "rayon"))]
                let iter = messages.into_iter();
                let (partials, messages): (Vec<_>, Vec<_>) = iter.filter_map(|message| {
                    tracing::info!(msg = ?message, "Received new message to sign");

                    match self.signer.sign(&message) {
                        Ok(sig) => Some((sig, message)),
                        Err(e) => {
                            tracing::error!(error = ?e, msg = ?message, "Failed to sign message.");
                            None
                        }
                    }
                }).collect();

                let to_aggregate: Vec<_> = {
                    let mut partials_cache = self
                        .partials_cache
                        .lock()
                        .expect("a thread panicked with the mutex");

                    // We filter with a sequential iterator here due to side effects
                    partials.iter().zip(messages.iter()).filter_map(|(partial_sig, m)| {
                        tracing::info!(msg = ?m, party_id = self.id, "Storing partial signature on message");
                        let partials = partials_cache.get_or_insert_mut(m.clone(), HashMap::default);
                        partials.insert(
                            self.id,
                            PartialSignature {
                                id: self.id,
                                sig: *partial_sig,
                            },
                        );

                        // Do we have exactly t partials?
                        if partials.len() == usize::from(self.t) {
                            // Aggregate the partials with Lagrange's interpolation
                            let points = partials
                                .values()
                                .map(|partial| (u64::from(partial.id), partial.sig.into_group()))
                                .collect::<Vec<_>>();
                            Some(points)
                        } else {
                            None
                        }
                    }).collect()
                };

                let span = span.entered();
                #[cfg(feature = "rayon")]
                tracing::debug!(
                    messages_count = messages.len(),
                    "Aggregating signatures in parallel"
                );
                #[cfg(not(feature = "rayon"))]
                tracing::debug!(
                    messages_count = messages.len(),
                    "Aggregating signatures sequentially"
                );
                let _span = span.exit();

                // Do the aggregation with a parallel iterator if rayon is enabled
                #[cfg(feature = "rayon")]
                let iter = to_aggregate.into_par_iter();
                #[cfg(not(feature = "rayon"))]
                let iter = to_aggregate.into_iter();
                let signatures: Vec<_> = iter
                    .map(|points| lagrange_points_interpolate_at(&points, 0).into_affine())
                    .collect();

                // We now have a bunch of signatures, store them
                {
                    let mut signatures_cache = self
                        .signatures_cache
                        .lock()
                        .expect("a thread panicked with the mutex");

                    // side effects, sequential iterator
                    signatures
                        .into_iter()
                        .zip(messages.iter())
                        .for_each(|(sig, message)| {
                            if let Some(Either::Right(tx_channel)) =
                                signatures_cache.put(message.to_owned(), Either::Left(sig))
                            {
                                // If there previously was a channel stored at the entry, also send signature through it
                                tx_channel.send_replace(Some(sig));
                            }
                        });
                }

                // Send it to other nodes with libp2p if threshold greater than 1
                if self.t > 1 {
                    partials
                        .into_iter()
                        .zip(messages)
                        .for_each(|(sig, message)| {
                            let partial = PartialSignatureWithMessage { sig, m: message };

                            let m = serde_cbor::to_vec(&partial)
                                .expect("serialization should always work");
                            Metrics::report_partials_sent(1);
                            if tx_to_libp2p.send(m).is_err() {
                                tracing::error!("Failed to send message to signer: channel closed");
                            }
                        });
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping recv loop due to cancellation token");
            },

            _ = inner_fn => (),
        }
    }

    async fn recv_new_signatures(
        self: Arc<Self>,
        mut rx_from_libp2p: tokio::sync::mpsc::UnboundedReceiver<(u16, Vec<u8>)>,
        new_message_to_sign: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
        cancellation_token: CancellationToken,
    ) {
        let inner_fn = async move {
            loop {
                let Some((party_id, partial)) = rx_from_libp2p.recv().await else {
                    tracing::warn!("Libp2p node has dropped sender, exiting recv loop");
                    break;
                };

                let partial: PartialSignatureWithMessage<SignatureGroup<BLS>> =
                    match serde_cbor::from_slice(&partial) {
                        Ok(partial) => partial,
                        Err(e) => {
                            tracing::error!(
                                sender_id = party_id,
                                error = ?e,
                                "Failed to decode partial signature."
                            );
                            continue;
                        }
                    };

                Metrics::report_partials_received(1);

                // Verify the validity of the partial signature against its pk
                let Some(pk) = self.pks.get(usize::from(party_id) - 1) else {
                    tracing::error!(sender_id = party_id, "Invalid party_id / pks vector");
                    continue;
                };
                if !self.signer.verify(&partial.m, partial.sig, *pk) {
                    tracing::error!(sender_id = party_id, "Received invalid partial signature");
                    Metrics::report_invalid_partials(1);
                    continue;
                }

                // Valid signature, add it to our cache
                self.store_and_process_partial(
                    partial.m.clone(),
                    PartialSignature {
                        id: party_id,
                        sig: partial.sig,
                    },
                );

                if self.eager_signing {
                    // If eager signing is enabled and the message has not been signed already,
                    // request to broadcast a partial signature on that message
                    if !self.partial_issued(&partial.m) {
                        new_message_to_sign
                            .send(partial.m)
                            .expect("failed to forward message to signer");
                    }
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping recv loop due to cancellation token");
            },

            _ = inner_fn => (),
        }
    }

    /// Verify whether a partial has already been issued or not.
    fn partial_issued(&self, message: &[u8]) -> bool {
        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked holding the mutex");
        let Some(partials_map) = partials_cache.get(message) else {
            return false;
        };

        partials_map.contains_key(&self.id)
    }

    /// Store a partial signature to the cache, and aggregate it if there are enough partials.
    fn store_and_process_partial(
        &self,
        message: Vec<u8>,
        partial: PartialSignature<SignatureGroup<BLS>>,
    ) {
        tracing::info!(msg = ?message, party_id = partial.id, "Storing partial signature on message");
        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked with the mutex");
        let partials = partials_cache.get_or_insert_mut(message.clone(), HashMap::default);
        partials.insert(
            partial.id,
            PartialSignature {
                id: partial.id,
                sig: partial.sig,
            },
        );

        // Do we have exactly t partials?
        if partials.len() == usize::from(self.t) {
            // Aggregate the partials with Lagrange's interpolation
            let points = partials
                .values()
                .map(|partial| (u64::from(partial.id), partial.sig.into_group()))
                .collect::<Vec<_>>();
            let sig = lagrange_points_interpolate_at(&points, 0).into_affine();

            // We now have a signature, store it
            let mut signatures_cache = self
                .signatures_cache
                .lock()
                .expect("a thread panicked with the mutex");
            if let Some(Either::Right(tx_channel)) =
                signatures_cache.put(message, Either::Left(sig))
            {
                // If there previously was a channel stored at the entry, also send signature through it
                tx_channel.send_replace(Some(sig));
            }
        }
    }
}

#[derive(thiserror::Error, Copy, Clone, Debug)]
pub enum AsyncThresholdSignerError {
    #[error("the message to sign has been dropped from cache")]
    DroppedFromCache,

    #[error("the watch sender has been dropped")]
    WatchSenderDropped,

    #[error("the channel used to request signatures has been closed")]
    CannotRequestNewSignatures,
}

impl<BLS, M> AsynchronousSigner<M> for AsyncThresholdSigner<BLS>
where
    BLS: BlsSigner + Send + Sync,
    M: AsRef<[u8]>,
    SignatureGroup<BLS>: EvmSerialize,
    for<'a> &'a SignatureGroup<BLS>: ToOwned,
{
    type Error = AsyncThresholdSignerError;
    type Signature = SignatureGroup<BLS>;

    fn async_sign(
        &self,
        m: M,
    ) -> impl Future<Output = Result<Self::Signature, Self::Error>> + Send {
        let m = m.as_ref().to_vec();
        async move {
            // We have three possibilities here:
            //  1. The message is not yet present in the map
            //      => a. insert a watch sender in the map,
            //         b. we notify of a new message,
            //         c. return a future awaiting the signature through the watch receiver.
            //  2. The message is in the map
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
                let signature_or_sender = signatures_cache.get_or_insert(m.clone(), || {
                    let (tx, _) = tokio::sync::watch::channel(None);
                    Either::Right(tx)
                });

                match signature_or_sender {
                    Either::Left(signature) => {
                        // 2a. The message is in the map and contains a signature
                        Ok(Either::Left(signature.to_owned()))
                    }

                    Either::Right(tx) => {
                        let rx = tx.subscribe();

                        // Notify of the new message to sign
                        self.new_message_to_sign
                            .send(m.to_vec())
                            .map_err(|_| AsyncThresholdSignerError::CannotRequestNewSignatures)?;

                        Ok(Either::Right(rx))
                    }
                }
            }?;

            // If the signature was cached, return immediately
            match signature_or_receiver {
                Either::Left(signature) => Ok(signature),
                Either::Right(mut rx) => {
                    // A signature may already be in the channel, borrow it and mark it as seen
                    let signature = *rx.borrow_and_update();

                    if let Some(sig) = signature {
                        // If it contains a signature, simply return
                        Ok(sig)
                    } else {
                        // Does not yet contain a signature, await for a change and return
                        match rx.changed().await {
                            Ok(()) => {
                                let sig = rx
                                    .borrow_and_update()
                                    .expect("watch channel updated but sig is None");
                                Ok(sig)
                            }
                            Err(_) => Err(AsyncThresholdSignerError::WatchSenderDropped)?,
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibe_helper::IbeIdentityOnBn254G1Suite;
    use ark_bn254::Fr;
    use ark_ff::MontFp;
    use std::time::Duration;

    #[tokio::test]
    async fn libp2p_async_threshold_signer() {
        let n = 3;
        let t = 2;
        let g2 = ark_bn254::G2Affine::generator();

        let _sk: Fr =
            MontFp!("7685086713915354683875500702831995067084988389812060097318430034144315778947");
        let sk1: Fr =
            MontFp!("5840327440053394277204603653048962762290958051681898697354171413163183818203");
        let sk2: Fr =
            MontFp!("3995568166191433870533706603265930457496927713551737297389912792182051857459");
        let sk3: Fr =
            MontFp!("2150808892329473463862809553482898152702897375421575897425654171200919896715");
        let pks = vec![g2 * sk1, g2 * sk2, g2 * sk3];
        let pks = pks
            .into_iter()
            .map(|pki| pki.into_affine())
            .collect::<Vec<_>>();

        let cs1 = IbeIdentityOnBn254G1Suite::new_signer(b"TEST", 31337, sk1);
        let cs2 = IbeIdentityOnBn254G1Suite::new_signer(b"TEST", 31337, sk2);
        let cs3 = IbeIdentityOnBn254G1Suite::new_signer(b"TEST", 31337, sk3);

        let libp2p_sk1 = ::libp2p::identity::Keypair::generate_ed25519();
        let libp2p_sk2 = ::libp2p::identity::Keypair::generate_ed25519();
        let libp2p_sk3 = ::libp2p::identity::Keypair::generate_ed25519();

        let ts1 = ThresholdSigner::new(cs1, n, t, 1, pks.clone());
        let ts2 = ThresholdSigner::new(cs2, n, t, 2, pks.clone());
        let ts3 = ThresholdSigner::new(cs3, n, t, 3, pks.clone());

        let addr_1: ::libp2p::Multiaddr = "/ip4/127.0.0.1/tcp/32140".parse().unwrap();
        let addr_2: ::libp2p::Multiaddr = "/ip4/127.0.0.1/tcp/32141".parse().unwrap();
        let addr_3: ::libp2p::Multiaddr = "/ip4/127.0.0.1/tcp/32142".parse().unwrap();

        // Start three threshold signers
        let (_, ch1) = ts1.run(
            libp2p_sk1.clone(),
            addr_1.clone(),
            vec![addr_2.clone(), addr_3.clone()],
            vec![
                libp2p_sk2.public().to_peer_id(),
                libp2p_sk3.public().to_peer_id(),
            ],
            vec![2, 3],
        );
        let (_, ch2) = ts2.run(
            libp2p_sk2.clone(),
            addr_2.clone(),
            vec![addr_1.clone(), addr_3.clone()],
            vec![
                libp2p_sk1.public().to_peer_id(),
                libp2p_sk3.public().to_peer_id(),
            ],
            vec![1, 3],
        );
        let (_, ch3) = ts3.run(
            libp2p_sk3.clone(),
            addr_3,
            vec![addr_1, addr_2],
            vec![
                libp2p_sk1.public().to_peer_id(),
                libp2p_sk2.public().to_peer_id(),
            ],
            vec![1, 2],
        );

        let message = b"my test message";
        let fut_sig1 = ch1.async_sign(message.to_vec());
        let fut_sig2 = ch2.async_sign(message.to_vec());
        let fut_sig3 = ch3.async_sign(message.to_vec());

        // Wait for signatures up to 1 second
        let sigs = tokio::select! {
            sigs = futures_util::future::join_all([fut_sig1, fut_sig2, fut_sig3]) => {
                sigs
            }

            _ = tokio::time::sleep(Duration::from_millis(1000)) => {
                panic!("failed to obtain threshold signatures after waiting 1000ms");
            }
        };

        assert_eq!(sigs.len(), 3);
        assert!(sigs[0].is_ok());
        assert!(sigs[1].is_ok());
        assert!(sigs[2].is_ok());
    }
}
