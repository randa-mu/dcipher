//! Implementation of a [`DSignerScheme`] for BLS signatures over any pairing-friendly curve.

mod aggregation;
mod dsigner_scheme_impl;
mod filter;
mod handlers;
pub mod metrics;
mod signer;

pub use aggregation::lagrange_points_interpolate_at;
pub use dsigner_scheme_impl::*;
pub use signer::*;

use crate::bls::filter::BlsFilter;
use crate::dsigner::{
    ApplicationArgs, BlsSignatureAlgorithm, BlsSignatureCurve, BlsSignatureHash, SchemeAlgorithm,
    SchemeDetails, SignatureAlgorithm, SignatureRequest,
};
use ark_ec::pairing::Pairing;
use ark_ec::{AffineRepr, CurveGroup};
use bytes::Bytes;
use dcipher_network::Transport;
use digest::DynDigest;
use digest::core_api::BlockSizeUser;
use itertools::Either;
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use strum::VariantArray;
use tokio_util::sync::CancellationToken;
use utils::dst::NamedCurveGroup;
use utils::hash_to_curve::CustomHashToCurve;
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

// Bunch of type alias to simplify access to BlsVerifier's associated types
type E<BLS> = <BLS as BlsVerifier>::E;
type G1<BLS> = <E<BLS> as Pairing>::G1;
type G2<BLS> = <E<BLS> as Pairing>::G2;
type G1Affine<BLS> = <G1<BLS> as CurveGroup>::Affine;
type G2Affine<BLS> = <G2<BLS> as CurveGroup>::Affine;

type SignatureOrChannel = Either<Bytes, tokio::sync::watch::Sender<Option<Bytes>>>;

type SharedSignatureCache =
    Arc<std::sync::Mutex<LruCache<StoredSignatureRequest, SignatureOrChannel>>>;

type SharedPartialsCache<BLS> = Arc<
    std::sync::Mutex<LruCache<StoredSignatureRequest, HashMap<u16, PartialSignature<Group<BLS>>>>>,
>;

#[derive(Clone, Serialize, Deserialize)]
struct PartialSignature<G> {
    id: u16,
    sig: G,
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
struct StoredSignatureRequest {
    m: Bytes,
    dst: Bytes,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct BlsSignatureRequest {
    m: Bytes,
    args: ApplicationArgs,
    alg: BlsSignatureAlgorithm,
}

impl TryFrom<SignatureRequest> for BlsSignatureRequest {
    type Error = BlsThresholdSignerError;

    fn try_from(value: SignatureRequest) -> Result<Self, Self::Error> {
        let SignatureAlgorithm::Bls(alg) = value.alg else {
            Err(Self::Error::UnsupportedAlgorithm)?
        };

        Ok(Self {
            alg,
            args: value.args,
            m: value.m,
        })
    }
}

impl From<BlsSignatureRequest> for SignatureRequest {
    fn from(value: BlsSignatureRequest) -> Self {
        Self {
            alg: SignatureAlgorithm::Bls(value.alg),
            args: value.args,
            m: value.m,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "Group<BLS>: Serialize",
    deserialize = "Group<BLS>: Deserialize<'de>"
))]
struct PartialSignatureWithMessage<BLS>
where
    BLS: BlsVerifier,
{
    m: Vec<u8>,
    sig: Group<BLS>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "Group<BLS>: Serialize",
    deserialize = "Group<BLS>: Deserialize<'de>"
))]
struct PartialSignatureWithRequest<BLS>
where
    BLS: BlsVerifier,
{
    sig: Group<BLS>,
    #[serde(flatten)]
    req: BlsSignatureRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "G1Affine<BLS>: PointSerializeCompressed, G2Affine<BLS>: PointSerializeCompressed",
    deserialize = "G1Affine<BLS>: PointDeserializeCompressed, G2Affine<BLS>: PointDeserializeCompressed"
))]
enum Group<BLS>
where
    BLS: BlsVerifier,
{
    G1Affine(#[serde(with = "utils::serialize::point::base64")] G1Affine<BLS>),
    G2Affine(#[serde(with = "utils::serialize::point::base64")] G2Affine<BLS>),
}

impl<BLS> Clone for Group<BLS>
where
    BLS: BlsVerifier,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<BLS> Copy for Group<BLS> where BLS: BlsVerifier {}

impl<BLS> Group<BLS>
where
    BLS: BlsVerifier,
{
    fn either(self) -> Either<G1Affine<BLS>, G2Affine<BLS>> {
        match self {
            Group::G1Affine(a) => Either::Left(a),
            Group::G2Affine(a) => Either::Right(a),
        }
    }
}

impl<BLS> From<Group<BLS>> for Either<G1Affine<BLS>, G2Affine<BLS>>
where
    BLS: BlsVerifier,
{
    fn from(value: Group<BLS>) -> Self {
        value.either()
    }
}

impl<BLS> From<Either<G1Affine<BLS>, G2Affine<BLS>>> for Group<BLS>
where
    BLS: BlsVerifier,
{
    fn from(value: Either<G1Affine<BLS>, G2Affine<BLS>>) -> Self {
        match value {
            Either::Left(a) => Group::G1Affine(a),
            Either::Right(a) => Group::G2Affine(a),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BlsThresholdSignerError {
    #[error("underlying signer error")]
    UnderlyingSigner(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("missing partial public key of party {0} on curve g1")]
    MissingPublicKeyG1(u16),

    #[error("missing partial public key of party {0} on curve g1")]
    MissingPublicKeyG2(u16),

    #[error("unsupported hash function: {0:?} does not support {1:?}")]
    UnsupportedHash(BlsSignatureCurve, BlsSignatureHash),

    #[error("unsupported curve: {0:?}")]
    UnsupportedCurve(BlsSignatureCurve),

    #[error("unsupported algorithm")]
    UnsupportedAlgorithm,
}

/// Threshold signer that relies on libp2p to exchange partial signatures.
pub struct BlsThresholdSigner<BLS>
where
    BLS: BlsSigner,
{
    // Signatures cache
    signatures_cache: SharedSignatureCache,

    // Map from messages to partials
    partials_cache: SharedPartialsCache<BLS>,

    // Ciphersuite + Signer
    signer: BLS,

    // Threshold parameters
    #[allow(unused)]
    n: u16,
    t: u16,
    id: u16,

    // Per party public keys
    pks_g1: HashMap<u16, G1Affine<BLS>>,
    pks_g2: HashMap<u16, G2Affine<BLS>>,

    // Group public keys
    pk_g1: Option<G1Affine<BLS>>,
    pk_g2: Option<G2Affine<BLS>>,

    // Applications & algorithms filters
    filter: BlsFilter,

    // Enable the node to broadcast a partial signature upon receiving a valid partial.
    eager_signing: bool,
}

impl<BLS> BlsThresholdSigner<BLS>
where
    BLS: BlsSigner,
{
    /// Create a new threshold signer by specifying the various threshold scheme parameters.
    pub fn new(
        cs: BLS,
        n: u16,
        t: u16,
        id: u16,
        pks_g1: HashMap<u16, G1Affine<BLS>>,
        pks_g2: HashMap<u16, G2Affine<BLS>>,
    ) -> Self {
        Self::new_with_cache_size(
            cs,
            n,
            t,
            id,
            pks_g1,
            pks_g2,
            const { NonZeroUsize::new(64).unwrap() },
        )
    }

    /// New threshold signer with a custom LRU cache size.
    pub fn new_with_cache_size(
        cs: BLS,
        n: u16,
        t: u16,
        id: u16,
        pks_g1: HashMap<u16, G1Affine<BLS>>,
        pks_g2: HashMap<u16, G2Affine<BLS>>,
        lru_cache_size: NonZeroUsize,
    ) -> Self {
        // Compute group public keys
        let pks_g1_vec = pks_g1
            .clone()
            .into_iter()
            .map(|(k, v)| (k.into(), v.into_group()))
            .collect::<Vec<_>>();
        let pks_g2_vec = pks_g2
            .clone()
            .into_iter()
            .map(|(k, v)| (k.into(), v.into_group()))
            .collect::<Vec<_>>();
        let pk_g1: Option<_> = (!pks_g1_vec.is_empty())
            .then_some(lagrange_points_interpolate_at(pks_g1_vec.as_slice(), 0).into_affine());
        let pk_g2: Option<_> = (!pks_g2_vec.is_empty())
            .then_some(lagrange_points_interpolate_at(pks_g2_vec.as_slice(), 0).into_affine());

        Self {
            signatures_cache: Arc::new(std::sync::Mutex::new(LruCache::new(lru_cache_size))),
            partials_cache: Arc::new(std::sync::Mutex::new(LruCache::new(lru_cache_size))),
            signer: cs,
            n,
            t,
            id,
            pks_g1,
            pks_g2,
            pk_g1,
            pk_g2,
            filter: BlsFilter::new(Self::supported_bls_algorithms()),
            // disable eager signing by default, i.e., automatically submitting a partial
            // signature upon receiving a valid partial from another node.
            eager_signing: false,
        }
    }

    /// Enable eager signing by automatically submitting a partial signature upon receiving
    /// a valid partial from another node.
    pub fn with_eager_signing(mut self) -> Self {
        self.eager_signing = true;
        self
    }

    /// Compute all possible supported algorithms for this curve
    fn supported_bls_algorithms() -> impl Iterator<Item = BlsSignatureAlgorithm> {
        let iter_all_hash = |curve| {
            BlsSignatureHash::VARIANTS
                .iter()
                .map(move |&hash| BlsSignatureAlgorithm { hash, curve })
        };

        iter_all_hash(<G1<BLS> as NamedCurveGroup>::CURVE_ID.into())
            .chain(iter_all_hash(<G2<BLS> as NamedCurveGroup>::CURVE_ID.into()))
    }
}

impl<BLS> BlsThresholdSigner<BLS>
where
    BLS: BlsSigner + Clone + Send + Sync + 'static,
    G1Affine<BLS>: PointSerializeCompressed + PointDeserializeCompressed,
    G2Affine<BLS>: PointSerializeCompressed + PointDeserializeCompressed,
{
    fn scheme_details(&self) -> SchemeDetails {
        let collect_supported_algs = |curve| {
            self.filter
                .supported_algs(&curve)
                .map(SignatureAlgorithm::Bls)
                .collect()
        };
        let supported_apps: Vec<_> = self.filter.supported_apps().collect();

        let mut scheme_algs = vec![];
        if let Some(pk_g1) = &self.pk_g1 {
            scheme_algs.push(SchemeAlgorithm {
                public_key: PointSerializeCompressed::ser(pk_g1).unwrap().into(),
                algs: collect_supported_algs(<G2<BLS> as NamedCurveGroup>::CURVE_ID.into()), // sig on G2
                apps: supported_apps.clone(),
            });
        }
        if let Some(pk_g2) = &self.pk_g2 {
            scheme_algs.push(SchemeAlgorithm {
                public_key: PointSerializeCompressed::ser(pk_g2).unwrap().into(),
                algs: collect_supported_algs(<G1<BLS> as NamedCurveGroup>::CURVE_ID.into()), // sig on G1
                apps: supported_apps,
            });
        }

        SchemeDetails {
            n: self.n,
            t: self.t,
            scheme_algs,
        }
    }

    /// Runs the threshold signer in a background task and obtain a cancellation token and a registry.
    pub fn run<T>(self, mut transport: T) -> (CancellationToken, AsyncThresholdSigner)
    where
        T: Transport<Identity = u16>,
    {
        let arc_self = Arc::new(self);
        let cancellation_token = CancellationToken::new();
        let (tx_registry_to_signer, rx_signer_to_registry) = tokio::sync::mpsc::unbounded_channel();

        // Create a [`DSignerScheme`]
        let signer = AsyncThresholdSigner::new(
            arc_self.scheme_details(),
            arc_self.signatures_cache.clone(),
            tx_registry_to_signer.clone(),
            arc_self.filter.clone(),
        );

        let partials_stream = transport
            .receiver_stream()
            .expect("transport should provide at least one receiver stream");
        let tx_signer_to_network = transport
            .sender()
            .expect("transport should provide at least one partial sender");

        // Spawn task that handles signing requests from registry
        tokio::task::spawn(arc_self.clone().recv_new_requests(
            rx_signer_to_registry,
            tx_signer_to_network,
            cancellation_token.child_token(),
        ));

        // Spawn task that handles messages from other nodes
        tokio::task::spawn(arc_self.clone().recv_new_signatures(
            partials_stream,
            tx_registry_to_signer,
            cancellation_token.child_token(),
        ));

        (cancellation_token, signer)
    }

    /// Sign a message using a specified algorithm
    fn sign(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
        alg: &BlsSignatureAlgorithm,
    ) -> Result<Group<BLS>, BlsThresholdSignerError> {
        match (alg.curve, alg.hash) {
            // Sign on G1
            (curve_id, hash_id) if Self::is_curve_g1(curve_id) => {
                let sig = match hash_id {
                    #[cfg(feature = "sha2")]
                    BlsSignatureHash::Sha256 => self.signer.sign_g1::<sha2::Sha256>(m, dst),
                    #[cfg(feature = "sha3")]
                    BlsSignatureHash::Keccak256 => self.signer.sign_g1::<sha3::Keccak256>(m, dst),
                };

                Ok(Group::G1Affine(sig.map_err(|e| {
                    BlsThresholdSignerError::UnderlyingSigner(e.into())
                })?))
            }

            // Sign on G2
            (curve_id, hash_id) if Self::is_curve_g2(curve_id) => {
                let sig = match hash_id {
                    #[cfg(feature = "sha2")]
                    BlsSignatureHash::Sha256 => self.signer.sign_g2::<sha2::Sha256>(m, dst),
                    #[cfg(feature = "sha3")]
                    BlsSignatureHash::Keccak256 => self.signer.sign_g2::<sha3::Keccak256>(m, dst),
                };

                Ok(Group::G2Affine(sig.map_err(|e| {
                    BlsThresholdSignerError::UnderlyingSigner(e.into())
                })?))
            }

            // Curve is neither G1 nor G2
            (curve, _) => Err(BlsThresholdSignerError::UnsupportedCurve(curve)),
        }
    }

    /// Verify a message using a specified algorithm if supported, Err otherwise
    fn try_verify(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
        sig: Group<BLS>,
        party_id: &u16,
        alg: &BlsSignatureAlgorithm,
    ) -> Result<bool, BlsThresholdSignerError> {
        let pk: Group<BLS> = match &sig {
            Group::G1Affine(_) => {
                let pk = self
                    .pks_g2
                    .get(party_id)
                    .cloned()
                    .or_else(|| {
                        tracing::warn!(sender_id = party_id, "Missing pk on g2 for party");
                        None
                    })
                    .ok_or(BlsThresholdSignerError::MissingPublicKeyG2(*party_id))?;
                Ok(Group::G2Affine(pk))
            }
            Group::G2Affine(_) => {
                let pk = self
                    .pks_g1
                    .get(party_id)
                    .cloned()
                    .or_else(|| {
                        tracing::warn!(sender_id = party_id, "Missing pk on g1 for party");
                        None
                    })
                    .ok_or(BlsThresholdSignerError::MissingPublicKeyG1(*party_id))?;
                Ok(Group::G1Affine(pk))
            }
        }?;

        match (alg.curve, alg.hash, sig, pk) {
            // Signature on G1, public key on G2
            (curve, hash, Group::G1Affine(sig), Group::G2Affine(pk))
                if Self::is_curve_g1(curve) =>
            {
                let valid = match hash {
                    #[cfg(feature = "sha2")]
                    BlsSignatureHash::Sha256 => {
                        self.signer.verify_g1::<sha2::Sha256>(m, dst, sig, pk)
                    }

                    #[cfg(feature = "sha3")]
                    BlsSignatureHash::Keccak256 => {
                        self.signer.verify_g1::<sha3::Keccak256>(m, dst, sig, pk)
                    }
                };
                Ok(valid)
            }

            // Signature on G2, public key on G1
            (curve, hash, Group::G2Affine(sig), Group::G1Affine(pk))
                if Self::is_curve_g2(curve) =>
            {
                let valid = match hash {
                    #[cfg(feature = "sha2")]
                    BlsSignatureHash::Sha256 => {
                        self.signer.verify_g2::<sha2::Sha256>(m, dst, sig, pk)
                    }

                    #[cfg(feature = "sha3")]
                    BlsSignatureHash::Keccak256 => {
                        self.signer.verify_g2::<sha3::Keccak256>(m, dst, sig, pk)
                    }
                };
                Ok(valid)
            }

            // Curve is neither G1 nor G2
            (curve, ..) => Err(BlsThresholdSignerError::UnsupportedCurve(curve)),
        }
    }

    /// Does the specified curve correspond to G1
    fn is_curve_g1(curve: BlsSignatureCurve) -> bool {
        <G1<BLS> as NamedCurveGroup>::CURVE_ID == curve.into()
    }

    /// Does the specified curve correspond to G2
    fn is_curve_g2(curve: BlsSignatureCurve) -> bool {
        <G2<BLS> as NamedCurveGroup>::CURVE_ID == curve.into()
    }
}

pub trait BlsVerifier
where
    <Self::E as Pairing>::G1: CustomHashToCurve + NamedCurveGroup,
    <Self::E as Pairing>::G2: CustomHashToCurve + NamedCurveGroup,
{
    type E: Pairing;

    /// Outputs true if the signature is valid under the specified message, DST, and public key.
    fn verify_g1<H: DynDigest + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
        signature: <Self::E as Pairing>::G1Affine,
        public_key: <Self::E as Pairing>::G2Affine,
    ) -> bool;

    /// Outputs true if the signature is valid under the specified message, DST, and public key.
    fn verify_g2<H: DynDigest + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
        signature: <Self::E as Pairing>::G2Affine,
        public_key: <Self::E as Pairing>::G1Affine,
    ) -> bool;
}

pub trait BlsSigner: BlsVerifier {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Sign a message using the signer's private key and a custom DST.
    fn sign_g1<H: DynDigest + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
    ) -> Result<<Self::E as Pairing>::G1Affine, Self::Error>;

    /// Sign a message using the signer's private key and a custom DST.
    fn sign_g2<H: DynDigest + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
    ) -> Result<<Self::E as Pairing>::G2Affine, Self::Error>;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use super::super::*;
        use crate::dsigner::{ApplicationAnyArgs, ApplicationArgs, DSignerSchemeSigner};
        use ark_bn254::Fr;
        use ark_ec::AffineRepr;
        use ark_ff::MontFp;
        use dcipher_network::transports::in_memory::MemoryNetwork;
        use std::time::Duration;

        #[tokio::test]
        async fn async_threshold_signer() {
            let n = 3;
            let t = 2;
            let g1 = ark_bn254::G1Affine::generator();
            let g2 = ark_bn254::G2Affine::generator();

            let _sk: Fr = MontFp!(
                "7685086713915354683875500702831995067084988389812060097318430034144315778947"
            );
            let sk1: Fr = MontFp!(
                "5840327440053394277204603653048962762290958051681898697354171413163183818203"
            );
            let sk2: Fr = MontFp!(
                "3995568166191433870533706603265930457496927713551737297389912792182051857459"
            );
            let sk3: Fr = MontFp!(
                "2150808892329473463862809553482898152702897375421575897425654171200919896715"
            );
            let pks_g2 = vec![g2 * sk1, g2 * sk2, g2 * sk3];
            let pks_g2 = pks_g2
                .into_iter()
                .enumerate()
                .map(|(i, pki)| (i as u16 + 1, pki.into_affine()))
                .collect::<HashMap<_, _>>();

            let pks_g1 = vec![g1 * sk1, g1 * sk2, g1 * sk3];
            let pks_g1 = pks_g1
                .into_iter()
                .enumerate()
                .map(|(i, pki)| (i as u16 + 1, pki.into_affine()))
                .collect::<HashMap<_, _>>();

            let cs1 = BlsPairingSigner::new_bn254(sk1);
            let cs2 = BlsPairingSigner::new_bn254(sk2);
            let cs3 = BlsPairingSigner::new_bn254(sk3);

            // Get transports
            let mut transports = MemoryNetwork::get_transports(1..=3);

            // Start three threshold signers
            let (_, ch1) = BlsThresholdSigner::new(cs1, n, t, 1, pks_g1.clone(), pks_g2.clone())
                .run(transports.pop_front().unwrap());
            let (_, ch2) = BlsThresholdSigner::new(cs2, n, t, 2, pks_g1.clone(), pks_g2.clone())
                .run(transports.pop_front().unwrap());
            let (_, ch3) = BlsThresholdSigner::new(cs3, n, t, 3, pks_g1.clone(), pks_g2.clone())
                .run(transports.pop_front().unwrap());

            let message = b"my test message";
            let req = SignatureRequest {
                m: message.to_vec().into(),
                args: ApplicationArgs::Any(ApplicationAnyArgs {
                    dst_suffix: "TEST".to_owned(),
                }),
                alg: SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                    curve: BlsSignatureCurve::Bn254G2,
                    hash: BlsSignatureHash::Keccak256,
                }),
            };
            let fut_sig1 = ch1.async_sign(req.clone());
            let fut_sig2 = ch2.async_sign(req.clone());
            let fut_sig3 = ch3.async_sign(req.clone());

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
}
