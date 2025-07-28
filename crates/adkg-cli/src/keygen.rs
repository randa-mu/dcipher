use adkg::scheme::bn254::DYX20Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use anyhow::anyhow;
use libp2p::{PeerId, identity};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use utils::serialize::fq::FqSerialize;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateKeyMaterial {
    pub adkg_sk: String,
    #[serde(with = "libp2p_keypair_serde")]
    pub libp2p_sk: identity::Keypair,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKeyMaterial {
    pub adkg_pk: String,
    pub peer_id: PeerId,
}

pub fn keygen(
    scheme_config: AdkgSchemeConfig,
) -> anyhow::Result<(PrivateKeyMaterial, PublicKeyMaterial)> {
    let libp2p_sk = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(libp2p_sk.public());
    match scheme_config.adkg_scheme_name.as_str() {
        DYX20Bn254G1Keccak256::NAME => {
            let scheme = DYX20Bn254G1Keccak256::try_from(scheme_config)?;
            let (adkg_sk, adkg_pk) = scheme.keygen(&mut thread_rng());
            let sk = PrivateKeyMaterial {
                adkg_sk: adkg_sk.ser_base64().expect("failed to serialize adkg sk"),
                libp2p_sk,
            };

            let pk = PublicKeyMaterial {
                adkg_pk: adkg_pk.ser_base64().expect("failed to serialize adkg pk"),
                peer_id,
            };

            Ok((sk, pk))
        }
        _ => Err(anyhow!("scheme is not supported")),
    }
}

mod libp2p_keypair_serde {
    use base64::prelude::*;
    use libp2p::identity;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(v: &identity::Keypair, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;

        let bytes = v.to_protobuf_encoding().map_err(Error::custom)?;
        s.serialize_str(&BASE64_STANDARD.encode(&bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<identity::Keypair, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let base64_str = String::deserialize(deserializer)?;
        let bytes = BASE64_STANDARD.decode(base64_str).map_err(Error::custom)?;
        identity::Keypair::from_protobuf_encoding(&bytes).map_err(Error::custom)
    }
}
