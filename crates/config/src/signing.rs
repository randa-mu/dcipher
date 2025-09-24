use crate::keys::SecretKey;
use ark_ec::AffineRepr;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
use utils::serialize::point::PointDeserializeCompressed;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Serialize, Deserialize, Debug, Clone)]
// Deserialize as a UnvalidatedCommitteeConfig first, and call try_from to get Self
#[serde(try_from = "UnvalidatedCommitteeConfig<G>")]
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
#[serde(bound(
    serialize = "G: PointSerializeCompressed",
    deserialize = "G: PointDeserializeCompressed"
))]
pub struct UnvalidatedCommitteeConfig<G: AffineRepr> {
    pub member_id: NonZeroU16,
    pub secret_key: SecretKey<G::ScalarField>,
    pub t: NonZeroU16,
    pub n: NonZeroU16,
    pub members: Vec<MemberConfig<G>>,
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

impl<G: AffineRepr> UnvalidatedCommitteeConfig<G> {
    pub fn parse(mut self) -> anyhow::Result<CommitteeConfig<G>> {
        let member_count = self.members.len();
        let n = self.n.get() as usize;
        let t = self.t.get() as usize;
        if member_count == 0 || n == 0 || t == 0 {
            anyhow::bail!("a committee must have members and a non-zero threshold");
        }
        if t > n {
            anyhow::bail!("threshold cannot be larger than the committee size");
        }
        if member_count != n {
            anyhow::bail!("the n must match the number of members of the committee")
        }

        // sort them to simplify things in threshold-land
        self.members.sort_by(|a, b| a.member_id.cmp(&b.member_id));

        // Verify that each node's index is valid
        if !self
            .members
            .iter()
            .all(|n| n.member_id.get() <= member_count as u16)
        {
            anyhow::bail!("node with index greater than n")
        }

        // Verify that each node's index is unique
        let mut unique_ids: Vec<_> = self.members.iter().map(|n| n.member_id).collect();
        unique_ids.dedup(); // vec is already sorted, can simply dedup
        if unique_ids.len() != n {
            anyhow::bail!("committee cannot contain duplicate members")
        }

        // return the config including our modified set of nodes (excluding ours)
        Ok(CommitteeConfig {
            member_id: self.member_id,
            secret_key: self.secret_key,
            n: self.n,
            t: self.t,
            members: self.members,
        })
    }
}

impl<G: AffineRepr> TryFrom<UnvalidatedCommitteeConfig<G>> for CommitteeConfig<G> {
    type Error = anyhow::Error;

    fn try_from(value: UnvalidatedCommitteeConfig<G>) -> Result<Self, Self::Error> {
        value.parse()
    }
}
