use adkg::scheme::bn254::DYX22Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use ark_ec::Group;
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
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SupportedAdkgScheme {
    DYX22Bn254G1Keccak256,
}

#[derive(thiserror::Error, Debug)]
#[error("unsupported scheme")]
pub struct UnsupportedAdkgScheme;

impl FromStr for SupportedAdkgScheme {
    type Err = UnsupportedAdkgScheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            <DYX22Bn254G1Keccak256 as AdkgScheme>::NAME => Ok(Self::DYX22Bn254G1Keccak256),
            _ => Err(UnsupportedAdkgScheme),
        }
    }
}
