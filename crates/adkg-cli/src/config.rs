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
