use alloy::consensus::private::serde::{Deserialize, Serialize};
use alloy::primitives::FixedBytes;
use ark_bn254::G2Affine;
use config::adkg::{AdkgPublic, AdkgSecret, GroupConfig, PrivateKeyMaterial};
use config::agent::AgentConfig;
use config::cli::FileArg;
use config::network::NetworkConfig;
use config::signing::{CommitteeConfig, CommitteeConfigFiles};
use libp2p::Multiaddr;
use omnievent::proto_types::BlockSafety;
use serde::Deserializer;
use serde::de::Error;
use std::fs;
use std::num::NonZeroU16;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfigFile {
    #[serde(default)]
    pub agent: AgentConfig,
    #[serde(default)]
    pub timeout: TimeoutConfig,
    pub networks: Vec<NetworkConfig>,
    pub longterm_secret_path: FileArg<PrivateKeyMaterial>,
    pub adkg_public_path: FileArg<AdkgPublic>,
    pub adkg_secret_path: FileArg<AdkgSecret>,
    pub group_path: FileArg<GroupConfig>,
    pub eth_private_key: EthPrivateKey,
    pub member_id: NonZeroU16,
    pub listen_addr: Multiaddr,
}

// an enum representing a config object that accepts either:
// a 0x-prefixed hex encoded ethereum private key
// a path to a file containing a 0x-prefixed hex encoded ethereum private key
#[derive(Debug, Clone)]
pub(crate) enum EthPrivateKey {
    Path(PathBuf),
    Value(FixedBytes<32>),
}

impl<'de> Deserialize<'de> for EthPrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("0x") {
            Ok(EthPrivateKey::Value(
                FixedBytes::from_str(&s).map_err(D::Error::custom)?,
            ))
        } else {
            Ok(EthPrivateKey::Path(
                PathBuf::from_str(&s).map_err(D::Error::custom)?,
            ))
        }
    }
}

impl TryInto<FixedBytes<32>> for EthPrivateKey {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FixedBytes<32>, Self::Error> {
        match self {
            EthPrivateKey::Path(path) => {
                let contents = fs::read_to_string(path)?;
                Ok(FixedBytes::from_str(&contents.trim())?)
            }
            EthPrivateKey::Value(s) => Ok(s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub agent: AgentConfig,
    #[serde(default)]
    pub timeout: TimeoutConfig,
    pub networks: Vec<NetworkConfig>,
    pub committee_config: CommitteeConfig<G2Affine>,
    pub eth_private_key: FixedBytes<32>,
    pub listen_addr: Multiaddr,
    pub longterm_secret: PrivateKeyMaterial,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeoutConfig {
    #[serde(default = "default_block_safety")]
    pub block_safety: BlockSafety,
    #[serde(with = "humantime_serde", default = "default_request_timeout")]
    pub request_timeout: Duration,
    #[serde(with = "humantime_serde", default = "default_retry_duration")]
    pub retry_duration: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        TimeoutConfig {
            block_safety: default_block_safety(),
            request_timeout: default_request_timeout(),
            retry_duration: default_retry_duration(),
        }
    }
}

const fn default_block_safety() -> BlockSafety {
    BlockSafety::Safe
}
const fn default_request_timeout() -> Duration {
    Duration::from_secs(30)
}

const fn default_retry_duration() -> Duration {
    Duration::from_secs(12)
}
impl TryFrom<AppConfigFile> for AppConfig {
    type Error = anyhow::Error;

    fn try_from(file: AppConfigFile) -> Result<Self, Self::Error> {
        let longterm_secret = file.longterm_secret_path.0;
        let eth_private_key = file.eth_private_key.try_into()?;
        let committee_config = CommitteeConfigFiles {
            adkg_public: file.adkg_public_path.0,
            adkg_secret: file.adkg_secret_path.0,
            group: file.group_path.0,
            member_id: file.member_id,
        }
        .try_into()?;

        Ok(AppConfig {
            agent: file.agent,
            networks: file.networks,
            listen_addr: file.listen_addr,
            longterm_secret,
            eth_private_key,
            committee_config,
            timeout: file.timeout,
        })
    }
}
