use adkg::scheme::AdkgSchemeConfig;
use adkg::scheme::bls12_381::DXKR23Bls12_381G1Sha256;
use adkg::scheme::bn254::DXKR23Bn254G1Keccak256;
use ark_ec::PrimeGroup;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdkgCliSchemeConfig {
    #[serde(flatten)]
    pub adkg_config: AdkgSchemeConfig,
    pub adkg_scheme_name: String,
    pub output_generator: String,
}

pub fn new_scheme_config(
    scheme_id: SupportedAdkgScheme,
    app_name: String,
) -> anyhow::Result<AdkgCliSchemeConfig> {
    match scheme_id {
        SupportedAdkgScheme::DXKR23Bn254G1Keccak256 => {
            let scheme_config = DXKR23Bn254G1Keccak256::new(app_name).into();
            Ok(AdkgCliSchemeConfig {
                adkg_config: scheme_config,
                adkg_scheme_name: scheme_id.to_string(),
                output_generator: ark_bn254::G2Projective::generator().ser_compressed_base64()?,
            })
        }
        SupportedAdkgScheme::DXKR23Bls12_381G1Sha256 => {
            let scheme_config = DXKR23Bls12_381G1Sha256::new(app_name).into();
            Ok(AdkgCliSchemeConfig {
                adkg_config: scheme_config,
                adkg_scheme_name: scheme_id.to_string(),
                output_generator: ark_bls12_381::G2Projective::generator()
                    .ser_compressed_base64()?,
            })
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SupportedAdkgScheme {
    DXKR23Bn254G1Keccak256,
    DXKR23Bls12_381G1Sha256,
}

impl Display for SupportedAdkgScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SupportedAdkgScheme::DXKR23Bn254G1Keccak256 => f.write_str("DXKR23-Bn254G1-Keccak256"),

            SupportedAdkgScheme::DXKR23Bls12_381G1Sha256 => {
                f.write_str("DXKR23-Bls12_381G1-Sha256")
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
            "DXKR23-Bn254G1-Keccak256" => Ok(Self::DXKR23Bn254G1Keccak256),
            "DXKR23-Bls12_381G1-Sha256" => Ok(Self::DXKR23Bls12_381G1Sha256),
            _ => Err(UnsupportedAdkgScheme),
        }
    }
}
