use crate::scheme::{AdkgCliSchemeConfig, SupportedAdkgScheme};
use adkg::scheme::DXKR23AdkgScheme;
use adkg::scheme::bls12_381::DXKR23Bls12_381G1Sha256;
use adkg::scheme::bn254::DXKR23Bn254G1Keccak256;
use anyhow::Context;
use libp2p::{identity, PeerId};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use utils::serialize::fq::FqSerialize;
use utils::serialize::point::PointSerializeCompressed;
use config::adkg::{PrivateKeyMaterial, PublicKeyMaterial};

pub fn keygen(
    scheme_config: AdkgCliSchemeConfig,
) -> anyhow::Result<(PrivateKeyMaterial, PublicKeyMaterial)> {
    let libp2p_sk = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(libp2p_sk.public());
    match scheme_config
        .adkg_scheme_name
        .parse()
        .context("scheme is not supported")?
    {
        SupportedAdkgScheme::DXKR23Bn254G1Keccak256 => {
            let scheme = DXKR23Bn254G1Keccak256::try_from(scheme_config.adkg_config)?;
            let (adkg_sk, adkg_pk) = scheme.keygen(&mut thread_rng());
            let sk = PrivateKeyMaterial {
                adkg_sk: adkg_sk.ser_base64().expect("failed to serialize adkg sk"),
                libp2p_sk,
            };

            let pk = PublicKeyMaterial {
                adkg_pk: adkg_pk
                    .ser_compressed_base64()
                    .expect("failed to serialize adkg pk"),
                peer_id,
            };

            Ok((sk, pk))
        }

        SupportedAdkgScheme::DXKR23Bls12_381G1Sha256 => {
            let scheme = DXKR23Bls12_381G1Sha256::try_from(scheme_config.adkg_config)?;
            let (adkg_sk, adkg_pk) = scheme.keygen(&mut thread_rng());
            let sk = PrivateKeyMaterial {
                adkg_sk: adkg_sk.ser_base64().expect("failed to serialize adkg sk"),
                libp2p_sk,
            };

            let pk = PublicKeyMaterial {
                adkg_pk: adkg_pk
                    .ser_compressed_base64()
                    .expect("failed to serialize adkg pk"),
                peer_id,
            };

            Ok((sk, pk))
        }
    }
}

