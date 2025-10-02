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
    pub member_id: NonZeroU16,
    pub secret_key: SecretKey<G::ScalarField>,
    pub t: NonZeroU16,
    pub n: NonZeroU16,
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

        Ok(Self {
            member_id: value.member_id,
            secret_key: SecretKey::from_str(&value.adkg_secret.sk)?,
            t: value.group.t.try_into()?,
            n: value.group.n.try_into()?,
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
