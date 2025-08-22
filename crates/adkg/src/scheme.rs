//! Default schemes configurations that can be used for the ADKG.

use crate::aba::AbaConfig;
use crate::adkg::Adkg;
use crate::helpers::PartyId;
use crate::rbc::ReliableBroadcastConfig;
use crate::vss::acss::AcssConfig;
use ark_ec::{CurveGroup, Group};
use ark_std::UniformRand;
use digest::DynDigest;
use digest::core_api::BlockSizeUser;
use rand::{CryptoRng, Rng};
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use utils::dst::{CurveId, HashId, NamedCurveGroup, NamedDynDigest};
use utils::hash_to_curve::HashToCurve;
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

pub const ADKG_VERSION: &str = "v0.1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdkgSchemeConfig {
    app_name: String,
    curve_id: CurveId,
    hash_id: HashId,
    adkg_version: String,
    pub adkg_scheme_name: String,
    generator_g: String,
    generator_h: String,
}

pub trait AdkgScheme: Send + Sync + 'static
where
    <Self::Curve as Group>::ScalarField: FqSerialize + FqDeserialize,
{
    const NAME: &'static str;

    type Error: std::error::Error + Send + Sync + 'static;
    type Curve: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve;
    type Hash: Default + DynDigest + BlockSizeUser + Clone;

    type RBCConfig: ReliableBroadcastConfig<'static, PartyId>;
    type ACSSConfig: AcssConfig<'static, Self::Curve, PartyId>;
    type ABAConfig: AbaConfig<'static, PartyId>;

    fn generator_g(&self) -> Self::Curve;

    fn keygen(
        &self,
        rng: &mut (impl Rng + CryptoRng),
    ) -> (<Self::Curve as Group>::ScalarField, Self::Curve) {
        let sk = <Self::Curve as Group>::ScalarField::rand(rng);
        let pk = self.generator_g() * sk;
        (sk, pk)
    }

    #[allow(clippy::type_complexity)]
    fn new_adkg(
        &self,
        id: PartyId,
        n: NonZeroUsize,
        t: NonZeroUsize,
        sk: <Self::Curve as Group>::ScalarField,
        pks: Vec<Self::Curve>,
    ) -> Result<
        Adkg<Self::Curve, Self::Hash, Self::RBCConfig, Self::ACSSConfig, Self::ABAConfig>,
        Self::Error,
    >
    where
        Self::Curve: NamedCurveGroup,
        Self::Hash: NamedDynDigest;
}

#[cfg(feature = "bn254")]
pub mod bn254 {
    use crate::aba::crain20::AbaCrain20Config;
    use crate::adkg::Adkg;
    use crate::adkg::types::LazyCoinKeys;
    use crate::helpers::PartyId;
    use crate::network::RetryStrategy;
    use crate::rbc::r4::Rbc4RoundsConfig;
    use crate::scheme::ADKG_VERSION;
    use crate::scheme::{AdkgScheme, AdkgSchemeConfig};
    use crate::vss::acss::hbacss0::HbAcss0Config;
    use ark_ec::Group;
    use digest::core_api::BlockSizeUser;
    use std::num::NonZeroUsize;
    use utils::dst::{EncodingType, MapId, NamedCurveGroup, NamedDynDigest, Rfc9380DstBuilder};
    use utils::hash_to_curve::HashToCurve;
    use utils::serialize::SerializationError;
    use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

    pub struct DYX22Bn254G1Keccak256 {
        app_name: String,
        generator_h: ark_bn254::G1Projective,
    }

    impl DYX22Bn254G1Keccak256 {
        pub fn new(app_name: String, generator_h: ark_bn254::G1Projective) -> Self {
            Self {
                app_name,
                generator_h,
            }
        }
    }

    impl AdkgScheme for DYX22Bn254G1Keccak256 {
        const NAME: &'static str = "DYX22-Bn254G1-Keccak256";

        type Error = DYX22Bn254G1Keccak256Error;
        type Curve = ark_bn254::G1Projective;
        type Hash = sha3::Keccak256;

        type RBCConfig = Rbc4RoundsConfig;
        type ACSSConfig = HbAcss0Config<Self::Curve, Self::Hash, Self::RBCConfig>;
        type ABAConfig = AbaCrain20Config<Self::Curve, LazyCoinKeys<Self::Curve>, Self::Hash>;

        fn generator_g(&self) -> Self::Curve {
            let app_name = format!("ADKG-{ADKG_VERSION}-{}", self.app_name)
                .as_bytes()
                .to_owned();
            get_generator_g_svdw::<_, Self::Hash>(app_name)
        }

        fn new_adkg(
            &self,
            id: PartyId,
            n: NonZeroUsize,
            t: NonZeroUsize,
            sk: <Self::Curve as Group>::ScalarField,
            pks: Vec<Self::Curve>,
        ) -> Result<
            Adkg<Self::Curve, Self::Hash, Self::RBCConfig, Self::ACSSConfig, Self::ABAConfig>,
            Self::Error,
        >
        where
            Self::Curve: NamedCurveGroup,
            Self::Hash: NamedDynDigest,
        {
            let n = n.get();
            let t = t.get();
            let max_t = (n - 1) / 3;
            if t > max_t {
                Err(Self::Error::ThresholdTooHigh(max_t))?
            }

            let retry_strategy = RetryStrategy::None;
            let generator_g = self.generator_g();

            let rbc_config = Rbc4RoundsConfig::new(id, n, t, &retry_strategy);
            let acss_config = HbAcss0Config::<_, Self::Hash, _>::new(
                id,
                sk,
                pks,
                rbc_config.clone(),
                n,
                t,
                generator_g,
                retry_strategy,
            );
            let aba_config =
                AbaCrain20Config::<_, _, _>::new(id, n, t, generator_g, retry_strategy);

            Ok(Adkg::new(
                id,
                n,
                t,
                generator_g,
                self.generator_h,
                rbc_config,
                acss_config,
                aba_config,
            ))
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum DYX22Bn254G1Keccak256Error {
        #[error("bad curve id")]
        BadCurveId,

        #[error("bad hash id")]
        BadHashId,

        #[error("generator does not match expected")]
        BadGenerator,

        #[error("unsupported adkg scheme")]
        UnsupportedScheme,

        #[error("unsupported adkg version")]
        UnsupportedAdkgVersion,

        #[error("threshold too high: t must be <= (n - 1) / 3 = {0}")]
        ThresholdTooHigh(usize),

        #[error("failed to deserialize point")]
        PointDeserialize(#[from] SerializationError),
    }

    impl TryFrom<AdkgSchemeConfig> for DYX22Bn254G1Keccak256 {
        type Error = DYX22Bn254G1Keccak256Error;

        fn try_from(value: AdkgSchemeConfig) -> Result<Self, Self::Error> {
            if value.adkg_scheme_name != <Self as AdkgScheme>::NAME {
                Err(Self::Error::UnsupportedScheme)?
            }

            if value.adkg_version != ADKG_VERSION {
                Err(Self::Error::UnsupportedAdkgVersion)?
            }

            if value.curve_id != <<Self as AdkgScheme>::Curve as NamedCurveGroup>::CURVE_ID {
                Err(Self::Error::BadCurveId)?
            }

            if value.hash_id != <<Self as AdkgScheme>::Hash as NamedDynDigest>::HASH_ID {
                Err(Self::Error::BadHashId)?
            }

            let generator_g =
                <Self as AdkgScheme>::Curve::deser_compressed_base64(&value.generator_g)?;
            let generator_h =
                <Self as AdkgScheme>::Curve::deser_compressed_base64(&value.generator_g)?;
            let scheme = Self::new(value.app_name, generator_h);

            // Make sure that the generator corresponds to the dynamically computed generator
            if generator_g != scheme.generator_g() {
                Err(Self::Error::BadGenerator)?
            }

            Ok(scheme)
        }
    }

    impl From<DYX22Bn254G1Keccak256> for AdkgSchemeConfig {
        fn from(value: DYX22Bn254G1Keccak256) -> Self {
            let generator_g = value
                .generator_g()
                .ser_compressed_base64()
                .expect("failed to serialize point to base64");
            let generator_h = value
                .generator_h
                .ser_compressed_base64()
                .expect("failed to serialize point to base64");
            Self {
                app_name: value.app_name,
                adkg_version: ADKG_VERSION.to_owned(),
                adkg_scheme_name: <DYX22Bn254G1Keccak256 as AdkgScheme>::NAME.to_owned(),
                generator_g,
                generator_h,
                curve_id:
                    <<DYX22Bn254G1Keccak256 as AdkgScheme>::Curve as NamedCurveGroup>::CURVE_ID,
                hash_id: <<DYX22Bn254G1Keccak256 as AdkgScheme>::Hash as NamedDynDigest>::HASH_ID,
            }
        }
    }

    fn get_generator_g_svdw<CG, H>(app_name: Vec<u8>) -> CG
    where
        CG: NamedCurveGroup + HashToCurve,
        H: Default + NamedDynDigest + BlockSizeUser + Clone,
    {
        let dst: Vec<_> = Rfc9380DstBuilder::empty()
            .with_application_name(app_name)
            .with_curve::<CG>()
            .with_hash::<H>()
            .with_mapping(MapId::SVDW)
            .with_encoding(EncodingType::Uniform)
            .with_suffix(b"GENERATORS".to_vec())
            .build()
            .into();

        CG::hash_to_curve_custom::<H>(b"ADKG_GENERATOR_G", &dst)
    }
}

#[cfg(feature = "bls12-381")]
pub mod bls12_381 {
    use crate::aba::crain20::AbaCrain20Config;
    use crate::adkg::Adkg;
    use crate::adkg::types::LazyCoinKeys;
    use crate::helpers::PartyId;
    use crate::network::RetryStrategy;
    use crate::rbc::r4::Rbc4RoundsConfig;
    use crate::scheme::ADKG_VERSION;
    use crate::scheme::{AdkgScheme, AdkgSchemeConfig};
    use crate::vss::acss::hbacss0::HbAcss0Config;
    use ark_ec::Group;
    use digest::core_api::BlockSizeUser;
    use std::num::NonZeroUsize;
    use utils::dst::{EncodingType, MapId, NamedCurveGroup, NamedDynDigest, Rfc9380DstBuilder};
    use utils::hash_to_curve::HashToCurve;
    use utils::serialize::SerializationError;
    use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

    pub struct DYX22Bls12_381G1Sha256 {
        app_name: String,
        generator_h: ark_bls12_381::G1Projective,
    }

    impl DYX22Bls12_381G1Sha256 {
        pub fn new(app_name: String, generator_h: ark_bls12_381::G1Projective) -> Self {
            Self {
                app_name,
                generator_h,
            }
        }
    }

    impl AdkgScheme for DYX22Bls12_381G1Sha256 {
        const NAME: &'static str = "DYX22-Bls12_381G1-Sha256";

        type Error = DYX22Bls12_381G1Sha256Error;
        type Curve = ark_bls12_381::G1Projective;
        type Hash = sha2::Sha256;

        type RBCConfig = Rbc4RoundsConfig;
        type ACSSConfig = HbAcss0Config<Self::Curve, Self::Hash, Self::RBCConfig>;
        type ABAConfig = AbaCrain20Config<Self::Curve, LazyCoinKeys<Self::Curve>, Self::Hash>;

        fn generator_g(&self) -> Self::Curve {
            let app_name = format!("ADKG-{ADKG_VERSION}-{}", self.app_name)
                .as_bytes()
                .to_owned();
            get_generator_g_svdw::<_, Self::Hash>(app_name)
        }

        fn new_adkg(
            &self,
            id: PartyId,
            n: NonZeroUsize,
            t: NonZeroUsize,
            sk: <Self::Curve as Group>::ScalarField,
            pks: Vec<Self::Curve>,
        ) -> Result<
            Adkg<Self::Curve, Self::Hash, Self::RBCConfig, Self::ACSSConfig, Self::ABAConfig>,
            Self::Error,
        >
        where
            Self::Curve: NamedCurveGroup,
            Self::Hash: NamedDynDigest,
        {
            let n = n.get();
            let t = t.get();
            let max_t = (n - 1) / 3;
            if t > max_t {
                Err(Self::Error::ThresholdTooHigh(max_t))?
            }

            let retry_strategy = RetryStrategy::None;
            let generator_g = self.generator_g();

            let rbc_config = Rbc4RoundsConfig::new(id, n, t, &retry_strategy);
            let acss_config = HbAcss0Config::<_, Self::Hash, _>::new(
                id,
                sk,
                pks,
                rbc_config.clone(),
                n,
                t,
                generator_g,
                retry_strategy,
            );
            let aba_config =
                AbaCrain20Config::<_, _, _>::new(id, n, t, generator_g, retry_strategy);

            Ok(Adkg::new(
                id,
                n,
                t,
                generator_g,
                self.generator_h,
                rbc_config,
                acss_config,
                aba_config,
            ))
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum DYX22Bls12_381G1Sha256Error {
        #[error("bad curve id")]
        BadCurveId,

        #[error("bad hash id")]
        BadHashId,

        #[error("generator does not match expected")]
        BadGenerator,

        #[error("unsupported adkg scheme")]
        UnsupportedScheme,

        #[error("unsupported adkg version")]
        UnsupportedAdkgVersion,

        #[error("threshold too high: t must be <= (n - 1) / 3 = {0}")]
        ThresholdTooHigh(usize),

        #[error("failed to deserialize point")]
        PointDeserialize(#[from] SerializationError),
    }

    impl TryFrom<AdkgSchemeConfig> for DYX22Bls12_381G1Sha256 {
        type Error = DYX22Bls12_381G1Sha256Error;

        fn try_from(value: AdkgSchemeConfig) -> Result<Self, Self::Error> {
            if value.adkg_scheme_name != <Self as AdkgScheme>::NAME {
                Err(Self::Error::UnsupportedScheme)?
            }

            if value.adkg_version != ADKG_VERSION {
                Err(Self::Error::UnsupportedAdkgVersion)?
            }

            if value.curve_id != <<Self as AdkgScheme>::Curve as NamedCurveGroup>::CURVE_ID {
                Err(Self::Error::BadCurveId)?
            }

            if value.hash_id != <<Self as AdkgScheme>::Hash as NamedDynDigest>::HASH_ID {
                Err(Self::Error::BadHashId)?
            }

            let generator_g = <Self as AdkgScheme>::Curve::deser_base64(&value.generator_g)?;
            let generator_h = <Self as AdkgScheme>::Curve::deser_base64(&value.generator_g)?;
            let scheme = Self::new(value.app_name, generator_h);

            // Make sure that the generator corresponds to the dynamically computed generator
            if generator_g != scheme.generator_g() {
                Err(Self::Error::BadGenerator)?
            }

            Ok(scheme)
        }
    }

    impl From<DYX22Bls12_381G1Sha256> for AdkgSchemeConfig {
        fn from(value: DYX22Bls12_381G1Sha256) -> Self {
            let generator_g = value
                .generator_g()
                .ser_base64()
                .expect("failed to serialize point to base64");
            let generator_h = value
                .generator_h
                .ser_base64()
                .expect("failed to serialize point to base64");
            Self {
                app_name: value.app_name,
                adkg_version: ADKG_VERSION.to_owned(),
                adkg_scheme_name: <DYX22Bls12_381G1Sha256 as AdkgScheme>::NAME.to_owned(),
                generator_g,
                generator_h,
                curve_id:
                    <<DYX22Bls12_381G1Sha256 as AdkgScheme>::Curve as NamedCurveGroup>::CURVE_ID,
                hash_id: <<DYX22Bls12_381G1Sha256 as AdkgScheme>::Hash as NamedDynDigest>::HASH_ID,
            }
        }
    }

    fn get_generator_g_svdw<CG, H>(app_name: Vec<u8>) -> CG
    where
        CG: NamedCurveGroup + HashToCurve,
        H: Default + NamedDynDigest + BlockSizeUser + Clone,
    {
        let dst: Vec<_> = Rfc9380DstBuilder::empty()
            .with_application_name(app_name)
            .with_curve::<CG>()
            .with_hash::<H>()
            .with_mapping(MapId::SVDW)
            .with_encoding(EncodingType::Uniform)
            .with_suffix(b"GENERATORS".to_vec())
            .build()
            .into();

        CG::hash_to_curve_custom::<H>(b"ADKG_GENERATOR_G", &dst)
    }
}
