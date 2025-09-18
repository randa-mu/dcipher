use crate::keys::libp2p_keypair_serde;
use libp2p::{Multiaddr, PeerId, identity};
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::str::FromStr;
use anyhow::{anyhow, Context};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeDetail {
    pub id: NonZeroUsize,
    #[serde(flatten)]
    pub public_key_material: PublicKeyMaterial,
    pub multiaddr: Multiaddr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupConfig {
    pub n: NonZeroUsize,
    pub t: NonZeroUsize,
    pub t_reconstruction: NonZeroUsize,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub nodes: Vec<NodeDetail>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AdkgSecret {
    pub adkg_scheme_name: String,
    pub genesis_timestamp: i64,
    pub sk: String,
}

/// The ADKG public output on a source group (i.e., the one used by the ADKG), and on a destination
/// group (i.e., the one used for signatures, etc.)
#[derive(Clone, Serialize, Deserialize)]
pub struct AdkgPublic {
    pub adkg_scheme_name: String,
    pub genesis_timestamp: i64,

    /// The destination group public key used for signatures
    pub group_pk: String,
    /// The destination node public keys used for signatures
    pub node_pks: Vec<AdkgNodePk>,

    /// The group public key obtained in the ADKG source group.
    #[serde(default)]
    pub group_pk_source: String,
    /// The node public keys obtained in the ADKG source group.
    #[serde(default)]
    pub node_pks_source: Vec<AdkgNodePk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdkgNodePk {
    pub id: NonZeroUsize,
    pub pk: String,
    pub peer_id: PeerId,
    pub multiaddr: Multiaddr,
}

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

impl FromStr for GroupConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut group_config: Self = toml::from_str(s).context("failed to parse group config")?;

        if group_config.n < group_config.t {
            Err(anyhow!("n cannot be smaller than t"))?;
        }

        if group_config.t_reconstruction < group_config.t {
            Err(anyhow!("reconstruction threshold cannot be smaller than t"))?;
        }

        if group_config.nodes.len() != group_config.n.get() {
            Err(anyhow!("number of nodes does not match n"))?;
        }

        if let Some(id) = group_config.nodes.iter().map(|n| n.id).duplicates().next() {
            Err(anyhow!("found node id {id} more than once"))?;
        }

        if let Some(peer_id) = group_config
            .nodes
            .iter()
            .map(|n| &n.public_key_material.peer_id)
            .duplicates()
            .next()
        {
            Err(anyhow!("found peer_id {peer_id} more than once"))?;
        }

        if let Some(adkg_pk) = group_config
            .nodes
            .iter()
            .map(|n| &n.public_key_material.adkg_pk)
            .duplicates()
            .next()
        {
            Err(anyhow!("found adkg_pk {adkg_pk} more than once"))?;
        }

        if let Some(multiaddr) = group_config
            .nodes
            .iter()
            .map(|n| &n.multiaddr)
            .duplicates()
            .next()
        {
            Err(anyhow!("found multiaddr {multiaddr} more than once"))?;
        }

        // Sort the nodes
        group_config.nodes.sort_by(|p1, p2| p1.id.cmp(&p2.id));

        Ok(group_config)
    }
}

impl GroupConfig {
    pub fn aligned_start_datetime(&self) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
        // Align the group config to a unix timestamp ending in 00
        let timestamp = self.start_time.timestamp();
        let timestamp_mod = timestamp % 100;
        let next_timestamp = if timestamp_mod == 0 {
            timestamp
        } else {
            timestamp + (100 - timestamp_mod)
        };

        chrono::DateTime::<chrono::Utc>::from_timestamp(next_timestamp, 0)
            .ok_or(anyhow!("failed to align unix timestamp"))
    }
}
