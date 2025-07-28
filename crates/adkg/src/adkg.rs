pub(crate) mod types;

/// Re-export types required by ADKG trait bounds
pub use types::AbaCrainInput;
pub use types::ShareWithPoly;

use crate::aba::crain20::AbaInput;
use crate::aba::multi_aba::MultiAba;
use crate::aba::{AbaConfig, Estimate};
use crate::helpers::{
    PartyId, SessionId, eval_poly, lagrange_points_interpolate_at, u64_from_usize,
};
use crate::network::{RetryStrategy, broadcast_with_self};
use crate::nizk::{NIZKDleqProof, NizkError};
use crate::rand::{AdkgRng, AdkgRngType};
use crate::rbc::ReliableBroadcastConfig;
use crate::rbc::multi_rbc::MultiRbc;
use crate::vss::acss::AcssConfig;
use crate::vss::acss::multi_acss::MultiAcss;
use ark_ec::CurveGroup;
use ark_std::UniformRand;
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::{ReceivedMessage, Transport};
use digest::DynDigest;
use digest::core_api::BlockSizeUser;
use futures::StreamExt;
use rand::rngs::OsRng;
use std::collections::{BTreeMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::oneshot;
use tokio::task;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use types::{
    AdkgMessage, AdkgPublicKeyMessage, CompletedAcssSessions, LazyCoinKeys, NotifyMap,
    NotifyPredicate, SharedState,
};
use utils::dst::{NamedCurveGroup, NamedDynDigest, Rfc9380DstBuilder};
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

const APPNAME: &[u8] = b"ADKG-v1";
const NIZK_DLEQ_SUFFIX: &[u8] = b"NIZK_DLEQ";

pub struct Adkg<CG, H, RBCConfig, ACSSConfig, ABAConfig>
where
    CG: CurveGroup,
    H: DynDigest,
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
    ABAConfig: AbaConfig<'static, PartyId>,
{
    id: PartyId,
    n: usize,
    t: usize,
    g: CG,
    h: CG,
    nizk_dleq_dst: Vec<u8>,
    cancel: CancellationToken,

    rbc_task: Option<JoinHandle<()>>,
    acss_task: Option<JoinHandle<()>>,

    shared_state: Arc<SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>>,

    _h: PhantomData<fn(H) -> H>,
}

#[derive(Error, Debug)]
pub enum AdkgError {
    #[error("ABA waiting task cancelled early")]
    AbaCancelledEarly,

    #[error("failed to initialize adkg transport")]
    TransportInit,

    #[error("nizk proof error: `{1}`")]
    Nizk(#[source] NizkError, &'static str),

    #[error("failed to serialize bson: `{1}`")]
    BsonSer(#[source] bson::ser::Error, &'static str),

    #[error("failed to serialize bson: `{1}`")]
    BsonDe(#[source] bson::de::Error, &'static str),

    #[error("rng error: `{1}`")]
    Rng(
        #[source] Box<dyn std::error::Error + Send + Sync + 'static>,
        &'static str,
    ),

    #[error("failed to derive group public key")]
    DeriveGroupPublicKey,
}

pub struct AdkgOutput<CG: CurveGroup> {
    pub sk: CG::ScalarField,
    pub used_sessions: Vec<SessionId>,
    pub group_pk: Option<CG>,
    pub node_pks: Option<Vec<CG>>,
}

impl<CG, H, RBCConfig, ACSSConfig, ABAConfig> Adkg<CG, H, RBCConfig, ACSSConfig, ABAConfig>
where
    CG: NamedCurveGroup,
    H: NamedDynDigest,
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
    ABAConfig: AbaConfig<'static, PartyId>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: PartyId,
        n: usize,
        t: usize,
        g: CG,
        h: CG,
        rbc_config: Arc<RBCConfig>,
        acss_config: Arc<ACSSConfig>,
        aba_config: Arc<ABAConfig>,
    ) -> Self {
        let multi_rbc = MultiRbc::new(id, n, rbc_config);
        let multi_acss = MultiAcss::new(id, n, acss_config);
        let multi_aba = MultiAba::new(id, n, aba_config);

        // Generate a DST in the following format: ADKG-v1_%CURVE_NAME%_XMD:%HASH_NAME%_RO_NIZK_DLEQ_
        // e.g.: ADKG-v1_BN254G1_XMD:SHA3-256_RO_NIZK_DLEQ_
        let dst = Rfc9380DstBuilder::empty()
            .with_application_name(APPNAME.to_vec())
            .with_curve::<CG>()
            .with_hash::<H>()
            .with_suffix(NIZK_DLEQ_SUFFIX.to_vec())
            .build();

        Self {
            id,
            n,
            t,
            g,
            h,
            cancel: CancellationToken::new(),
            rbc_task: None,
            acss_task: None,
            shared_state: SharedState::new(id, n, t, multi_rbc, multi_acss, multi_aba).into(),
            nizk_dleq_dst: dst.into(),
            _h: PhantomData,
        }
    }
}

impl<CG, H, RBCConfig, ACSSConfig, ABAConfig> Adkg<CG, H, RBCConfig, ACSSConfig, ABAConfig>
where
    CG: CurveGroup
        + PointSerializeCompressed
        + PointDeserializeCompressed
        + utils::hash_to_curve::HashToCurve,
    CG::ScalarField: FqSerialize + FqDeserialize,
    H: Default + DynDigest + BlockSizeUser + Clone + 'static,
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
    ACSSConfig::Output: Sized + Clone + Into<ShareWithPoly<CG>>,
    ABAConfig: AbaConfig<'static, PartyId, Input = AbaCrainInput<CG>>,
{
    pub async fn start<T>(
        &mut self,
        rng: &mut impl AdkgRng,
        transport: Arc<T>,
    ) -> Result<AdkgOutput<CG>, AdkgError>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        self.execute(rng, transport).await
    }

    pub async fn stop(mut self) {
        let Some(rbc_task) = self.rbc_task.take() else {
            return;
        };
        let Some(acss_task) = self.acss_task.take() else {
            return;
        };

        // Cancel the remaining tasks
        self.cancel.cancel();
        info!("ADKG main thread of node `{}` joining ACSS task", self.id);
        if let Err(e) = acss_task.await {
            error!(
                "ADKG main thread of node `{}` failed to join ACSS task: {e:?}",
                self.id
            );
        }

        info!("ADKG main thread of node `{}` joining RBC task", self.id);
        if let Err(e) = rbc_task.await {
            error!(
                "ADKG main thread of node `{}` failed to join RBC task: {e:?}",
                self.id
            );
        }

        // With acss_task and rbc_task cancelled, we ought to have the only Arc<Self> reference here
        match Arc::try_unwrap(self.shared_state) {
            Ok(shared_state) => {
                if let Err(e) = shared_state.multi_aba.into_inner().stop().await {
                    error!(
                        "ADKG main thread of node `{}` stopped multi ABA with some errors: {e:?}",
                        self.id
                    );
                }
                if let Err(e) = shared_state.multi_rbc.into_inner().stop().await {
                    error!(
                        "ADKG main thread of node `{}` stopped multi RBC with some errors: {e:?}",
                        self.id
                    );
                }
                if let Err(e) = shared_state.multi_acss.into_inner().stop().await {
                    error!(
                        "ADKG main thread of node `{}` stopped multi ACSS with some errors: {e:?}",
                        self.id
                    );
                }
            }
            Err(shared_state_arc) => {
                error!(
                    "ADKG main thread of node `{}` failed to join obtain shared_state: there are `{}` > 1 outstanding references",
                    self.id,
                    Arc::strong_count(&shared_state_arc)
                );
            }
        }
    }

    async fn execute<T>(
        &mut self,
        rng: &mut impl AdkgRng,
        transport: Arc<T>,
    ) -> Result<AdkgOutput<CG>, AdkgError>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        let state = self.shared_state.clone();
        let mut adkg_transport = transport
            .get_transport_for("adkg".to_string())
            .ok_or(AdkgError::TransportInit)?;
        let adkg_sender = adkg_transport.sender().ok_or(AdkgError::TransportInit)?;
        let mut adkg_receiver = adkg_transport
            .receiver_stream()
            .ok_or(AdkgError::TransportInit)?;

        // Get an RNG instance for the ACSS secret
        let mut acss_rng = rng
            .get(AdkgRngType::AcssSecret)
            .map_err(|e| AdkgError::Rng(e.into(), "failed to get acss secret rng"))?;

        // Generate a random secret scalar to be used in the node's ACSS
        let s = CG::ScalarField::rand(&mut acss_rng);

        // Generate predicates for each of the RBCs
        let rbc_predicates: Vec<_> = PartyId::iter_all(self.n)
            .map(|i| NotifyPredicate {
                completed_acss: state.completed_acss_outputs.clone(),
                expected_sender: i,
            })
            .collect();

        // Start the multi RBC, ACSS and ABA
        state
            .multi_acss
            .lock()
            .await
            .start(&s, rng, transport.clone());
        state
            .multi_rbc
            .lock()
            .await
            .start(rbc_predicates, transport.clone());
        state.multi_aba.lock().await.start(rng, transport.clone());

        // Get the node's own RBC
        let leader_sender = state
            .multi_rbc
            .lock()
            .await
            .get_leader_sender()
            .expect("failed to get leader sender");

        // Create cancellation tokens for each subtask
        let acss_cancel = self.cancel.child_token();
        let rbc_cancel = self.cancel.child_token();
        let aba_cancel = self.cancel.child_token();

        // Handler for the key set proposal phase. Manages the termination of
        self.acss_task = Some(task::spawn(Self::acss_task(
            leader_sender,
            state.clone(),
            acss_cancel.clone(),
        )));

        self.rbc_task = Some(task::spawn(Self::rbc_task(
            state.clone(),
            rbc_cancel.clone(),
        )));

        // Upon termination of jth ABA
        let abas_task = task::spawn(Self::aba_outputs_task(state.clone(), aba_cancel.clone()));

        // Try to join ABAs task, and obtain the final list of parties.
        info!(
            "self main thread of node `{}` waiting on ABA task to complete",
            self.id
        );
        let abas_task_result = abas_task.await;
        let final_sessions = match abas_task_result {
            Ok(Ok(final_parties)) => {
                info!(
                    "self main thread of node `{}` obtained the final list of parties: {final_parties:?}",
                    self.id
                );
                final_parties
            }
            Ok(Err(e)) => {
                error!(
                    "self main thread of node `{}` failed to obtain the final list of parties: {e:?}",
                    self.id
                );
                panic!(
                    "self main thread of node `{}` failed to obtain the final list of parties: {e:?}",
                    self.id
                );
            }
            Err(e) => {
                panic!(
                    "self main thread of node `{}` failed to join ABA task: {e:?}",
                    self.id
                );
            }
        };

        // We got the final list of parties, enough ACSS instances have completed
        let shares: Vec<_> = state
            .completed_acss_outputs
            .filter_outputs(final_sessions.iter())
            .collect();

        // 42: z_i = \sum_{k \in T} s_{k, i}
        // z_i is the sum of the ACSS shares with id in final_parties
        let z_i: CG::ScalarField = shares
            .iter()
            .filter_map(|(k, v)| {
                if final_sessions.contains(k) {
                    Some(v.share)
                } else {
                    None
                }
            })
            .sum();

        // 43 Let \pi_i be the NIZK proof of log_g g^{z_i} = log_h h^{z_i}
        let pk_i = self.h * z_i;
        let pi_i: NIZKDleqProof<CG, H> = NIZKDleqProof::prove(
            &z_i,
            &self.g,
            &self.h,
            &(self.g * z_i),
            &pk_i,
            &self.nizk_dleq_dst,
            &mut OsRng,
        )
        .map_err(|e| AdkgError::Nizk(e, "failed to generate nizk proof"))?;

        // 44: Send \langle KEY, h_^{z_i}, \pi_i \rangle to all
        let msg = AdkgMessage::PublicKey(AdkgPublicKeyMessage {
            pi_j: pi_i,
            pk_j: pk_i,
        });

        if let Err(e) = broadcast_with_self(&msg, &RetryStrategy::None, &adkg_sender).await {
            error!(
                "Node `{}` failed to send ADKG Public Key message to other nodes: {e:?}",
                self.id
            );
        }

        // Obtain the combined public polynomial as p_j = \sum_{k \in rbc_parties} p_k(x)
        // which is the sum of the public polynomial output by each ACSS specified in the j-th RBC
        let public_poly: Vec<CG> = (0..=self.t)
            .map(|i| shares.iter().map(|(_, out)| out.public_poly[i]).sum())
            .collect();
        let (group_pk, node_pks) = self
            .derive_group_pk::<T::Transport>(&mut adkg_receiver, &public_poly)
            .await
            .map(|(group_pk, node_pks)| (Some(group_pk), Some(node_pks.into_iter().collect())))
            .unwrap_or_else(|e| {
                error!("Failed to compute group pk {e:?}");
                (None, None)
            });

        Ok(AdkgOutput {
            sk: z_i,
            used_sessions: final_sessions,
            group_pk,
            node_pks,
        })
    }

    async fn derive_group_pk<T>(
        &self,
        receiver: &mut T::ReceiveMessageStream,
        public_poly: &[CG],
    ) -> Result<(CG, impl IntoIterator<Item = CG>), AdkgError>
    where
        T: Transport<Identity = PartyId>,
    {
        let mut pubs = BTreeMap::new();
        loop {
            let ReceivedMessage {
                sender, content, ..
            } = match receiver.next().await {
                Some(Ok(m)) => m,
                Some(Err(e)) => {
                    warn!("Node `{}` failed to recv: {e:?}", self.id);
                    continue;
                }
                None => {
                    error!("Node `{}` failed to recv: no more items in stream", self.id);
                    return Err(AdkgError::DeriveGroupPublicKey);
                }
            };
            let msg: AdkgMessage<CG, H> = match bson::from_slice(&content) {
                Ok(m) => m,
                Err(e) => {
                    warn!(
                        "Node `{}` received ADKG Public Key message from {sender} with bad format: {e:?}",
                        self.id
                    );
                    continue;
                }
            };

            let AdkgMessage::PublicKey(m) = msg;
            let g_z_j = eval_poly(&u64::from(sender).into(), public_poly);
            if m.pi_j
                .verify(&self.g, &self.h, &g_z_j, &m.pk_j, &self.nizk_dleq_dst)
                .is_ok()
            {
                pubs.insert(u64::from(sender), m.pk_j);
            }

            // We now have more than t public keys, we can interpolate the group pk and the missing pks
            if pubs.len() > self.t {
                let pubs_vec: Vec<_> = pubs.iter().map(|(id, pk)| (*id, *pk)).collect();
                let group_pk = lagrange_points_interpolate_at(&pubs_vec[..], 0);

                let iter = (1..=u64_from_usize(self.n)).map(move |id| {
                    pubs.get(&id)
                        .copied()
                        .unwrap_or_else(|| lagrange_points_interpolate_at(&pubs_vec[..], id))
                });
                return Ok((group_pk, iter));
            }
        }
    }

    async fn acss_task(
        rbc_leader_sender: oneshot::Sender<Vec<u8>>,
        state: Arc<SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>>,
        cancel: CancellationToken,
    ) {
        let id = state.id;
        let n = state.n;
        let inner_fn = async move {
            let mut rbc_leader_sender = Some(rbc_leader_sender);
            let mut remaining_outputs = state.multi_acss.lock().await.iter_remaining_outputs();
            // Loop until there are no more outputs available
            loop {
                // Wait for the next output
                let (id, sk) = match remaining_outputs.next().await {
                    // No more outputs, break
                    None => {
                        break;
                    }
                    // Got an output with an error, log it and get next output
                    Some((id, Err(e))) => {
                        error!(
                            "Node {} failed to get output from ACSS with sid `{id}`: {e:?}",
                            state.id,
                        );
                        continue;
                    }
                    // Got output
                    Some((id, Ok(out))) => {
                        info!("Node {} completed ACSS with sid {}", state.id, id);
                        (id, out)
                    }
                };

                // Insert the new share in the map and notify rbc predicate waiters
                state.completed_acss_outputs.insert(id, sk.into());

                // Once we have completed t + 1 ACSSs, broadcast the list of parties through our RBC
                let completed_acss_ids = state.completed_acss_outputs.keys();
                // TODO: We probably need to set some stages specifying how many shares we want to wait for, and
                //  after how long we go to the next stage. E.g., n: wait for 1 minute, then, n/2: wait for 2 minutes,
                //  n/3 otherwise.
                //  If we stick to only t + 1, the ADKG may terminate quickly using only t + 1 ACSS shares, if we set
                //  it to only n, the ADKG may never terminate.
                if completed_acss_ids.len() == state.t + 1 {
                    if rbc_leader_sender
                        .take()
                        .expect("cannot enter condition twice")
                        .send(
                            bson::to_vec(&CompletedAcssSessions {
                                v: completed_acss_ids.into_iter().collect(),
                            })
                            .unwrap(),
                        )
                        .is_ok()
                    {
                        info!("Node {} set its own RBC input", state.id);
                    } else {
                        error!(
                            "Node {} failed to send input to RBC, receiver dropped",
                            state.id
                        );
                    }
                }
            }

            state.completed_acss_outputs.keys()
        };

        tokio::select! {
            _ = cancel.cancelled_owned() => {
                info!(
                    "ADKG ACSS task of node `{id}` exiting due to CancellationToken"
                );
            }

            completed_acss_ids = inner_fn => {
                if completed_acss_ids.len() == n {
                    info!(
                        "Node {id} completed all ACSS, exiting ADKG ACSS handler task"
                    );
                } else {
                    warn!(
                        "Node {id} exiting ACSS handler task with missing shares. The following ACSS completed: {:?}", completed_acss_ids
                    );
                }
            }
        }
    }

    async fn rbc_task(
        state: Arc<SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>>,
        cancel: CancellationToken,
    ) {
        let id = state.id;
        let cancel_child = cancel.child_token();
        let inner_fn = async move {
            let mut multi_rbc = state.multi_rbc.lock().await;

            loop {
                // Loop until there are no more outputs available
                let Some(res) = multi_rbc.join_next().await else {
                    break;
                };

                let (j, v) = match res {
                    Ok((j, Ok(v))) => (j, v),
                    Ok((j, Err(e))) => {
                        error!(
                            "Node {} failed to get output from RBC with sid `{j}`: {:?}",
                            state.id, e
                        );
                        continue;
                    }
                    Err(e) => {
                        error!("Node {} failed to join an RBC task: {:?}", state.id, e);
                        continue;
                    }
                };

                info!("Node {} completed RBC with sid `{j}`", state.id);

                // Try to deserialize the list of parties
                let rbc_parties = match bson::from_slice(&v) {
                    Ok(CompletedAcssSessions { v }) => v,
                    Err(e) => {
                        error!(
                            "Node {} received a badly formed output from RBC with sid {j}: {e:}",
                            state.id
                        );
                        continue;
                    }
                };

                // Store the RBC output in the state machine
                state.rbc_outputs.insert(j, rbc_parties.clone());

                // Create new task that sets the input of the j-th ABA once rbc_parties is a subset of the completed_acss_outputs
                task::spawn(Self::aba_starter(
                    j,
                    state.clone(),
                    rbc_parties,
                    cancel_child.clone(),
                ));
            }

            let rbc_outputs_ids: Vec<_> = state.rbc_outputs.keys();

            if rbc_outputs_ids.len() == state.n {
                info!(
                    "Node {} completed all RBCs, exiting RBC handler task",
                    state.id
                );
            } else {
                warn!(
                    "Node {} exiting RBC handler task with missing outputs. The following RBC completed: {:?}",
                    state.id, rbc_outputs_ids
                );
            }
        };

        tokio::select! {
            _ = cancel.cancelled_owned() => {
                info!(
                    "ADKG RBC task of node `{id}` exiting due to CancellationToken"
                );
            }

            _ = inner_fn => {
                info!(
                    "ADKG RBC task of node `{id}` exiting due to all outputs obtained",
                );
            }
        }
    }

    async fn aba_outputs_task(
        state: Arc<SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>>,
        cancel: CancellationToken,
    ) -> Result<Vec<SessionId>, AdkgError> {
        let id = state.id;
        let inner_fn = async move {
            let mut final_parties: HashSet<SessionId> = HashSet::new();
            let mut remaining_estimates = state.multi_aba.lock().await.iter_remaining_estimates();
            loop {
                let Some((j, res)) = remaining_estimates.next().await else {
                    break;
                };
                let Ok(estimate) = res else {
                    error!(
                        "Node {} failed to get output from ABA with sid {j}: sender dropped",
                        id,
                    );
                    continue;
                };

                info!(
                    "Node {} completed ABA with sid {} and got estimate {estimate:?}",
                    id, j
                );

                state
                    .multi_aba
                    .lock()
                    .await
                    .cancel(j)
                    .await
                    .unwrap()
                    .unwrap();

                if let Estimate::One = estimate {
                    let rbc_parties = match state.rbc_outputs.get(&j) {
                        Some(rbc_parties) => rbc_parties,
                        None => {
                            info!(
                                "Node `{}` obtained an estimate from ABA with sid `{j}` without having obtained an RBC output, waiting.",
                                state.id
                            );
                            Self::wait_for_rbc_output(&state.rbc_outputs, &j).await
                        }
                    };

                    final_parties.extend(rbc_parties.iter());

                    // Input 0 to remaining ABAs
                    let iter_remaining_senders =
                        state.multi_aba.lock().await.iter_remaining_senders();
                    for (k, sender) in iter_remaining_senders {
                        // Create new channel and send coin_keys through it
                        let (isender, ireceiver) = oneshot::channel();
                        let input = AbaInput {
                            v: Estimate::Zero,
                            coin_keys_receiver: ireceiver,
                        };

                        info!("Node `{}` inputting 0 to ABA with sid `{}`", id, k);
                        let Ok(_) = sender.send(input) else {
                            error!(
                                "Node `{}` failed to input 0 to ABA with sid `{}`: input has already been sent",
                                id, k
                            );
                            continue;
                        };

                        // Spawn task to send coin_keys input upon receiving it, or cancel if ireceiver is dropped before
                        task::spawn(Self::async_coinkeys_sender_task(state.clone(), k, isender));
                    }
                }
            }
            final_parties.into_iter().collect()
        };

        tokio::select! {
            _ = cancel.cancelled_owned() => {
                info!(
                    "ADKG ABA task of node `{id}` exiting due to CancellationToken"
                );
                Err(AdkgError::AbaCancelledEarly)
            }

            final_parties = inner_fn => {
                info!(
                    "ADKG ABA task of node `{id}` exiting due to all ABAs completed",
                );

                Ok(final_parties)
            }
        }
    }

    async fn aba_starter(
        j: SessionId,
        state: Arc<SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>>,
        rbc_parties: HashSet<SessionId>,
        cancel: CancellationToken,
    ) {
        let id = state.id;
        let inner_fn = async move {
            loop {
                // Check whether the completed ACSSs is a superset of rbc_parties
                // i.e., rbc_parties is a subset of completed_acss_outputs
                if state.completed_acss_outputs.is_superset(&rbc_parties) {
                    // Get the acss outputs to be used as the coin keys
                    let outputs: Vec<_> = state
                        .completed_acss_outputs
                        .filter_outputs(rbc_parties.iter())
                        .collect();
                    let coin_keys = LazyCoinKeys::new(state.n, state.t, outputs);

                    // Create new channel and send coin_keys through it
                    let (sender, receiver) = oneshot::channel();
                    sender.send(coin_keys).expect("receiver not dropped here");

                    // Input 1 to the j-th ABA if it has not received an input
                    let input = AbaInput {
                        v: Estimate::One,
                        coin_keys_receiver: receiver,
                    };
                    match state
                        .multi_aba
                        .lock()
                        .await
                        .get_sender(&j)
                        .map(|s| s.send(input))
                    {
                        None => {
                            // ABA has already received an input, ignore it
                        }
                        Some(Ok(_)) => {}
                        Some(Err(_)) => {
                            error!(
                                "Node {} failed to set input to ABA {j} to estimate {:?}: receiver dropped",
                                state.id,
                                Estimate::One,
                            );
                            break;
                        }
                    };

                    break;
                }

                // If not, wait for an update
                state.completed_acss_outputs.wait().await;
            }
        };

        tokio::select! {
            _ = cancel.cancelled_owned() => {
                info!(
                    "ADKG CoinKeys provider of node `{id}` for ABA {j} exiting due to CancellationToken"
                );
            }

            _ = inner_fn => {
                info!(
                    "ADKG CoinKeys provider of node `{id}` for ABA {j} exiting due to CancellationToken"
                );
            }
        }
    }

    async fn async_coinkeys_sender_task(
        state: Arc<SharedState<CG, RBCConfig, ACSSConfig, ABAConfig>>,
        sid: SessionId,
        mut sender: oneshot::Sender<LazyCoinKeys<CG>>,
    ) {
        let inner_fn = async {
            let rbc_output = Self::wait_for_rbc_output(&state.rbc_outputs, &sid).await;

            loop {
                if state.completed_acss_outputs.is_superset(&rbc_output) {
                    // Get the acss outputs to be used as the coin keys
                    let outputs: Vec<_> = state
                        .completed_acss_outputs
                        .filter_outputs(rbc_output.iter())
                        .collect();
                    let coin_keys = LazyCoinKeys::new(state.n, state.t, outputs);
                    return coin_keys;
                }

                // If not, wait for an update
                state.completed_acss_outputs.wait().await;
            }
        };

        // Execute inner_fn until the coin_keys receiver is dropped
        tokio::select! {
            coin_keys = inner_fn => {
                // send may fail if inner_fn completes at the same time as the receiver is dropped.
                // ignore error silently for that case.
                let _ = sender.send(coin_keys);
            }
            _ = sender.closed() => {}
        }
    }

    async fn wait_for_rbc_output(
        rbc_outputs: &Arc<NotifyMap<HashSet<SessionId>>>,
        sid: &SessionId,
    ) -> HashSet<SessionId> {
        let wait_for_rbc = async {
            loop {
                if let Some(rbc_output) = rbc_outputs.get(sid) {
                    return rbc_output;
                }

                rbc_outputs.wait().await;
            }
        };
        wait_for_rbc.await
    }
}

#[cfg(test)]
mod tests {
    use crate::aba::crain20::AbaCrain20Config;
    use crate::adkg::{APPNAME, Adkg, AdkgOutput};
    use crate::helpers::{PartyId, lagrange_interpolate_at};
    use crate::network::RetryStrategy;
    use crate::rand::{AdkgRng, AdkgRngType, get_rng};
    use crate::rbc::r4::Rbc4RoundsConfig;
    use crate::vss::acss::hbacss0::HbAcss0Config;
    use ark_ec::{CurveGroup, Group};
    use ark_std::UniformRand;
    use dcipher_network::topic::dispatcher::TopicDispatcher;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use digest::core_api::BlockSizeUser;
    use itertools::izip;
    use std::collections::{HashMap, VecDeque};
    use std::sync::Arc;
    use tokio::task::JoinSet;
    use utils::dst::{NamedCurveGroup, NamedDynDigest, Rfc9380DstBuilder};
    use utils::hash_to_curve::HashToCurve;
    use utils::serialize::fq::{FqDeserialize, FqSerialize};
    use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

    fn get_expected_acss_secrets<CG: CurveGroup>(n: usize, seed: &[u8]) -> Vec<CG::ScalarField> {
        let mut rngs: Vec<_> = PartyId::iter_all(n).map(|i| get_rng(i, seed)).collect();
        rngs.iter_mut()
            .map(|rng| {
                let mut rng = rng.get(AdkgRngType::AcssSecret).unwrap();
                CG::ScalarField::rand(&mut rng)
            })
            .collect()
    }

    fn get_generator_g<CG, H>() -> CG
    where
        CG: NamedCurveGroup + HashToCurve,
        H: Default + NamedDynDigest + BlockSizeUser + Clone,
    {
        let dst: Vec<_> = Rfc9380DstBuilder::empty()
            .with_application_name(APPNAME.to_vec())
            .with_curve::<CG>()
            .with_hash::<H>()
            .with_suffix(b"TEST".to_vec())
            .build()
            .into();

        CG::hash_to_curve_custom::<H>(b"ADKG_GENERATOR_G", &dst)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 32)]
    async fn adkg_test_bn254() {
        // Static configuration and long term keys
        let t = 2;
        let n = 3 * t + 1;

        const SEED: &[u8] = b"ADKG_BN254_TEST_SEED";

        // We use h == Bn254 G1 as the generator for the group public key
        // and an independent generator g for the ADKG operations.
        let g = get_generator_g::<_, sha3::Sha3_256>();
        let h = ark_bn254::G1Projective::generator();

        run_adkg_test::<_, sha3::Sha3_256>(t, n, g, h, SEED).await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn adkg_test_bls12_381() {
        // Static configuration and long term keys
        let t = 2;
        let n = 3 * t + 1;

        const SEED: &[u8] = b"ADKG_BLS12381_TEST_SEED";

        // We use h == Bls12_381 G1 as the generator for the group public key
        // and an independent generator g for the ADKG operations.
        let g = get_generator_g::<_, sha3::Sha3_256>();
        let h = ark_bls12_381::G1Projective::generator();

        run_adkg_test::<_, sha3::Sha3_256>(t, n, g, h, SEED).await
    }

    async fn run_adkg_test<CG, H>(t: usize, n: usize, g: CG, h: CG, seed: &[u8])
    where
        CG: NamedCurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve,
        CG::ScalarField: FqSerialize + FqDeserialize,
        H: Default + NamedDynDigest + BlockSizeUser + Clone + 'static,
    {
        _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .try_init();

        let sks: VecDeque<CG::ScalarField> = (1..=n)
            .map(|_| CG::ScalarField::rand(&mut rand::thread_rng()))
            .collect();
        let pks: Vec<CG> = sks.iter().map(|sk| g * sk).collect();

        // Setup the networking
        let (dispatchers, tbts): (Vec<_>, Vec<_>) =
            MemoryNetwork::get_transports(PartyId::iter_all(n))
                .into_iter()
                .map(|t| {
                    let mut dispatcher = TopicDispatcher::new();
                    let tbt = dispatcher.start(t);
                    (dispatcher, tbt)
                })
                .collect();
        let instances: Vec<_> = PartyId::iter_all(n)
            .map(|i| {
                let rbc_config = Rbc4RoundsConfig::new(i, n, t, &RetryStrategy::None);
                let acss_config = HbAcss0Config::<_, sha3::Sha3_256, _>::new(
                    i,
                    sks[i.as_index()],
                    pks.clone(),
                    rbc_config.clone(),
                    n,
                    t,
                    g,
                    RetryStrategy::None,
                );
                let aba_config =
                    AbaCrain20Config::<_, _, sha3::Sha3_256>::new(i, n, t, g, RetryStrategy::None);

                Adkg::<_, H, _, _, _>::new(i, n, t, g, h, rbc_config, acss_config, aba_config)
            })
            .collect();

        // Execute each node
        let mut tasks = JoinSet::new();
        for (id, transport, mut adkg) in izip!(PartyId::iter_all(n), tbts.into_iter(), instances) {
            let mut rng = get_rng(id, seed);
            tasks.spawn(async move {
                let out = adkg.start(&mut rng, Arc::new(transport)).await;
                adkg.stop().await;
                (id, out, rng)
            });
        }

        let mut shares = HashMap::new();
        let mut rngs = HashMap::new();
        while let Some(res) = tasks.join_next().await {
            let (id, out, rng) = res.unwrap();
            shares.insert(id, out.unwrap());
            rngs.insert(id, rng);
        }

        for id in PartyId::iter_all(n) {
            let out = shares.get(&id).unwrap();
            println!("Node `{id}` got share {}", out.sk);
            println!(
                "Node `{id}` got group public key {}\n",
                out.group_pk.unwrap()
            );
        }

        // Verify that each node has the same public key
        let mut iter_shares = shares.iter();
        let AdkgOutput {
            group_pk: Some(group_pk),
            used_sessions,
            ..
        } = iter_shares.next().unwrap().1
        else {
            panic!("failed to obtain group pk")
        };
        assert!(iter_shares.all(|(_, out)| out.group_pk.unwrap() == *group_pk));
        assert!(iter_shares.all(|(_, out)| out.used_sessions == *used_sessions));

        let points: Vec<(u64, CG::ScalarField)> =
            shares.iter().map(|(id, out)| (id.into(), out.sk)).collect();

        // Verify that the group public key matches the group secret
        let s: CG::ScalarField = lagrange_interpolate_at::<CG>(&points[0..=t], 0);
        assert_eq!((h * s).into_affine(), group_pk.into_affine());

        // The group secret is the sum of the used ACSS secrets
        let acss_secrets = get_expected_acss_secrets::<CG>(n, seed);
        let expected_s: CG::ScalarField = used_sessions.iter().map(|i| acss_secrets[i]).sum();
        assert_eq!(s, expected_s);

        for d in dispatchers {
            d.stop().await;
        }
    }
}
