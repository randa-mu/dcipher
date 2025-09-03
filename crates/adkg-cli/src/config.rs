use crate::keygen::PublicKeyMaterial;
use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

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

#[derive(Clone, Serialize, Deserialize)]
pub struct AdkgPublic {
    pub adkg_scheme_name: String,
    pub genesis_timestamp: i64,
    pub group_pk: String,
    pub node_pks: Vec<AdkgNodePk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdkgNodePk {
    pub id: NonZeroUsize,
    pub pk: String,
    pub peer_id: PeerId,
    pub multiaddr: Multiaddr,
}
