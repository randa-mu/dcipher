mod randex;
pub(crate) mod types;

/// Re-export types required by ADKG trait bounds
pub use types::AbaCrainInput;
pub use types::ShareWithPoly;

use crate::aba::crain20::AbaInput;
use crate::aba::multi_aba::MultiAba;
use crate::aba::{AbaConfig, Estimate};
use crate::adkg::randex::build_randex_matrices;
use crate::adkg::types::{AdkgKeyMessage, AdkgRandExMessage};
use crate::helpers::{PartyId, SessionId, lagrange_points_interpolate_at};
use crate::network::{RetryStrategy, broadcast_with_self, send_serialize_helper};
use crate::pok::{PokError, PokProof};
use crate::rand::{AdkgRng, AdkgRngType};
use crate::rbc::ReliableBroadcastConfig;
use crate::rbc::multi_rbc::MultiRbc;
use crate::vss::acss::AcssConfig;
use crate::vss::acss::hbacss0::PedersenSecret;
use crate::vss::acss::multi_acss::MultiAcss;
use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::Zero;
use ark_std::UniformRand;
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::{ReceivedMessage, Recipient, Transport};
use digest::core_api::BlockSizeUser;
use digest::{DynDigest, FixedOutputReset};
use futures::StreamExt;
use itertools::izip;
use nalgebra::RowDVector;
use std::collections::{BTreeMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::oneshot;
use tokio::task;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use types::{
    AdkgMessage, CompletedAcssSessions, LazyCoinKeys, NotifyMap, NotifyPredicate, SharedState,
};
use utils::dst::{NamedCurveGroup, NamedDynDigest, Rfc9380DstBuilder};
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

const APPNAME: &[u8] = b"ADKG-v1";
const KEY_POK_SUFFIX: &[u8] = b"KEY_POK";

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
    t_reconstruction: usize,

    g: CG,
    h: CG,
    pok_dst: Vec<u8>,
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

    #[error("pok error: `{1}`")]
    Pok(#[source] PokError, &'static str),

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

    #[error("failed to complete randomness extraction phase")]
    RandEx,
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
        t_reconstruction: usize,
        g: CG,
        h: CG,
        rbc_config: Arc<RBCConfig>,
        acss_config: Arc<ACSSConfig>,
        aba_config: Arc<ABAConfig>,
    ) -> Self {
        // Check bounds
        if n < 3 * t + 1 {
            panic!(
                "t must be <= (n - 1) / 3 = {}, got n = {n}, t = {t}",
                (n - 1) / 3
            )
        }
        if t_reconstruction < t || t_reconstruction >= n - t {
            panic!(
                "reconstruction threshold ({}) must be in [t, n - t - 1] == [{t}, {}]",
                t_reconstruction,
                n - t - 1
            );
        }

        let multi_rbc = MultiRbc::new(id, n, rbc_config);
        let multi_acss = MultiAcss::new(id, n, acss_config);
        let multi_aba = MultiAba::new(id, n, aba_config);

        // Generate a DST in the following format: ADKG-v1_%CURVE_NAME%_XMD:%HASH_NAME%_RO_KEY_POK_
        // e.g.: ADKG-v1_BN254G1_XMD:SHA3-256_RO_KEY_POK_
        let pok_dst = Rfc9380DstBuilder::empty()
            .with_application_name(APPNAME.to_vec())
            .with_curve::<CG>()
            .with_hash::<H>()
            .with_suffix(KEY_POK_SUFFIX.to_vec())
            .build()
            .into();

        Self {
            id,
            n,
            t,
            t_reconstruction,
            g,
            h,
            cancel: CancellationToken::new(),
            rbc_task: None,
            acss_task: None,
            shared_state: SharedState::new(id, n, t, multi_rbc, multi_acss, multi_aba).into(),
            pok_dst,
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
    H: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone + 'static,
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
    ACSSConfig: AcssConfig<'static, CG, PartyId, Input = Vec<PedersenSecret<CG::ScalarField>>>,
    ACSSConfig::Output: Into<ShareWithPoly<CG>>,
    ABAConfig: AbaConfig<'static, PartyId, Input = AbaCrainInput<CG>>,
{
    /// Start the ADKG immediately
    pub async fn start<T>(
        &mut self,
        rng: &mut impl AdkgRng,
        transport: Arc<T>,
    ) -> Result<AdkgOutput<CG>, AdkgError>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        self.execute_internal(std::future::ready(()), rng, transport)
            .await
    }

    /// Start the ADKG with a delay
    pub async fn start_delayed<T>(
        &mut self,
        start: impl Future,
        rng: &mut impl AdkgRng,
        transport: Arc<T>,
    ) -> Result<AdkgOutput<CG>, AdkgError>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        self.execute_internal(start, rng, transport).await
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

    async fn execute_internal<T>(
        &mut self,
        start_signal: impl Future,
        rng: &mut impl AdkgRng,
        transport: Arc<T>,
    ) -> Result<AdkgOutput<CG>, AdkgError>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        // Either one or two secrets are required depending on the reconstruction threshold. Greater than
        // t, two secrets, one otherwise.
        let shares_per_acss: usize = 1 + usize::from(self.t_reconstruction > self.t);

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

        // Generate random secret scalars to be used in the node's ACSS
        let s: Vec<_> = (0..shares_per_acss)
            .map(|_| {
                let a = CG::ScalarField::rand(&mut acss_rng);
                let a_hat = CG::ScalarField::rand(&mut acss_rng);
                PedersenSecret { s: a, r: a_hat }
            })
            .collect();

        // Generate predicates for each of the RBCs
        let rbc_predicates: Vec<_> = PartyId::iter_all(self.n)
            .map(|i| NotifyPredicate {
                min_size: self.n - self.t,
                completed_acss: state.completed_acss_outputs.clone(),
                expected_sender: i,
            })
            .collect();

        // Start the multi RBC, ACSS and ABA
        state.multi_acss.lock().await.start(rng, transport.clone());
        state
            .multi_rbc
            .lock()
            .await
            .start(rbc_predicates, transport.clone());
        state.multi_aba.lock().await.start(rng, transport.clone());

        // Get the ACSS sender
        let acss_leader_sender = state
            .multi_acss
            .lock()
            .await
            .get_leader_sender()
            .expect("failed to get acss leader sender");

        // Get the node's own RBC
        let rbc_leader_sender = state
            .multi_rbc
            .lock()
            .await
            .get_leader_sender()
            .expect("failed to get rbc leader sender");

        // Create cancellation tokens for each subtask
        let acss_cancel = self.cancel.child_token();
        let rbc_cancel = self.cancel.child_token();
        let aba_cancel = self.cancel.child_token();

        // Handler for the key set proposal phase. Manages the termination of
        self.acss_task = Some(task::spawn(Self::acss_task(
            rbc_leader_sender,
            state.clone(),
            acss_cancel.clone(),
        )));

        self.rbc_task = Some(task::spawn(Self::rbc_task(
            state.clone(),
            rbc_cancel.clone(),
        )));

        // Upon termination of jth ABA
        let abas_task = task::spawn(Self::aba_outputs_task(state.clone(), aba_cancel.clone()));

        // Everything has been set-up, wait for the start signal
        start_signal.await;
        if acss_leader_sender.send(s).is_err() {
            error!(
                "ADKG main thread of node `{}` failed to set ACSS input",
                self.id
            );
        }

        // Try to join ABAs task, and obtain the final list of parties.
        info!(
            "ADKG main thread of node `{}` waiting on ABA task to complete",
            self.id
        );
        let abas_task_result = abas_task.await;
        let final_sessions = match abas_task_result {
            Ok(Ok(final_parties)) => {
                info!(
                    "ADKG main thread of node `{}` obtained the final list of parties: {final_parties:?}",
                    self.id
                );
                final_parties
            }
            Ok(Err(e)) => {
                error!(
                    "ADKG main thread of node `{}` failed to obtain the final list of parties: {e:?}",
                    self.id
                );
                panic!(
                    "ADKG main thread of node `{}` failed to obtain the final list of parties: {e:?}",
                    self.id
                );
            }
            Err(e) => {
                panic!(
                    "ADKG main thread of node `{}` failed to join ABA task: {e:?}",
                    self.id
                );
            }
        };

        // We got the final list of parties, enough ACSS instances have completed
        // 24: Let T be the output of the MVBA protocol
        let acss_outputs: Vec<_> = state
            .completed_acss_outputs
            .filter_outputs(final_sessions.iter())
            .collect();

        // Randomness extraction phase: recover pedersen commits, shares z_i, z_hat_i, and, optionally,
        // messages for the next round.
        let (z_i, z_hat_i, ped_commits, key_messages) = self
            .randex_phase::<T::Transport>(
                shares_per_acss,
                acss_outputs,
                &adkg_sender,
                &mut adkg_receiver,
            )
            .await?;

        // Group & partial public key derivation phase
        let key_der_out = self
            .key_derivation_phase::<T::Transport>(
                &z_i,
                &z_hat_i,
                &ped_commits,
                key_messages,
                &adkg_sender,
                &mut adkg_receiver,
                rng,
            )
            .await;
        let (group_pk, node_pks) = match key_der_out {
            Ok((group_pk, nodes_pks)) => (Some(group_pk), Some(nodes_pks)),
            Err(e) => {
                error!(
                    error = ?e,
                    "ADKG failed to derive public keys"
                );

                // If it fails, return no group_pk / nodes_pks
                (None, None)
            }
        };

        Ok(AdkgOutput {
            sk: z_i,
            used_sessions: final_sessions,
            group_pk,
            node_pks,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn key_derivation_phase<T>(
        &mut self,
        z_i: &<CG as PrimeGroup>::ScalarField,
        z_hat_i: &<CG as PrimeGroup>::ScalarField,
        ped_commits: &[CG],
        key_messages: impl IntoIterator<Item = (PartyId, AdkgKeyMessage<CG, H>)>,
        adkg_sender: &T::Sender,
        adkg_receiver: &mut T::ReceiveMessageStream,
        rng: &mut impl AdkgRng,
    ) -> Result<(CG, Vec<CG>), AdkgError>
    where
        T: Transport<Identity = PartyId>,
    {
        // Key derivation phase
        // 51: Generate proof of knowledge of z_i, z_hat_i
        let mut pok_rng = rng
            .get(AdkgRngType::KeyPok)
            .map_err(|e| AdkgError::Rng(e.into(), "failed to obtain rng for PoK"))?;
        let g_z_i = self.g * z_i;
        let pok_z_i = PokProof::<CG, H>::prove(z_i, &self.g, &g_z_i, &self.pok_dst, &mut pok_rng)
            .map_err(|e| AdkgError::Pok(e, "failed to create PoK on z_i"))?;
        let h_z_hat_i = self.h * z_hat_i;
        let pok_z_hat_i =
            PokProof::<CG, H>::prove(z_hat_i, &self.h, &h_z_hat_i, &self.pok_dst, &mut pok_rng)
                .map_err(|e| AdkgError::Pok(e, "failed to create PoK on z_hat_i"))?;

        // 52: Broadcast proofs & public keys
        let msg: AdkgMessage<CG, H> = AdkgMessage::Key(AdkgKeyMessage {
            z_j_proof: pok_z_i,
            z_hat_j_proof: pok_z_hat_i,
            g_z_j: g_z_i,
            h_z_hat_j: h_z_hat_i,
        });
        if let Err(e) = broadcast_with_self(&msg, &RetryStrategy::None, adkg_sender).await {
            error!(
                "Node `{}` failed to send ADKG KEY message to other nodes: {e:?}",
                self.id
            );
        }

        self.key_derivation::<T>(ped_commits, key_messages, adkg_receiver)
            .await
    }

    async fn key_derivation<T>(
        &mut self,
        ped_commits: &[CG],
        key_messages: impl IntoIterator<Item = (PartyId, AdkgKeyMessage<CG, H>)>,
        adkg_receiver: &mut T::ReceiveMessageStream,
    ) -> Result<(CG, Vec<CG>), AdkgError>
    where
        T: Transport<Identity = PartyId>,
    {
        let mut key_messages = key_messages.into_iter();
        let mut valid_keys = BTreeMap::new();
        loop {
            // Attempt to drain key_messages first
            let (sender, msg) = match key_messages.next() {
                Some(sender_msg) => sender_msg,
                None => {
                    // Otherwise, recv new message
                    let (sender, AdkgMessage::Key(msg)) =
                        self.recv_next_adkg_msg::<T>(adkg_receiver).await?
                    else {
                        // recv errors are unrecoverable
                        continue; // ignore other types of messages
                    };

                    (sender, msg)
                }
            };

            // Ignore message if invalid
            if !self.is_key_msg_valid(ped_commits, &sender, &msg) {
                warn!(?sender, "Ignoring invalid ADKG KEY message");
                continue;
            }

            valid_keys.insert(sender, msg.g_z_j);

            #[allow(clippy::int_plus_one)]
            if valid_keys.len() >= self.t_reconstruction + 1 {
                // More keys than the reconstruction threshold, we can interpolate the rest
                let g_z_i_points: Vec<_> = valid_keys
                    .iter()
                    .map(|(j, g_z_j)| (j.into(), *g_z_j))
                    .collect();
                let g_z_0 = lagrange_points_interpolate_at(&g_z_i_points, 0);

                let missing_keys: Vec<_> = PartyId::iter_all(self.n)
                    .filter(|j| !valid_keys.contains_key(j))
                    .map(|j| (j, lagrange_points_interpolate_at(&g_z_i_points, j.into())))
                    .collect();

                valid_keys.extend(missing_keys);
                return Ok((g_z_0, valid_keys.into_values().collect()));
            }
        }
    }

    async fn randex_phase<T>(
        &mut self,
        shares_per_acss: usize,
        acss_outputs: Vec<(SessionId, ShareWithPoly<CG>)>,
        adkg_sender: &T::Sender,
        adkg_receiver: &mut T::ReceiveMessageStream,
    ) -> Result<
        (
            <CG as PrimeGroup>::ScalarField,
            <CG as PrimeGroup>::ScalarField,
            Vec<CG>,
            BTreeMap<PartyId, AdkgKeyMessage<CG, H>>,
        ),
        AdkgError,
    >
    where
        T: Transport<Identity = PartyId>,
    {
        // 25-28: for each j \in [n] \setminus T: set identity
        // Set the secrets, randomness, ped_commits to zero when the ACSS was selected, use our share
        // otherwise.
        let mut secrets = vec![vec![CG::ScalarField::zero(); self.n]; shares_per_acss];
        let mut randomness = vec![vec![CG::ScalarField::zero(); self.n]; shares_per_acss];
        let mut all_ped_commits = vec![vec![CG::Affine::zero(); self.n]; shares_per_acss];
        for (sid, acss_output) in acss_outputs {
            for (share_idx, (share, poly)) in
                izip!(acss_output.shares, acss_output.public_polys).enumerate()
            {
                secrets[share_idx][sid] = share.si; // a_i, b_i
                randomness[share_idx][sid] = share.ri; // \hat{a}_i
                all_ped_commits[share_idx][sid] = poly.to_vec()[0].into_affine(); // u_i
            }
        }

        // Randomness extraction phase
        // 32-37: Compute [[z(j)]]_i, [[z_hat(j)]]_i for j \in [n]
        // Compute a secret-shared evaluation of the polynomial z(x), z_hat(x) for each of the parties
        let (m, m_tilde) =
            build_randex_matrices::<CG::ScalarField>(self.t, self.t_reconstruction, self.n);
        let m = [m, m_tilde];
        let m_vec: Vec<_> = m
            .clone()
            .into_iter()
            .map(|m_i| {
                m_i.row_iter()
                    .map(|row| row.into_iter().cloned().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut z_shares = vec![CG::ScalarField::zero(); self.n];
        let mut z_hat_shares = vec![CG::ScalarField::zero(); self.n];
        let mut ped_commits = vec![CG::zero(); self.n];
        for i in 0..self.n {
            for share_idx in 0..shares_per_acss {
                z_shares[i] += m[share_idx]
                    .row(i)
                    .dot(&RowDVector::from(secrets[share_idx].clone()));
                z_hat_shares[i] += m[share_idx]
                    .row(i)
                    .dot(&RowDVector::from(randomness[share_idx].clone()));
                ped_commits[i] += CG::msm(&all_ped_commits[share_idx], &m_vec[share_idx][i])
                    .expect("msm failed: bases and scalars have different lengths");
            }
        }

        // 38: Send \langle RANDEX, [[z(j)]]_i, [[\hat{z}(j)]]_i \rangle
        for (j_id, z_j, z_hat_j) in izip!(PartyId::iter_all(self.n), z_shares, z_hat_shares) {
            let msg: AdkgMessage<CG, H> = AdkgMessage::RandEx(AdkgRandExMessage { z_j, z_hat_j });
            if let Err(e) = send_serialize_helper(
                &msg,
                Recipient::Single(j_id),
                &RetryStrategy::None,
                adkg_sender,
            )
            .await
            {
                error!(
                    "Node `{}` failed to send ADKG RANDEX message to other nodes: {e:?}",
                    self.id
                );
            }
        }

        // 39-42: Wait for messages and execute OEC to recover our shares z_i, z_hat_i
        let mut key_messages = BTreeMap::new();
        let (z_i, z_hat_i) = self
            .randex_oec::<T>(&ped_commits, &mut key_messages, adkg_receiver)
            .await?;
        Ok((z_i, z_hat_i, ped_commits, key_messages))
    }

    async fn randex_oec<T>(
        &mut self,
        ped_commits: &[CG],
        key_messages: &mut BTreeMap<PartyId, AdkgKeyMessage<CG, H>>,
        adkg_receiver: &mut T::ReceiveMessageStream,
    ) -> Result<(CG::ScalarField, CG::ScalarField), AdkgError>
    where
        T: Transport<Identity = PartyId>,
    {
        let mut randex_messages = BTreeMap::new();
        loop {
            let (sender, adkg_msg) = self.recv_next_adkg_msg::<T>(adkg_receiver).await?; // recv errors are unrecoverable
            match adkg_msg {
                AdkgMessage::RandEx(m) => {
                    // Store it and continue evaluating
                    randex_messages.insert(sender, m);
                }
                AdkgMessage::Key(m) => {
                    // Got a public key message, simply store it for later
                    key_messages.insert(sender, m);
                    continue;
                }
            };

            #[allow(clippy::int_plus_one)]
            if randex_messages.len() >= self.t + 1 {
                // Once we have at least t + 1 randex messages, we can attempt an error correction with
                // no errors.
                // With 2t + 2 messages, we can try an error correction with 1 error up to t errors
                if let Some((z_i, z_hat_i)) = randex::oec_round(
                    randex_messages.to_owned(),
                    self.t,
                    &ped_commits[self.id],
                    &self.g,
                    &self.h,
                ) {
                    return Ok((z_i, z_hat_i));
                } else if randex_messages.len() == self.n {
                    error!(
                        "Failed to recover share through randex with OEC: too many invalid shares"
                    );
                    // Unrecoverable
                    return Err(AdkgError::RandEx);
                }
            }
        }
    }

    async fn recv_next_adkg_msg<T>(
        &self,
        adkg_receiver: &mut T::ReceiveMessageStream,
    ) -> Result<(T::Identity, AdkgMessage<CG, H>), AdkgError>
    where
        T: Transport,
    {
        loop {
            let ReceivedMessage {
                sender, content, ..
            } = match adkg_receiver.next().await {
                Some(Ok(m)) => m,
                Some(Err(e)) => {
                    warn!("Node `{}` failed to recv: {e:?}", self.id);
                    continue;
                }
                None => {
                    error!("Node `{}` failed to recv: no more items in stream", self.id);
                    Err(AdkgError::RandEx)?
                }
            };

            let msg: AdkgMessage<CG, H> = match bson::from_slice(&content) {
                Ok(m) => m,
                Err(e) => {
                    warn!(
                        "Node `{}` received ADKG message from {sender} with bad format: {e:?}",
                        self.id
                    );
                    continue;
                }
            };

            return Ok((sender, msg));
        }
    }

    fn is_key_msg_valid(
        &self,
        ped_commits: &[CG],
        sender: &PartyId,
        msg: &AdkgKeyMessage<CG, H>,
    ) -> bool {
        // Validate the proofs
        if msg
            .z_j_proof
            .verify(&self.g, &msg.g_z_j, &self.pok_dst)
            .is_err()
        {
            warn!(?sender, "Failed to verify z_j proof");
            return false;
        }
        if msg
            .z_hat_j_proof
            .verify(&self.h, &msg.h_z_hat_j, &self.pok_dst)
            .is_err()
        {
            warn!(?sender, "Failed to verify z_hat_j proof");
            return false;
        }

        // Now that we know z_j = log_g g_z_j and z_hat_j = log_h h_z_hat_j, make sure that
        // commit(z_j, z_hat_j) == g_z_j + h_z_hat_j
        if ped_commits[sender] != msg.g_z_j + msg.h_z_hat_j {
            warn!(?sender, "Node sent proofs with invalid shares");
            return false;
        }

        // Valid message
        true
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
                if completed_acss_ids.len() == state.n - state.t {
                    if rbc_leader_sender
                        .take()
                        .expect("cannot enter condition twice")
                        .send(
                            bson::to_vec(&CompletedAcssSessions {
                                v: completed_acss_ids.clone().into_iter().collect(),
                            })
                            .unwrap(),
                        )
                        .is_ok()
                    {
                        info!(rbc_input = ?completed_acss_ids, "Node {} set its own RBC input", state.id);
                    } else {
                        error!(
                            rbc_input = ?completed_acss_ids,
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

                info!(rbc_output = ?rbc_parties, "Node {} completed RBC with sid `{j}`", state.id);

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
                    sender.send(coin_keys).unwrap_or_else(|_| {
                        panic!("receiver should not have been dropped at this point")
                    });

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
                debug!(
                    "ADKG CoinKeys provider of node `{id}` for ABA {j} exiting due to CancellationToken"
                );
            }

            _ = inner_fn => {
                debug!(
                    "ADKG CoinKeys provider of node `{id}` for ABA {j} exiting: keys transmitted"
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
    use ark_ec::{CurveGroup, PrimeGroup};
    use ark_std::UniformRand;
    use dcipher_network::topic::dispatcher::TopicDispatcher;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use digest::FixedOutputReset;
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
        H: Default + NamedDynDigest + FixedOutputReset + BlockSizeUser + Clone,
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

        // run adkg with reconstruction threshold of t & 2t
        run_adkg_test::<_, sha3::Sha3_256>(t, t, n, g, h, SEED).await;
        run_adkg_test::<_, sha3::Sha3_256>(2 * t, t, n, g, h, SEED).await;
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

        run_adkg_test::<_, sha3::Sha3_256>(t, t, n, g, h, SEED).await;
        run_adkg_test::<_, sha3::Sha3_256>(2 * t, t, n, g, h, SEED).await;
    }

    async fn run_adkg_test<CG, H>(
        t_reconstruction: usize,
        t: usize,
        n: usize,
        g: CG,
        h: CG,
        seed: &[u8],
    ) where
        CG: NamedCurveGroup + PointSerializeCompressed + PointDeserializeCompressed + HashToCurve,
        CG::ScalarField: FqSerialize + FqDeserialize,
        H: Default + NamedDynDigest + FixedOutputReset + BlockSizeUser + Clone + 'static,
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
                    h,
                    RetryStrategy::None,
                );
                let aba_config =
                    AbaCrain20Config::<_, _, sha3::Sha3_256>::new(i, n, t, g, RetryStrategy::None);

                Adkg::<_, H, _, _, _>::new(
                    i,
                    n,
                    t,
                    t_reconstruction,
                    g,
                    h,
                    rbc_config,
                    acss_config,
                    aba_config,
                )
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
        let s: CG::ScalarField = lagrange_interpolate_at::<CG>(&points[0..=t_reconstruction], 0);
        assert_eq!((g * s).into_affine(), group_pk.into_affine());

        // The group secret is the sum of the used ACSS secrets
        let acss_secrets = get_expected_acss_secrets::<CG>(n, seed);
        let expected_s: CG::ScalarField = used_sessions.iter().map(|i| acss_secrets[i]).sum();
        assert_eq!(s, expected_s);

        for d in dispatchers {
            d.stop().await;
        }
    }
}
