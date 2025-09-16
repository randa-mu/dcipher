use crate::keys::Bn254SecretKey;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitteeConfig {
    pub member_id: NonZeroU16,
    pub secret_key: Bn254SecretKey,
    pub t: NonZeroU16,
    pub n: NonZeroU16,
    pub members: Vec<MemberConfig>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnvalidatedCommitteeConfig {
    pub member_id: NonZeroU16,
    pub secret_key: Bn254SecretKey,
    pub t: NonZeroU16,
    pub n: NonZeroU16,
    pub members: Vec<MemberConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberConfig {
    /// Node identifier used in the threshold scheme
    pub member_id: NonZeroU16,

    /// BN254 public key of the node
    #[serde(with = "utils::serialize::point::base64")]
    pub bls_pk: ark_bn254::G2Affine,

    /// Libp2p peer address
    pub address: libp2p::Multiaddr,

    /// Peer id
    pub peer_id: libp2p::PeerId,
}
