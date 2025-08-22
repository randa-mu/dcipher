//! Module used to dynamically generate RFC 9380 compliant DSTs (domain separation tags).

use ark_ec::CurveGroup;
use digest::DynDigest;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Structure representing an RFC 9380 DST.
#[derive(Clone)]
pub struct Rfc9380Dst(pub Vec<u8>);

/// Helper struct used to build an RFC 9380 DST by specifying various parameters such as application name,
/// curve, hash function, mapping type, encoding and suffix.
/// Note that we do not enforce compliance, hence one can build DST with fewer elements, by not specifying
/// the curve name for instance.
/// The resulting DST is built as:
///     `%application_name%_%curve_name%_%expand%_%hash_name%_%mapping%_%encoding%_%suffix%_`
/// where `expand` is set to `XMD` when using a fixed-width hash function, and to `XOF` when using an
/// extendable-output function.
#[derive(Clone)]
pub struct Rfc9380DstBuilder {
    application_name: Option<Vec<u8>>,
    curve_id: Option<Vec<u8>>,
    hash_id: Option<Vec<u8>>,
    mapping: Option<Vec<u8>>,
    encoding: Option<Vec<u8>>,
    suffix: Option<Vec<u8>>,
}

/// Identifier for the elliptic curve group based on RFC 9380.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum CurveId {
    Bn254G1,
    Bn254G2,
    Bls12_381G1,
    Bls12_381G2,
    Custom(Cow<'static, [u8]>, MapId),
}

impl CurveId {
    pub fn default_mapping(&self) -> MapId {
        match self {
            CurveId::Bn254G1 | CurveId::Bn254G2 => MapId::SVDW,
            CurveId::Bls12_381G1 | CurveId::Bls12_381G2 => MapId::SSWU,
            CurveId::Custom(_, map_id) => *map_id,
        }
    }
}

impl From<CurveId> for Cow<'static, [u8]> {
    fn from(value: CurveId) -> Self {
        match value {
            CurveId::Bn254G1 => {
                const CURVE_ID: Cow<'static, [u8]> = Cow::Borrowed(b"BN254G1");
                CURVE_ID
            }
            CurveId::Bn254G2 => {
                const CURVE_ID: Cow<'static, [u8]> = Cow::Borrowed(b"BN254G2");
                CURVE_ID
            }
            CurveId::Bls12_381G1 => {
                const CURVE_ID: Cow<'static, [u8]> = Cow::Borrowed(b"BLS12381G1");
                CURVE_ID
            }
            CurveId::Bls12_381G2 => {
                const CURVE_ID: Cow<'static, [u8]> = Cow::Borrowed(b"BLS12381G2");
                CURVE_ID
            }
            CurveId::Custom(name, _) => name,
        }
    }
}

/// Curve group with an associated curve identifier.
pub trait NamedCurveGroup: CurveGroup {
    const CURVE_ID: CurveId;
}

/// Identifier for the cryptographic hash function based on RFC 9380.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum HashId {
    Sha256,
    Sha3_256,
    Keccak256,
    Custom(Cow<'static, [u8]>),
}

impl From<HashId> for Cow<'static, [u8]> {
    fn from(value: HashId) -> Self {
        match value {
            HashId::Sha256 => Cow::Borrowed(b"SHA-256"),
            HashId::Sha3_256 => Cow::Borrowed(b"SHA3-256"),
            HashId::Keccak256 => Cow::Borrowed(b"KECCAK-256"),
            HashId::Custom(name) => name,
        }
    }
}

/// Identifier for extensible-output cryptographic hash functions based on RFC 9380.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum XofId {
    Custom(Cow<'static, [u8]>),
}

impl From<XofId> for Cow<'static, [u8]> {
    fn from(value: XofId) -> Self {
        match value {
            XofId::Custom(name) => name,
        }
    }
}

/// DynDigest with an associated hash identifier.
pub trait NamedDynDigest: DynDigest {
    const HASH_ID: HashId;
}

/// Identifier for the `map_to_curve` function described in RFC 9380.
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MapId {
    SVDW,
    SSWU,
    ELL2,
}

impl From<MapId> for &'static [u8] {
    fn from(val: MapId) -> &'static [u8] {
        match val {
            MapId::SVDW => b"SVDW",
            MapId::SSWU => b"SSWU",
            MapId::ELL2 => b"ELL2",
        }
    }
}

/// Encoding types described in RFC 9380.
pub enum EncodingType {
    NonUniform,
    Uniform,
}

impl From<EncodingType> for &'static [u8] {
    fn from(val: EncodingType) -> Self {
        match val {
            EncodingType::NonUniform => b"NU",
            EncodingType::Uniform => b"RO",
        }
    }
}

impl Rfc9380DstBuilder {
    pub const fn empty() -> Self {
        Self {
            application_name: None,
            hash_id: None,
            curve_id: None,
            mapping: None,
            encoding: None,
            suffix: None,
        }
    }

    pub fn with_application_name(mut self, app_name: Vec<u8>) -> Self {
        self.application_name = Some(app_name);
        self
    }

    pub fn with_curve<CG: NamedCurveGroup>(self) -> Self {
        self.with_curve_id(CG::CURVE_ID)
    }

    pub fn with_curve_id(mut self, curve_id: CurveId) -> Self {
        self.curve_id = Some(curve_id.into());
        self
    }

    pub fn with_xof_id(mut self, hash_id: XofId) -> Self {
        let name: &[u8] = &Cow::from(hash_id);
        self.hash_id = Some([b"XOF:", name].concat().to_vec());
        self
    }

    pub fn with_hash<H: NamedDynDigest>(self) -> Self {
        self.with_hash_id(H::HASH_ID)
    }

    pub fn with_hash_id(mut self, hash_id: HashId) -> Self {
        let name: &[u8] = &Cow::from(hash_id);
        self.hash_id = Some([b"XMD:", name].concat().to_vec());
        self
    }

    pub fn with_mapping(mut self, mapping: MapId) -> Self {
        self.mapping = Some(Into::<&[u8]>::into(mapping).to_vec());
        self
    }

    pub fn with_encoding(mut self, encoding: EncodingType) -> Self {
        self.encoding = Some(Into::<&[u8]>::into(encoding).to_vec());
        self
    }

    pub fn with_suffix(mut self, suffix: Vec<u8>) -> Self {
        self.suffix = Some(suffix);
        self
    }

    /// Build with the default prefix separator ('_')
    pub fn build(self) -> Rfc9380Dst {
        self.build_with_app_name_sep(b'_')
    }

    /// Build with a custom prefix separator
    pub fn build_with_app_name_sep(self, prefix_sep: u8) -> Rfc9380Dst {
        let mut dst = vec![];
        if let Some(application_name) = self.application_name {
            dst = [dst, application_name, vec![prefix_sep]].concat();
        }

        if let Some(curve_name) = self.curve_id {
            dst = [dst, curve_name, vec![b'_']].concat();
        }

        if let Some(hash_name) = self.hash_id {
            dst = [dst, hash_name, vec![b'_']].concat();
        }

        if let Some(mapping) = self.mapping {
            dst = [dst, mapping, vec![b'_']].concat();
        }

        if let Some(encoding) = self.encoding {
            dst = [dst, encoding, vec![b'_']].concat();
        }

        if let Some(suffix) = self.suffix {
            dst = [dst, suffix, vec![b'_']].concat();
        }

        Rfc9380Dst(dst)
    }
}

impl From<CurveId> for Vec<u8> {
    fn from(value: CurveId) -> Self {
        Cow::from(value).to_vec()
    }
}

impl From<HashId> for Vec<u8> {
    fn from(value: HashId) -> Self {
        Cow::from(value).to_vec()
    }
}

impl From<Rfc9380Dst> for Vec<u8> {
    fn from(value: Rfc9380Dst) -> Self {
        value.0
    }
}

#[cfg(feature = "bn254")]
mod bn254_named {
    use super::{CurveId, NamedCurveGroup};

    impl NamedCurveGroup for ark_ec::short_weierstrass::Projective<ark_bn254::g1::Config> {
        const CURVE_ID: CurveId = CurveId::Bn254G1;
    }

    impl NamedCurveGroup for ark_ec::short_weierstrass::Projective<ark_bn254::g2::Config> {
        const CURVE_ID: CurveId = CurveId::Bn254G2;
    }
}

#[cfg(feature = "bls12-381")]
mod bls12_381_named {
    use super::{CurveId, NamedCurveGroup};

    impl NamedCurveGroup for ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config> {
        const CURVE_ID: CurveId = CurveId::Bls12_381G1;
    }

    impl NamedCurveGroup for ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config> {
        const CURVE_ID: CurveId = CurveId::Bls12_381G2;
    }
}

#[cfg(feature = "sha3")]
mod sha3_named {
    use super::{HashId, NamedDynDigest};

    impl NamedDynDigest for sha3::Sha3_256 {
        const HASH_ID: HashId = HashId::Sha3_256;
    }

    impl NamedDynDigest for sha3::Keccak256 {
        const HASH_ID: HashId = HashId::Keccak256;
    }
}

mod sha2_named {
    use super::{HashId, NamedDynDigest};

    impl NamedDynDigest for sha2::Sha256 {
        const HASH_ID: HashId = HashId::Sha256;
    }
}

#[cfg(test)]
mod tests {
    // Test ciphersuites for BLS12-381
    #[cfg(feature = "bls12-381")]
    mod bls12_381 {
        use super::super::*;

        #[test]
        fn bls12_rfc9380() {
            // BLS12381G1_XMD:SHA-256_SSWU_RO_
            let bls12_381_g1_ciphersuite_ro = Rfc9380DstBuilder::empty()
                .with_curve::<ark_bls12_381::G1Projective>()
                .with_hash::<sha2::Sha256>()
                .with_mapping(MapId::SSWU)
                .with_encoding(EncodingType::Uniform)
                .build();
            assert_eq!(
                bls12_381_g1_ciphersuite_ro.0.as_slice(),
                b"BLS12381G1_XMD:SHA-256_SSWU_RO_"
            );

            // BLS12381G1_XMD:SHA-256_SSWU_NU_
            let bls12_381_g1_ciphersuite_nu = Rfc9380DstBuilder::empty()
                .with_curve::<ark_bls12_381::G1Projective>()
                .with_hash::<sha2::Sha256>()
                .with_mapping(MapId::SSWU)
                .with_encoding(EncodingType::NonUniform)
                .build();
            assert_eq!(
                bls12_381_g1_ciphersuite_nu.0.as_slice(),
                b"BLS12381G1_XMD:SHA-256_SSWU_NU_"
            );

            // BLS12381G2_XMD:SHA-256_SSWU_RO_
            let bls12_381_g2_ciphersuite_ro = Rfc9380DstBuilder::empty()
                .with_curve::<ark_bls12_381::G2Projective>()
                .with_hash::<sha2::Sha256>()
                .with_mapping(MapId::SSWU)
                .with_encoding(EncodingType::Uniform)
                .build();
            assert_eq!(
                bls12_381_g2_ciphersuite_ro.0.as_slice(),
                b"BLS12381G2_XMD:SHA-256_SSWU_RO_"
            );

            // BLS12381G2_XMD:SHA-256_SSWU_NU_
            let bls12_381_g2_ciphersuite_nu = Rfc9380DstBuilder::empty()
                .with_curve::<ark_bls12_381::G2Projective>()
                .with_hash::<sha2::Sha256>()
                .with_mapping(MapId::SSWU)
                .with_encoding(EncodingType::NonUniform)
                .build();
            assert_eq!(
                bls12_381_g2_ciphersuite_nu.0.as_slice(),
                b"BLS12381G2_XMD:SHA-256_SSWU_NU_"
            );
        }
    }
}
