use adkg::scheme::bls12_381::DYX22Bls12_381G1Sha256;
use adkg::scheme::bn254::DYX22Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use ark_ec::Group;
use std::fmt::Display;
use std::str::FromStr;

pub fn new_scheme_config(
    scheme_id: SupportedAdkgScheme,
    app_name: String,
) -> anyhow::Result<AdkgSchemeConfig> {
    match scheme_id {
        SupportedAdkgScheme::DYX22Bn254G1Keccak256 => {
            // We use h == Bn254 G1 as the generator for the group public key
            // and an independent generator g for the ADKG operations.
            let generator_h = ark_bn254::G1Projective::generator();
            let scheme_config = DYX22Bn254G1Keccak256::new(app_name, generator_h).into();
            Ok(scheme_config)
        }
        SupportedAdkgScheme::DYX22Bls12_381G1Sha256 => {
            // We use h == Bls12_381 G1 as the generator for the group public key
            // and an independent generator g for the ADKG operations.
            let generator_h = ark_bls12_381::G1Projective::generator();
            let scheme_config = DYX22Bls12_381G1Sha256::new(app_name, generator_h).into();
            Ok(scheme_config)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SupportedAdkgScheme {
    DYX22Bn254G1Keccak256,
    DYX22Bls12_381G1Sha256,
}

impl Display for SupportedAdkgScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SupportedAdkgScheme::DYX22Bn254G1Keccak256 => {
                f.write_str(<DYX22Bn254G1Keccak256 as AdkgScheme>::NAME)
            }

            SupportedAdkgScheme::DYX22Bls12_381G1Sha256 => {
                f.write_str(<DYX22Bls12_381G1Sha256 as AdkgScheme>::NAME)
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
            // Also allow legacy name that mistakenly used 20 instead of 22 for the publication
            // year
            "DYX20-Bn254G1-Keccak256" | <DYX22Bn254G1Keccak256 as AdkgScheme>::NAME => {
                Ok(Self::DYX22Bn254G1Keccak256)
            }

            <DYX22Bls12_381G1Sha256 as AdkgScheme>::NAME => Ok(Self::DYX22Bls12_381G1Sha256),
            _ => Err(UnsupportedAdkgScheme),
        }
    }
}
