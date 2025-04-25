//! Concrete implementation of a [`RequestSigningRegistry`] for a (t, n) threshold network of
//! participants.

mod aggregation;
mod libp2p;

use crate::decryption_sender::{DecryptionRequest, SignedDecryptionRequest};
use crate::ibe_helper::{IbeCipherSuite, IbeCiphertext};
use crate::ser::EvmSerialize;
use crate::signer::RequestSigningRegistry;
use crate::signer::threshold_signer::aggregation::lagrange_points_interpolate_at;
use crate::signer::threshold_signer::libp2p::LibP2PNode;
use alloy::primitives::Bytes;
use ark_ec::{AffineRepr, CurveGroup};
use lru::LruCache;
use pairing_utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

type SharedSignatureCache<CS> = Arc<
    std::sync::Mutex<
        LruCache<Vec<u8>, (<CS as IbeCipherSuite>::IdentityGroup, Cow<'static, Bytes>)>,
    >,
>;

type SharedPartialsCache<CS> = Arc<
    std::sync::Mutex<
        LruCache<Vec<u8>, HashMap<u16, PartialSignature<<CS as IbeCipherSuite>::IdentityGroup>>>,
    >,
>;

pub struct Registry<CS>
where
    CS: IbeCipherSuite,
{
    cs: CS,
    signatures_cache: SharedSignatureCache<CS>,
    new_condition: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
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
pub struct ThresholdSigner<CS>
where
    CS: IbeCipherSuite,
{
    // Signatures cache
    signatures_cache: SharedSignatureCache<CS>,

    // Map from conditions to partials
    partials_cache: SharedPartialsCache<CS>,

    // Ciphersuite and secret key
    cs: CS,
    sk: <CS::IdentityGroup as AffineRepr>::ScalarField,

    // Threshold parameters
    n: u16,
    t: u16,
    id: u16,
    pks: Vec<CS::PublicKeyGroup>,
}

impl<CS> ThresholdSigner<CS>
where
    CS: IbeCipherSuite + Clone + Send + Sync + 'static,
    CS::IdentityGroup: EvmSerialize + PointSerializeCompressed + PointDeserializeCompressed,
    Registry<CS>: RequestSigningRegistry,
{
    /// Create a new threshold signer by specifying the various threshold scheme parameters.
    pub fn new(
        cs: CS,
        sk: <CS::IdentityGroup as AffineRepr>::ScalarField,
        n: u16,
        t: u16,
        id: u16,
        pks: Vec<CS::PublicKeyGroup>,
    ) -> Self {
        Self {
            signatures_cache: Arc::new(std::sync::Mutex::new(LruCache::new(
                const { NonZeroUsize::new(64).unwrap() }, // cache with 64 conditions
            ))),
            partials_cache: Arc::new(std::sync::Mutex::new(LruCache::new(
                const { NonZeroUsize::new(64).unwrap() }, // cache with 64 conditions
            ))),
            cs,
            sk,
            n,
            t,
            id,
            pks,
        }
    }

    /// Runs the threshold signer in a background task and obtain a cancellation token and a registry.
    pub fn run(
        self,
        libp2p_keypair: ::libp2p::identity::Keypair,
        libp2p_listenaddr: ::libp2p::Multiaddr,
        libp2p_peer_addresses: Vec<::libp2p::Multiaddr>,
        libp2p_peer_ids: Vec<::libp2p::PeerId>,
        short_peer_ids: Vec<u16>,
    ) -> (CancellationToken, Registry<CS>) {
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
        let registry = Registry {
            cs: arc_self.cs.clone(),
            signatures_cache: arc_self.signatures_cache.clone(),
            new_condition: tx_registry_to_signer,
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
        tokio::task::spawn(arc_self.clone().recv_new_conditions(
            rx_signer_to_registry,
            tx_signer_to_libp2p,
            cancellation_token.child_token(),
        ));

        // Spawn task that handles messages from other nodes
        tokio::task::spawn(
            arc_self
                .clone()
                .recv_new_signatures(rx_signer_from_libp2p, cancellation_token.child_token()),
        );

        (cancellation_token, registry)
    }

    async fn recv_new_conditions(
        self: Arc<Self>,
        mut rx_conditions: tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>,
        tx_to_libp2p: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
        cancellation_token: CancellationToken,
    ) {
        let inner_fn = async move {
            loop {
                let Some(condition) = rx_conditions.recv().await else {
                    tracing::warn!("Registry has dropped condition sender, exiting recv loop");
                    break;
                };

                // If a partial was already issued, ignore the condition
                if self.partial_issued(&condition) {
                    tracing::debug!(condition = ?condition, "Received condition signing request, but request was already signed");
                    continue;
                }

                tracing::info!(condition = ?condition, "Received new condition to sign");
                let condition_identity = self.cs.h1(&condition);
                let sig = self.cs.decryption_key(&self.sk, condition_identity);
                let partial = PartialSignatureWithMessage {
                    sig,
                    m: condition.clone(),
                };

                // Save the signature, and aggregate it if we have enough signatures
                self.store_and_process_partial(condition, PartialSignature { id: self.id, sig });

                // Send it to other nodes with libp2p
                let m = serde_cbor::to_vec(&partial).expect("serialization should always work");
                if tx_to_libp2p.send(m).is_err() {
                    tracing::error!("Failed to send condition to signer: channel closed");
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
        cancellation_token: CancellationToken,
    ) {
        let inner_fn = async move {
            loop {
                let Some((party_id, partial)) = rx_from_libp2p.recv().await else {
                    tracing::warn!("Libp2p node has dropped sender, exiting recv loop");
                    break;
                };

                let partial: PartialSignatureWithMessage<CS::IdentityGroup> =
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

                // Verify the validity of the partial signature against its pk
                let Some(pk) = self.pks.get(usize::from(party_id) - 1) else {
                    tracing::error!(sender_id = party_id, "Invalid party_id / pks vector");
                    continue;
                };
                if !self.cs.verify_decryption_key(&partial.m, partial.sig, *pk) {
                    tracing::error!(sender_id = party_id, "Received invalid partial signature");
                    continue;
                }

                // Valid signature, add it to our cache
                self.store_and_process_partial(
                    partial.m,
                    PartialSignature {
                        id: party_id,
                        sig: partial.sig,
                    },
                );
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
    fn partial_issued(&self, condition: &[u8]) -> bool {
        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked holding the mutex");
        let Some(partials_map) = partials_cache.get(condition) else {
            return false;
        };

        partials_map.contains_key(&self.id)
    }

    /// Store a partial signature to the cache, and aggregate it if there are enough partials.
    fn store_and_process_partial(
        &self,
        condition: Vec<u8>,
        partial: PartialSignature<CS::IdentityGroup>,
    ) {
        tracing::info!(condition = ?condition, party_id = partial.id, "Storing partial signature on condition");
        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked with the mutex");
        let partials = partials_cache.get_or_insert_mut(condition.clone(), HashMap::default);
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
            let sig_bytes = Cow::Owned(EvmSerialize::ser_bytes(&sig));

            // We now have a signature, store it
            self.signatures_cache
                .lock()
                .expect("a thread panicked with the mutex")
                .push(condition, (sig, sig_bytes));
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("cannot parse the ciphertext")]
    CannotParseCiphertext,
}

impl<CS> Registry<CS>
where
    CS: IbeCipherSuite,
    for<'a> &'a DecryptionRequest: TryInto<CS::Ciphertext>,
    for<'a> <&'a DecryptionRequest as TryInto<CS::Ciphertext>>::Error:
        std::error::Error + Send + Sync + 'static,
    CS::IdentityGroup: EvmSerialize,
{
    fn get_signed_request(
        &self,
        req: &DecryptionRequest,
        sig: CS::IdentityGroup,
        sig_bytes: Cow<'static, Bytes>,
    ) -> Option<SignedDecryptionRequest<'static>> {
        // Preprocess decryption keys using the signature and the ciphertext's ephemeral public key
        let ct: CS::Ciphertext = match req.try_into() {
            Ok(ct) => ct,
            Err(e) => {
                // If we fail to generate keys, it is likely due to an invalid ephemeral public key / ciphertext,
                // not much we can do here.
                tracing::error!(error = %e, request_id = %req.id, "Failed to generate decryption keys / signature... ignoring request");
                None?
            }
        };
        let preprocessed_decryption_key = self.cs.preprocess_decryption_key(sig, ct.ephemeral_pk());
        let signed_req = SignedDecryptionRequest::new(
            req.id,
            Bytes::from(preprocessed_decryption_key.as_ref().to_vec()),
            sig_bytes,
        );
        Some(signed_req)
    }
}

impl<CS> RequestSigningRegistry for Registry<CS>
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
        let mut signatures_cache = self
            .signatures_cache
            .lock()
            .expect("a thread panicked with the mutex");

        let results = inputs
            .into_iter()
            .map(|req| {
                let condition = req.condition.to_vec();

                // If a signature is available, we can preprocess the decryption key immediately
                if let Some((sig, sig_bytes)) = signatures_cache.get(&condition) {
                    self.get_signed_request(req, *sig, sig_bytes.clone())
                } else {
                    // Signature not available, request it
                    if self.new_condition.send(condition).is_err() {
                        tracing::error!("Failed to send condition to signer: channel closed");
                    }
                    None?
                }
            })
            .collect::<Vec<_>>();

        results.into_iter()
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
    async fn libp2p_aggregation() {
        let n = 3;
        let t = 2;
        let g2 = ark_bn254::G2Affine::generator();
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST", 31337);

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

        let libp2p_sk1 = ::libp2p::identity::Keypair::generate_ed25519();
        let libp2p_sk2 = ::libp2p::identity::Keypair::generate_ed25519();
        let libp2p_sk3 = ::libp2p::identity::Keypair::generate_ed25519();

        let ts1 = ThresholdSigner::new(cs.clone(), sk1, n, t, 1, pks.clone());
        let ts2 = ThresholdSigner::new(cs.clone(), sk2, n, t, 2, pks.clone());
        let ts3 = ThresholdSigner::new(cs.clone(), sk3, n, t, 3, pks.clone());

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

        let condition = b"my test condition";
        ch1.new_condition.send(condition.to_vec()).unwrap();
        ch2.new_condition.send(condition.to_vec()).unwrap();
        ch3.new_condition.send(condition.to_vec()).unwrap();

        // Since it's all processed asynchronously, try up to 3 times with 200ms wait each try
        let mut retries = 3;
        let (c1, c2, c3) = loop {
            {
                let cache1 = ch1.signatures_cache.lock().unwrap();
                let cache2 = ch2.signatures_cache.lock().unwrap();
                let cache3 = ch3.signatures_cache.lock().unwrap();
                if !cache1.is_empty() && !cache2.is_empty() && !cache3.is_empty() {
                    // none of the caches are empty, return them
                    break (cache1, cache2, cache3);
                }

                retries -= 1;
                if retries == 0 {
                    break (cache1, cache2, cache3);
                }
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        };

        assert_eq!(c1.len(), 1);
        assert_eq!(c2.len(), 1);
        assert_eq!(c3.len(), 1);
    }
}
