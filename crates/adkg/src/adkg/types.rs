use crate::aba::AbaConfig;
use crate::aba::crain20::{AbaInput, CoinKeys};
use crate::aba::multi_aba::MultiAba;
use crate::helpers::{PartyId, SessionId, eval_poly};
use crate::pok::PokProof;
use crate::rbc::multi_rbc::MultiRbc;
use crate::rbc::{RbcPredicate, ReliableBroadcastConfig};
use crate::vss::acss::AcssConfig;
use crate::vss::acss::hbacss0::{FeldPublicPoly, PedPublicPoly};
use crate::vss::acss::multi_acss::MultiAcss;
use crate::vss::pedersen::PedersenPartyShare;
use ark_ec::CurveGroup;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::sync::futures::Notified;
use tracing::warn;
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

/// Input to Tyler Crain's ABA
pub type AbaCrainInput<CG> = AbaInput<LazyCoinKeys<CG>>;

/// Structure to evaluate ABA CoinKeys upon calling Into<CoinKeys>
pub struct LazyCoinKeys<CG: CurveGroup> {
    n: usize,
    t: usize,
    outputs: Vec<(SessionId, ShareWithPoly<CG>)>,
}

/// ACSS output required by ADKG.
#[derive(Clone)]
pub struct ShareWithPoly<CG: CurveGroup> {
    pub mvba_share: CG::ScalarField,
    pub mvba_public_poly: FeldPublicPoly<CG>,
    pub shares: Vec<PedersenPartyShare<CG::ScalarField>>,
    pub public_polys: Vec<PedPublicPoly<CG>>,
}

/// Predicate used by reliable broadcasts.
#[derive(Clone)]
pub(super) struct NotifyPredicate<S> {
    pub min_size: usize,
    pub expected_sender: PartyId,
    pub completed_acss: Arc<NotifyMap<S>>,
}

/// Helper struct used to (de)serialize RBC inputs/outputs.
#[derive(Serialize, Deserialize)]
pub(super) struct CompletedAcssSessions {
    pub v: HashSet<SessionId>,
}

/// Messages sent during the ADKG protocol.
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed, CG::ScalarField: FqSerialize",
    deserialize = "CG: PointDeserializeCompressed, CG::ScalarField: FqDeserialize",
))]
pub(super) enum AdkgMessage<CG: CurveGroup, H> {
    RandEx(AdkgRandExMessage<CG::ScalarField>),
    Key(AdkgKeyMessage<CG, H>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(serialize = "F: FqSerialize", deserialize = "F: FqDeserialize"))]
pub(super) struct AdkgRandExMessage<F> {
    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub z_j: F,

    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub z_hat_j: F,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "PokProof<CG, H>: Serialize, CG: PointSerializeCompressed",
    deserialize = "PokProof<CG, H>: Deserialize<'de>, CG: PointDeserializeCompressed"
))]
pub(super) struct AdkgKeyMessage<CG: CurveGroup, H> {
    pub z_j_proof: PokProof<CG, H>,
    pub z_hat_j_proof: PokProof<CG, H>,

    #[serde(with = "utils::serialize::point::base64")]
    pub g_z_j: CG,

    #[serde(with = "utils::serialize::point::base64")]
    pub h_z_hat_j: CG,
}

pub(super) struct SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>
where
    CG: CurveGroup,
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
    ABAConfig: AbaConfig<'static, PartyId>,
{
    pub id: PartyId,
    pub n: usize,
    pub t: usize,

    pub multi_rbc: tokio::sync::Mutex<MultiRbc<RBCConfig>>,
    pub multi_acss: tokio::sync::Mutex<MultiAcss<CG, ACSSConfig>>,
    pub multi_aba: tokio::sync::Mutex<MultiAba<ABAConfig>>,

    pub completed_acss_outputs: Arc<NotifyMap<ShareWithPoly<CG>>>,
    pub rbc_outputs: Arc<NotifyMap<HashSet<SessionId>>>,
}

pub(super) struct NotifyMap<S> {
    map: std::sync::Mutex<HashMap<SessionId, S>>,
    notify: Notify,
}

impl<CG: CurveGroup> LazyCoinKeys<CG> {
    pub fn new(n: usize, t: usize, outputs: Vec<(SessionId, ShareWithPoly<CG>)>) -> Self {
        Self { n, t, outputs }
    }
}

impl<CG: CurveGroup> From<LazyCoinKeys<CG>> for CoinKeys<CG> {
    fn from(val: LazyCoinKeys<CG>) -> Self {
        // Obtain the combined public polynomial as p_j = \sum_{k \in rbc_parties} p_k(x)
        // which is the sum of the MVBA public polynomial output by each ACSS specified in the j-th RBC
        let public_poly: Vec<CG> = (0..=val.t)
            .map(|i| {
                val.outputs
                    .iter()
                    .map(|(_, out)| out.mvba_public_poly.0[i])
                    .sum()
            })
            .collect();

        // Our own secret share, the sum of our ACSS MVBA shares
        // u_{i,j} = \sum_{k \in rbc_parties} s_{k,j} =
        let u_i_j: CG::ScalarField = val.outputs.iter().map(|(_, out)| out.mvba_share).sum();

        // Obtain commitments to the secret shares of the other parties
        // (g^{u_{1,j}}, ... g^{u_{n,j}}) = (g^p*(1), ..., g^p*(n))
        let g_u_is_j: Vec<CG> = PartyId::iter_all(val.n)
            .map(|i| eval_poly(&u64::from(i).into(), &public_poly))
            .collect();

        // Interpolate the group public key
        // g^{u_j} = g^{\sum_{k \in rbc_parties} s_k}
        let g_u_j: CG = eval_poly(&0u64.into(), &public_poly);

        CoinKeys {
            sk: u_i_j,
            vks: g_u_is_j,
            combined_vk: g_u_j,
        }
    }
}

#[async_trait]
impl<S: Send + Sync> RbcPredicate for NotifyPredicate<S> {
    async fn predicate(&self, sender: PartyId, m: &[u8]) -> bool {
        if sender != self.expected_sender {
            return false;
        }

        // Deserialize the rbc message into an RbcSet else return false
        let rbc_parties: CompletedAcssSessions = match bson::from_slice(m) {
            Ok(rbc_parties) => rbc_parties,
            Err(e) => {
                warn!(
                    "Message with invalid format rejected by the rbc predicate: bson error {e:?}"
                );
                return false;
            }
        };

        loop {
            // Register interest for notification prior to checking to not lose notifications
            let notified = self.completed_acss.notified();

            // Check that we have at least min_size parties, and that the completed ACSSs is a superset
            // of the parties in the message
            if rbc_parties.v.len() >= self.min_size
                && self.completed_acss.is_superset(&rbc_parties.v)
            {
                // true, accept the proposal
                return true;
            }

            // If not, wait for an update to completed_acss
            notified.await;
        }
    }
}

impl<S> NotifyMap<S> {
    pub fn new() -> Arc<Self> {
        Self {
            map: std::sync::Mutex::new(HashMap::new()),
            notify: Notify::new(),
        }
        .into()
    }

    pub fn notify(self: &Arc<Self>) {
        self.notify.notify_waiters()
    }

    pub fn notified(self: &Arc<Self>) -> Notified<'_> {
        self.notify.notified()
    }

    pub fn keys(self: &Arc<Self>) -> Vec<SessionId>
    where
        S: Clone,
    {
        self.map.lock().unwrap().keys().cloned().collect::<Vec<_>>()
    }

    pub fn filter_outputs<'a, I>(
        self: &'a Arc<Self>,
        keys: I,
    ) -> impl Iterator<Item = (SessionId, S)> + 'a
    where
        S: Clone,
        I: Iterator<Item = &'a SessionId> + 'a,
    {
        let completed_acss = self.map.lock().unwrap();
        keys.filter_map(move |k| completed_acss.get(k).cloned().map(|v| (*k, v)))
    }

    pub fn insert(self: &Arc<Self>, k: SessionId, share: S) {
        self.map
            .lock()
            .expect("a task panicked while holding the mutex")
            .insert(k, share);
        self.notify();
    }

    pub fn get(self: &Arc<Self>, k: &SessionId) -> Option<S>
    where
        S: Clone,
    {
        self.map
            .lock()
            .expect("a task panicked while holding the mutex")
            .get(k)
            .cloned()
    }

    pub fn is_superset(self: &Arc<Self>, other: &HashSet<SessionId>) -> bool {
        let keys: HashSet<_> = self
            .map
            .lock()
            .expect("a task panicked while holding the mutex")
            .keys()
            .cloned()
            .collect();

        keys.is_superset(other)
    }
}

impl<CG, RBCConfig, ACSSConfig, ABAConfig> SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>
where
    CG: CurveGroup,
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
    ABAConfig: AbaConfig<'static, PartyId>,
{
    pub fn new(
        id: PartyId,
        n: usize,
        t: usize,
        multi_rbc: MultiRbc<RBCConfig>,
        multi_acss: MultiAcss<CG, ACSSConfig>,
        multi_aba: MultiAba<ABAConfig>,
    ) -> Self {
        Self {
            id,
            n,
            t,
            multi_rbc: tokio::sync::Mutex::new(multi_rbc),
            multi_acss: tokio::sync::Mutex::new(multi_acss),
            multi_aba: tokio::sync::Mutex::new(multi_aba),
            completed_acss_outputs: NotifyMap::new(),
            rbc_outputs: NotifyMap::new(),
        }
    }
}
