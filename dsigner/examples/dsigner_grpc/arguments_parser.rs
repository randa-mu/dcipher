use anyhow::anyhow;
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use clap::Parser;
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use itertools::Itertools;
use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::num::{NonZeroU16, NonZeroUsize};
use std::path::PathBuf;
use utils::serialize::point::PointDeserializeCompressed;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Clone, Serialize, Deserialize)]
pub struct SchemesConfig {
    pub node_id: NonZeroU16,
    pub schemes: HashMap<String, SchemeConfigType>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SchemeConfigType {
    Bn254(BlsSchemeConfig<ark_bn254::Bn254>),
    Bls12_381(BlsSchemeConfig<ark_bls12_381::Bls12_381>),
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "BlsNodeConfig<E>: Serialize",
    deserialize = "BlsNodeConfig<E>: Deserialize<'de>"
))]
pub struct BlsSchemeConfig<E: Pairing> {
    pub sk: FpWrapper<E::ScalarField>,
    pub n: NonZeroU16,
    pub t: NonZeroU16,
    pub nodes: Vec<BlsNodeConfig<E>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound(
    serialize = "E::G1Affine: PointSerializeCompressed, E::G2Affine: PointSerializeCompressed",
    deserialize = "E::G1Affine: PointDeserializeCompressed, E::G2Affine: PointDeserializeCompressed"
))]
pub struct BlsNodeConfig<E: Pairing> {
    pub id: NonZeroU16,
    #[serde(with = "utils::serialize::point::base64")]
    pub pk_g1: E::G1Affine,
    #[serde(with = "utils::serialize::point::base64")]
    pub pk_g2: E::G2Affine,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    pub id: NonZeroU16,
    pub libp2p_listen_addr: Multiaddr,
    pub libp2p_key: Libp2pKeyWrapper,
    pub peers: Vec<NetworkPeer>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkPeer {
    pub id: NonZeroU16,
    pub peer_id: PeerId,
    pub multiaddr: Multiaddr,
}

/// Wrapper around Fp that allows deserialization from hex
pub struct FpWrapper<Fp>(pub Fp);

/// Wrapper around libp2p::identity::Keypair with (de)serialization & cmd line parsing.
#[derive(Clone, Debug)]
pub struct Libp2pKeyWrapper(pub ::libp2p::identity::Keypair);

/// dsigner arguments
#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The address to host HTTP server
    #[arg(long, env = "DSIGNER_LISTEN_ADDR", default_value = "0.0.0.0")]
    pub listen_addr: String,

    /// The port to host the health-check HTTP server
    #[arg(long, env = "DSIGNER_PORT", default_value = "8080")]
    pub port: u16,

    /// Schemes configuration file
    #[arg(long, env = "DSIGNER_SCHEMES_CONFIG")]
    pub schemes_config: PathBuf,

    /// Network configuration file
    #[arg(long, env = "DSIGNER_NETWORK_CONFIG")]
    pub network_config: PathBuf,

    /// The logging level for structured JSON logging
    /// Can be "info", "debug", "error", or "trace"
    #[arg(long, env = "DSIGNER_LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// LRU cache size used for partial signatures and signatures
    #[arg(long, env = "DSIGNER_LRU_CACHE_SIZE", default_value = "64")]
    pub lru_cache_size: NonZeroUsize,
}

pub struct DSignerConfig {
    pub config: Args,
    pub network_config: NetworkConfig,
    pub schemes_config: SchemesConfig,
}

impl DSignerConfig {
    pub fn parse() -> anyhow::Result<Self> {
        let c: Args = Figment::new()
            .merge(Serialized::defaults(Args::parse()))
            .merge(Toml::file("config.toml"))
            .extract()?;

        let network_config = Self::parse_network_config(&c)?;
        let schemes_config = Self::parse_schemes_config(&c)?;

        Ok(Self {
            config: c,
            schemes_config,
            network_config,
        })
    }

    fn parse_network_config(config: &Args) -> anyhow::Result<NetworkConfig> {
        let mut network_config: NetworkConfig = Figment::new()
            .merge(Toml::file(&config.network_config))
            .extract()?;

        let peers_len = network_config.peers.len();
        network_config.peers = network_config
            .peers
            .into_iter()
            .sorted_by(|a, b| a.id.cmp(&b.id))
            .unique_by(|a| a.id)
            .collect();
        if network_config.peers.len() != peers_len {
            Err(anyhow!(
                "network peer configuration contains duplicated peers"
            ))?
        }

        Ok(network_config)
    }

    fn parse_schemes_config(config: &Args) -> anyhow::Result<SchemesConfig> {
        let mut schemes_config: SchemesConfig = Figment::new()
            .merge(Toml::file(&config.schemes_config))
            .extract()?;

        schemes_config.schemes = schemes_config
            .schemes
            .into_iter()
            .map(|(scheme_id, scheme)| -> anyhow::Result<_> {
                let scheme = match scheme {
                    SchemeConfigType::Bn254(scheme) => {
                        SchemeConfigType::Bn254(Self::parse_bls_scheme(scheme)?)
                    }
                    SchemeConfigType::Bls12_381(scheme) => {
                        SchemeConfigType::Bls12_381(Self::parse_bls_scheme(scheme)?)
                    }
                };

                Ok((scheme_id, scheme))
            })
            .collect::<Result<_, _>>()?;

        Ok(schemes_config)
    }

    fn parse_bls_scheme<E: Pairing>(
        mut scheme: BlsSchemeConfig<E>,
    ) -> anyhow::Result<BlsSchemeConfig<E>> {
        scheme.nodes = scheme
            .nodes
            .into_iter()
            .sorted_by(|a, b| a.id.cmp(&b.id))
            .unique_by(|a| a.id)
            .collect();

        if scheme.nodes.len() != scheme.n.get() as usize {
            Err(anyhow!(
                "number of nodes does not match scheme's number of nodes"
            ))?
        }

        if scheme.t > scheme.n {
            Err(anyhow!("t cannot be greater than n"))?
        }

        Ok(scheme)
    }
}

impl<Fp: std::fmt::Debug> std::fmt::Debug for FpWrapper<Fp> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<Fp: Clone> Clone for FpWrapper<Fp> {
    fn clone(&self) -> Self {
        FpWrapper(self.0.clone())
    }
}

impl<Fp: PrimeField> Serialize for FpWrapper<Fp> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.into_bigint().to_bytes_be();
        serializer.serialize_str(&format!("0x{}", hex::encode(&bytes)))
    }
}

impl<'de, Fp: PrimeField> Deserialize<'de> for FpWrapper<Fp> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let hex_str = String::deserialize(deserializer)?;
        if &hex_str[0..2] != "0x" {
            Err(D::Error::custom("invalid hex string"))?
        }

        let bytes = hex::decode(&hex_str[2..]).map_err(D::Error::custom)?;
        Ok(FpWrapper(Fp::from_be_bytes_mod_order(&bytes)))
    }
}

impl From<Libp2pKeyWrapper> for ::libp2p::identity::Keypair {
    fn from(value: Libp2pKeyWrapper) -> Self {
        value.0
    }
}

impl Serialize for Libp2pKeyWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use base64::prelude::*;
        use serde::ser::Error;

        let bytes = self.0.to_protobuf_encoding().map_err(S::Error::custom)?;
        let encoded = BASE64_STANDARD.encode(&bytes);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for Libp2pKeyWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use base64::prelude::*;
        use serde::de::Error;

        let base64_str = String::deserialize(deserializer)?;
        let bytes = BASE64_STANDARD
            .decode(&base64_str)
            .map_err(D::Error::custom)?;
        Ok(Libp2pKeyWrapper(
            ::libp2p::identity::Keypair::from_protobuf_encoding(&bytes)
                .map_err(D::Error::custom)?,
        ))
    }
}
