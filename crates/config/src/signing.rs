use figment::Figment;
use figment::providers::{Format, Json, Toml};
use serde::{Deserialize, Serialize};
use serde_keys::Bn254SecretKey;
use shellexpand::tilde;
use std::num::NonZeroU16;
use std::path::PathBuf;

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

impl CommitteeConfig {
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let c: UnvalidatedCommitteeConfig = Figment::new()
            .merge(Toml::file(&path))
            .merge(Json::file(&path))
            .extract()?;
        c.parse()
    }
    pub fn from_path_str(path: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::from_path(PathBuf::from(tilde(&path).as_ref()))
    }
}

impl UnvalidatedCommitteeConfig {
    pub fn parse(mut self) -> anyhow::Result<CommitteeConfig> {
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
