//! Default schemes configurations that can be used for the ADKG.

use crate::aba::AbaConfig;
use crate::aba::crain20::{AbaCrain20Config, EcdhCoinToss};
use crate::adkg::Adkg;
use crate::adkg::types::LazyCoinKeys;
use crate::helpers::PartyId;
use crate::network::RetryStrategy;
use crate::rbc::ReliableBroadcastConfig;
use crate::rbc::r4::Rbc4RoundsConfig;
use crate::vss::acss::AcssConfig;
use crate::vss::acss::hbacss0::{HbAcss0Config, Hbacss0Input};
use ark_ec::{CurveGroup, PrimeGroup};
use ark_std::UniformRand;
use digest::core_api::BlockSizeUser;
use digest::{DynDigest, FixedOutputReset};
use rand::{CryptoRng, Rng};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use utils::dst::{CurveId, HashId, NamedCurveGroup, NamedDynDigest};
use utils::dst::{EncodingType, Rfc9380DstBuilder};
use utils::hash_to_curve::HashToCurve;
use utils::serialize::SerializationError;
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

pub const ADKG_VERSION: &str = "v0.1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdkgSchemeConfig {
    app_name: String,
    curve_id: CurveId,
    hash_id: HashId,
    adkg_version: String,
    generator_g: String,
    generator_h: String,
}

pub trait DXKR23AdkgScheme: Send + Sync + 'static
where
    <Self::Curve as PrimeGroup>::ScalarField: FqSerialize + FqDeserialize,
{
    type Error: std::error::Error + Send + Sync + 'static;
    type Curve: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve;
    type Hash: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone;

    type RBCConfig: ReliableBroadcastConfig<'static, PartyId>;
    type ACSSConfig: AcssConfig<
            'static,
            Self::Curve,
            PartyId,
            Input = Hbacss0Input<<Self::Curve as PrimeGroup>::ScalarField>,
        >;
    type ABAConfig: AbaConfig<'static, PartyId>;

    fn generator_g(&self) -> Self::Curve;
    fn generator_h(&self) -> Self::Curve;

    fn keygen(
        &self,
        rng: &mut (impl Rng + CryptoRng),
    ) -> (<Self::Curve as PrimeGroup>::ScalarField, Self::Curve) {
        let sk = <Self::Curve as PrimeGroup>::ScalarField::rand(rng);
        let pk = self.generator_g() * sk;
        (sk, pk)
    }

    #[allow(clippy::type_complexity)]
    fn new_adkg(
        &self,
        id: PartyId,
        n: NonZeroUsize,
        t: NonZeroUsize,
        t_reconstruction: NonZeroUsize,
        sk: <Self::Curve as PrimeGroup>::ScalarField,
        pks: Vec<Self::Curve>,
    ) -> Result<
        Adkg<Self::Curve, Self::Hash, Self::RBCConfig, Self::ACSSConfig, Self::ABAConfig>,
        Self::Error,
    >
    where
        Self::Curve: NamedCurveGroup,
        Self::Hash: NamedDynDigest;
}

#[derive(thiserror::Error, Debug)]
pub enum SchemeError {
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

    #[error("reconstruction threshold not in [t, n - t - 1] = [{0}, {1}]")]
    ReconstructionThresholdNotInRange(usize, usize),

    #[error("failed to deserialize point")]
    PointDeserialize(#[from] SerializationError),
}

pub struct DXKR23Scheme<CG, H> {
    app_name: String,
    _cg: PhantomData<CG>,
    _h: PhantomData<H>,
}

impl<CG, H> DXKR23Scheme<CG, H> {
    pub fn new(app_name: String) -> Self {
        Self {
            app_name,
            _cg: PhantomData,
            _h: PhantomData,
        }
    }
}

impl<CG, H> DXKR23AdkgScheme for DXKR23Scheme<CG, H>
where
    CG: NamedCurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve,
    CG::ScalarField: FqSerialize + FqDeserialize,
    H: Default + NamedDynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
{
    type Error = SchemeError;
    type Curve = CG;
    type Hash = H;

    type RBCConfig = Rbc4RoundsConfig;
    type ACSSConfig = HbAcss0Config<Self::Curve, Self::Hash, Self::RBCConfig>;
    type ABAConfig =
        AbaCrain20Config<EcdhCoinToss<Self::Curve, Self::Hash>, LazyCoinKeys<Self::Curve>>;

    fn generator_g(&self) -> Self::Curve {
        let app_name = format!("ADKG-{ADKG_VERSION}-{}", self.app_name)
            .as_bytes()
            .to_owned();
        get_generator::<_, Self::Hash>(app_name, b"ADKG_GENERATOR_G")
    }

    fn generator_h(&self) -> Self::Curve {
        let app_name = format!("ADKG-{ADKG_VERSION}-{}", self.app_name)
            .as_bytes()
            .to_owned();
        get_generator::<_, Self::Hash>(app_name, b"ADKG_GENERATOR_H")
    }

    fn new_adkg(
        &self,
        id: PartyId,
        n: NonZeroUsize,
        t: NonZeroUsize,
        t_reconstruction: NonZeroUsize,
        sk: <Self::Curve as PrimeGroup>::ScalarField,
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
        let t_reconstruction = t_reconstruction.get();
        let max_t = (n - 1) / 3;
        if t > max_t {
            Err(Self::Error::ThresholdTooHigh(max_t))?
        }

        let max_t_reconstruction = n - t - 1;
        if t_reconstruction < t || t_reconstruction > max_t_reconstruction {
            Err(Self::Error::ReconstructionThresholdNotInRange(
                t,
                max_t_reconstruction,
            ))?
        }

        let retry_strategy = RetryStrategy::None;
        let generator_g = self.generator_g();
        let generator_h = self.generator_h();

        let rbc_config = Rbc4RoundsConfig::new(id, n, t, &retry_strategy);
        let acss_config = HbAcss0Config::<_, Self::Hash, _>::new(
            id,
            sk,
            pks,
            rbc_config.clone(),
            n,
            t,
            generator_g,
            generator_h,
            retry_strategy,
        );
        let aba_config = AbaCrain20Config::<_, _>::new(id, n, t, retry_strategy);

        Ok(Adkg::new(
            id,
            n,
            t,
            t_reconstruction,
            generator_g,
            generator_h,
            rbc_config,
            acss_config,
            aba_config,
        ))
    }
}

fn get_generator<CG, H>(app_name: Vec<u8>, generator_name: &[u8]) -> CG
where
    CG: NamedCurveGroup + HashToCurve,
    H: Default + NamedDynDigest + FixedOutputReset + BlockSizeUser + Clone,
{
    let dst: Vec<_> = Rfc9380DstBuilder::empty()
        .with_application_name(app_name)
        .with_curve::<CG>()
        .with_hash::<H>()
        .with_mapping(CG::CURVE_ID.default_mapping())
        .with_encoding(EncodingType::Uniform)
        .with_suffix(b"GENERATORS".to_vec())
        .build()
        .into();
    CG::hash_to_curve_custom::<H>(generator_name, &dst)
}

impl<CG, H> TryFrom<AdkgSchemeConfig> for DXKR23Scheme<CG, H>
where
    CG: NamedCurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve,
    CG::ScalarField: FqSerialize + FqDeserialize,
    H: Default + NamedDynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
{
    type Error = SchemeError;

    fn try_from(value: AdkgSchemeConfig) -> Result<Self, Self::Error> {
        if value.adkg_version != ADKG_VERSION {
            Err(Self::Error::UnsupportedAdkgVersion)?
        }

        if value.curve_id != <<Self as DXKR23AdkgScheme>::Curve as NamedCurveGroup>::CURVE_ID {
            Err(Self::Error::BadCurveId)?
        }

        if value.hash_id != <<Self as DXKR23AdkgScheme>::Hash as NamedDynDigest>::HASH_ID {
            Err(Self::Error::BadHashId)?
        }

        let generator_g =
            <Self as DXKR23AdkgScheme>::Curve::deser_compressed_base64(&value.generator_g)?;
        let generator_h =
            <Self as DXKR23AdkgScheme>::Curve::deser_compressed_base64(&value.generator_h)?;
        let scheme = Self::new(value.app_name);

        // Make sure that the generator corresponds to the dynamically computed generator
        if generator_g != scheme.generator_g() {
            Err(Self::Error::BadGenerator)?
        }

        if generator_h != scheme.generator_h() {
            Err(Self::Error::BadGenerator)?
        }

        Ok(scheme)
    }
}

impl<CG, H> From<DXKR23Scheme<CG, H>> for AdkgSchemeConfig
where
    CG: NamedCurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve,
    CG::ScalarField: FqSerialize + FqDeserialize,
    H: Default + NamedDynDigest + FixedOutputReset + BlockSizeUser + Clone + Send + Sync + 'static,
{
    fn from(value: DXKR23Scheme<CG, H>) -> Self {
        let generator_g = value
            .generator_g()
            .ser_compressed_base64()
            .expect("failed to serialize point to base64");
        let generator_h = value
            .generator_h()
            .ser_compressed_base64()
            .expect("failed to serialize point to base64");
        Self {
            app_name: value.app_name,
            adkg_version: ADKG_VERSION.to_owned(),
            generator_g,
            generator_h,
            curve_id:
                <<DXKR23Scheme<CG, H> as DXKR23AdkgScheme>::Curve as NamedCurveGroup>::CURVE_ID,
            hash_id: <<DXKR23Scheme<CG, H> as DXKR23AdkgScheme>::Hash as NamedDynDigest>::HASH_ID,
        }
    }
}

#[cfg(feature = "bn254")]
pub mod bn254 {
    use super::*;

    pub type DXKR23Bn254G1Keccak256 = DXKR23Scheme<ark_bn254::G1Projective, sha3::Keccak256>;
}

#[cfg(feature = "bls12-381")]
pub mod bls12_381 {
    use super::*;

    pub type DXKR23Bls12_381G1Sha256 = DXKR23Scheme<ark_bls12_381::G1Projective, sha2::Sha256>;
}
