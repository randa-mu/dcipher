use crate::adkg::{AdkgNodePk, AdkgPublic, AdkgSecret, GroupConfig};
use crate::keys::SecretKey;
use ark_ec::AffineRepr;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
use std::str::FromStr;
use utils::serialize::point::PointDeserializeCompressed;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Serialize, Deserialize, Debug, Clone)]
// Deserialize as a CommitteeConfigFiles first, and call try_from to get Self
#[serde(try_from = "CommitteeConfigFiles")]
#[serde(bound(
    serialize = "G: PointSerializeCompressed",
    deserialize = "G: PointDeserializeCompressed"
))]
pub struct CommitteeConfig<G: AffineRepr> {
    // this node's position in the `members` Vec
    pub member_id: NonZeroU16,

    // this node's keyshare created in the ADKG
    pub secret_key: SecretKey<G::ScalarField>,

    // count of nodes in the group. It should basically always be the same as `members.len()`
    pub n: NonZeroU16,

    // signing_threshold is number of partial signatures required to reconstruct a group signature
    pub signing_threshold: NonZeroU16,

    // all the details we need to contact and verify messages from other nodes
    pub members: Vec<MemberConfig<G>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitteeConfigFiles {
    pub adkg_public: AdkgPublic,
    pub adkg_secret: AdkgSecret,
    pub group: GroupConfig,
    pub member_id: NonZeroU16,
}

impl<G: AffineRepr + PointDeserializeCompressed> TryFrom<CommitteeConfigFiles>
    for CommitteeConfig<G>
{
    type Error = anyhow::Error;

    fn try_from(value: CommitteeConfigFiles) -> Result<Self, Self::Error> {
        let mut members = Vec::new();
        for node in value.adkg_public.node_pks {
            members.push(node.try_into()?);
        }

        // the `t_reconstruction` used in the DKG is actually an inverse of the
        // threshold we use in the signers, so we take it away from `n` to get
        // our usable threshold
        let n = value.group.n.get();
        let inverse_t = value.group.t_reconstruction.get();
        let threshold = n - inverse_t;
        if threshold < 1 {
            anyhow::bail!("can't have a signing threshold less than 1");
        }

        Ok(Self {
            member_id: value.member_id,
            secret_key: SecretKey::from_str(&value.adkg_secret.sk)?,
            n: value.group.n.try_into()?,
            signing_threshold: NonZeroU16::new(threshold as u16)
                .expect("we already check the value is greater than 0 above"),
            members,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(
    serialize = "G: PointSerializeCompressed",
    deserialize = "G: PointDeserializeCompressed"
))]
pub struct MemberConfig<G> {
    /// Node identifier used in the threshold scheme
    pub member_id: NonZeroU16,

    /// BN254 public key of the node
    #[serde(with = "utils::serialize::point::base64")]
    pub bls_pk: G,

    /// Libp2p peer address
    pub address: libp2p::Multiaddr,

    /// Peer id
    pub peer_id: libp2p::PeerId,
}

impl<G: AffineRepr + PointDeserializeCompressed> TryFrom<AdkgNodePk> for MemberConfig<G> {
    type Error = anyhow::Error;

    fn try_from(value: AdkgNodePk) -> Result<Self, Self::Error> {
        Ok(MemberConfig {
            member_id: value.id.try_into()?,
            peer_id: value.peer_id,
            address: value.multiaddr,
            bls_pk: PointDeserializeCompressed::deser_compressed_base64(&value.pk)?,
        })
    }
}
