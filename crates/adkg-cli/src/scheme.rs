use adkg::scheme::bls12_381::DXK23Bls12_381G1Sha256;
use adkg::scheme::bn254::DXK23Bn254G1Keccak256;
use adkg::scheme::{AdkgSchemeConfig, DXK23AdkgScheme};
use ark_ec::Group;
use std::fmt::Display;
use std::str::FromStr;

pub fn new_scheme_config(
    scheme_id: SupportedAdkgScheme,
    app_name: String,
) -> anyhow::Result<AdkgSchemeConfig> {
    match scheme_id {
        SupportedAdkgScheme::DXK23Bn254G1Keccak256 => {
            // We use h == Bn254 G1 as the generator for the group public key
            // and an independent generator g for the ADKG operations.
            let generator_h = ark_bn254::G1Projective::generator();
            let scheme_config = DXK23Bn254G1Keccak256::new(app_name, generator_h).into();
            Ok(scheme_config)
        }
        SupportedAdkgScheme::DXK23Bls12_381G1Sha256 => {
            // We use h == Bls12_381 G1 as the generator for the group public key
            // and an independent generator g for the ADKG operations.
            let generator_h = ark_bls12_381::G1Projective::generator();
            let scheme_config = DXK23Bls12_381G1Sha256::new(app_name, generator_h).into();
            Ok(scheme_config)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SupportedAdkgScheme {
    DXK23Bn254G1Keccak256,
    DXK23Bls12_381G1Sha256,
}

impl Display for SupportedAdkgScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SupportedAdkgScheme::DXK23Bn254G1Keccak256 => {
                f.write_str(<DXK23Bn254G1Keccak256 as DXK23AdkgScheme>::NAME)
            }

            SupportedAdkgScheme::DXK23Bls12_381G1Sha256 => {
                f.write_str(<DXK23Bls12_381G1Sha256 as DXK23AdkgScheme>::NAME)
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("unsupported scheme")]
pub struct UnsupportedAdkgScheme;

impl FromStr for SupportedAdkgScheme {
    type Err = UnsupportedAdkgScheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            <DXK23Bn254G1Keccak256 as DXK23AdkgScheme>::NAME => Ok(Self::DXK23Bn254G1Keccak256),
            <DXK23Bls12_381G1Sha256 as DXK23AdkgScheme>::NAME => Ok(Self::DXK23Bls12_381G1Sha256),
            _ => Err(UnsupportedAdkgScheme),
        }
    }
}
