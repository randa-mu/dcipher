use futures::StreamExt;
mod handlers;
mod types;

// Re-export public types
pub use types::{AcssError, AcssMessage, ImplicateMessage};

use super::{Acss, AcssConfig};
use crate::adkg::types::ShareWithPoly;
use crate::helpers::PartyId;
use crate::network::{RetryStrategy, broadcast_with_self};
use crate::nizk::NIZKDleqProof;
use crate::rbc::ReliableBroadcastConfig;
use crate::{
    pke::ec_hybrid_chacha20poly1305::{self, MultiHybridCiphertext},
    rbc::{RbcPredicate, ReliableBroadcast},
    vss::feldman::{self},
};
use ark_ec::CurveGroup;
use ark_ff::FftField;
use async_trait::async_trait;
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::{ReceivedMessage, Transport, TransportSender};
use digest::DynDigest;
use digest::core_api::BlockSizeUser;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::select;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use types::{AcssBroadcastMessage, AcssStatus, FedVerifyPredicate, StateMachine};
use utils::dst::{NamedCurveGroup, NamedDynDigest, Rfc9380DstBuilder};
use utils::serialize::{
    SerializationError,
    fq::{FqDeserialize, FqSerialize},
    point::{PointDeserializeCompressed, PointSerializeCompressed},
};

const TOPIC: &str = "hbacss0";
const APPNAME: &[u8] = b"HBACSS0-v1";
const NIZK_DLEQ_SUFFIX: &[u8] = b"NIZK_DLEQ";

pub struct HbAcss0Config<CG, H, RBCConfig>
where
    CG: CurveGroup,
{
    id: PartyId,                   // Id of the party in [1, n]
    sk: CG::ScalarField,           // Long term secret key associated to pks[id - 1]
    pks: Vec<CG>,                  // Long term public keys of the other parties
    n: usize,                      // The number of parties taking part in the scheme
    t: usize,                      // The threshold of parties
    g: CG,                         // The generator g to be used
    rbc_config: Arc<RBCConfig>,    // Internal reliable broadcast protocol configuration
    retry_strategy: RetryStrategy, // Retry strategy when sending messages
    nizk_dleq_dst: Vec<u8>,        // Domain separation tag used for NIZK DLEQ proofs
    _h: PhantomData<fn(H) -> H>,   // Cryptographically secure hash function used internally.
}

impl<CG, H, RBCConfig> HbAcss0Config<CG, H, RBCConfig>
where
    CG: NamedCurveGroup,
    H: NamedDynDigest,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: PartyId,
        sk: CG::ScalarField,
        pks: Vec<CG>,
        rbc_config: Arc<RBCConfig>,
        n: usize,
        t: usize,
        g: CG,
        retry_strategy: RetryStrategy,
    ) -> Arc<Self> {
        // Generate a DST in the following format: HBACSS0-v1_%CURVE_NAME%_XMD:%HASH_NAME%_NIZK_DLEQ_
        // e.g.: HBACSS0-v1_BN254G1_XMD:SHA3-256_NIZK_DLEQ_
        let dst = Rfc9380DstBuilder::empty()
            .with_application_name(APPNAME.to_vec())
            .with_curve::<CG>()
            .with_hash::<H>()
            .with_suffix(NIZK_DLEQ_SUFFIX.to_vec())
            .build();

        Arc::new(Self {
            sk,
            pks,
            id,
            rbc_config,
            n,
            t,
            g,
            retry_strategy,
            nizk_dleq_dst: dst.into(),
            _h: PhantomData,
        })
    }
}

impl<CG, H, RBCConfig> HbAcss0Config<CG, H, RBCConfig>
where
    CG: CurveGroup,
{
    fn get_topic(prefix: &str) -> String {
        if prefix.is_empty() {
            TOPIC.to_owned()
        } else {
            format!("{prefix}/{TOPIC}")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Hbacss0Output<CG: CurveGroup> {
    pub share: CG::ScalarField,
    pub public_poly: Vec<CG>,
}

impl<CG: CurveGroup> From<Hbacss0Output<CG>> for ShareWithPoly<CG> {
    fn from(value: Hbacss0Output<CG>) -> Self {
        Self {
            public_poly: value.public_poly,
            share: value.share,
        }
    }
}

impl<'a, CG, H, RBCConfig> AcssConfig<'a, CG, PartyId> for HbAcss0Config<CG, H, RBCConfig>
where
    CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
    CG::ScalarField: FqSerialize + FqDeserialize,
    AcssBroadcastMessage<CG>: Serialize,
    H: Default + DynDigest + BlockSizeUser + Clone + 'static,
    RBCConfig: for<'lt_rbc> ReliableBroadcastConfig<'lt_rbc, PartyId> + 'a,
{
    type Output = Hbacss0Output<CG>;
    type Error = Box<AcssError>;

    fn new_instance_with_prefix<T>(
        self: &Arc<Self>,
        topic_prefix: String,
        transport: T,
    ) -> Result<impl Acss<CG, PartyId, Output = Self::Output, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        let topic = Self::get_topic(&topic_prefix);
        let rbc = self
            .rbc_config
            .new_instance_with_prefix(topic.clone(), transport.clone())
            .map_err(|_| AcssError::TransportInit)?;

        let mut transport = transport
            .get_transport_for(topic)
            .ok_or(AcssError::TransportInit)?;
        let receiver = transport
            .receiver_stream()
            .ok_or(AcssError::TransportInit)?;
        let sender = transport.sender().ok_or(AcssError::TransportInit)?;

        Ok(HbAcss0::<_, _, _, _, T::Transport> {
            config: self.clone(),
            rbc: Some(rbc),
            receiver,
            sender,
        })
    }
}

/// Implementation of the hbACSS0 scheme without batching described in https://eprint.iacr.org/2021/159.pdf.
/// We rely on hybrid encryption using an ephemeral key for the dealer, and long term keys for the rest of the parties.
struct HbAcss0<CG, H, RBCConfig, RBC, T>
where
    CG: CurveGroup,
    RBC: ReliableBroadcast<Identity = PartyId>,
    T: Transport,
{
    config: Arc<HbAcss0Config<CG, H, RBCConfig>>,
    rbc: Option<RBC>, // Option for interior mutability since RBC::start/listen moves it
    receiver: T::ReceiveMessageStream,
    sender: T::Sender,
}

struct HbAcss0Instance<CG, H, RBCConfig, TS>
where
    CG: CurveGroup,
    TS: TransportSender,
{
    config: Arc<HbAcss0Config<CG, H, RBCConfig>>,
    sender: TS,
}

impl<CG, H, RBCConfig, RBC, T> Acss<CG, PartyId> for HbAcss0<CG, H, RBCConfig, RBC, T>
where
    CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
    CG::ScalarField: FqSerialize + FqDeserialize,
    AcssBroadcastMessage<CG>: Serialize,
    H: Default + DynDigest + BlockSizeUser + Clone,
    RBC: ReliableBroadcast<Identity = PartyId>,
    RBCConfig: for<'a> ReliableBroadcastConfig<'a, PartyId>,
    T: Transport<Identity = PartyId>,
{
    type Error = Box<AcssError>;
    type Output = Hbacss0Output<CG>;

    async fn deal<RNG>(
        self,
        s: &CG::ScalarField,
        cancel: CancellationToken,
        output: oneshot::Sender<Self::Output>,
        rng: &mut RNG,
    ) -> Result<(), Self::Error>
    where
        RNG: RngCore + CryptoRng + Send,
    {
        let id = self.config.id;
        let res = select! {
            res = self.acss_dealer(
                s,
                cancel.child_token(),
                output,
                rng
            ) => {
                // Log and rewrap error
                if let Err(e) = res {
                    error!("ACSS of leader node {id} completed with an error: {e:?}.");
                    Err(e)?
                }

                info!("ACSS of leader node {id} successfully completed.");
                Ok(())
            }

            _ = cancel.cancelled() => {
                info!("ACSS of leader node {id} cancelled through token.");
                Ok(())
            }
        };

        res
    }

    async fn get_share<RNG>(
        self,
        expected_broadcaster: PartyId,
        cancel: CancellationToken,
        output: oneshot::Sender<Self::Output>,
        rng: &mut RNG,
    ) -> Result<(), Self::Error>
    where
        RNG: RngCore + CryptoRng + Send,
    {
        let id = self.config.id;
        let res = select! {
            res = self.acss_client_participate(
                expected_broadcaster,
                cancel.child_token(),
                output,
                rng
            ) => {
                // Log and rewrap error
                if let Err(e) = res {
                    error!("ACSS of node {} completed with an error: {e:?}.", id);
                    Err(e)?
                }

                info!("ACSS of node {} successfully completed.", id);
                Ok(())
            }

            _ = cancel.cancelled() => {
                info!("ACSS of node {} cancelled through token.", id);
                Ok(())
            }
        };

        res
    }
}

impl<CG, H, RBCConfig, RBC, T> HbAcss0<CG, H, RBCConfig, RBC, T>
where
    CG: CurveGroup,
    H: Default + DynDigest + BlockSizeUser + Clone,
    RBCConfig: for<'a> ReliableBroadcastConfig<'a, PartyId>,
    RBC: ReliableBroadcast<Identity = PartyId>,
    T: Transport<Identity = PartyId>,
{
    /// Beginning of the ACSS protocol as the dealer.
    async fn acss_dealer<RNG>(
        mut self,
        s: &CG::ScalarField,
        rbc_cancel: CancellationToken,
        output: oneshot::Sender<Hbacss0Output<CG>>,
        rng: &mut RNG,
    ) -> Result<(), Box<AcssError>>
    where
        AcssBroadcastMessage<CG>: Serialize,
        CG: PointSerializeCompressed + PointDeserializeCompressed,
        CG::ScalarField: FqSerialize + FqDeserialize,
        RNG: RngCore + CryptoRng,
    {
        // Feldman's Polynomial Commitment of degree t where p(0) = s
        let g = self.config.g;
        let vss_share = feldman::share(s, &g, self.config.n, self.config.t, rng);
        let public_poly = vss_share.get_public_poly();

        // Encrypt and Disperse
        // Each share is encrypted towards the receiving party
        let shares = PartyId::iter_all(self.config.n)
            .map(|i| -> Result<Vec<u8>, SerializationError> {
                vss_share
                    .get_party_secrets(i)
                    .expect("feldman output less than n shares") // gen n shares, loop n times
                    .ser()
            })
            .collect::<Result<Vec<Vec<u8>>, _>>()
            .map_err(|e| AcssError::Ser(e, "dealer failed to serialize vss shares"))?; // unexpected error, abort ACSS

        let enc_shares =
            ec_hybrid_chacha20poly1305::encrypt_multi(&shares, &self.config.pks, &g, rng)
                .map_err(|e| AcssError::HybridEncryption(e, "dealer failed to encrypt shares"))?; // unexpected error, abort ACSS

        // Disperse encrypted shares and public polynomial through the broadcast channel
        let broadcast = AcssBroadcastMessage {
            enc_shares,
            public_poly: public_poly.to_vec(),
        };
        let m = bson::to_vec(&broadcast)
            .map_err(|e| AcssError::BsonSer(e, "dealer failed to serialize broadcast message"))?; // unexpected error, abort ACSS

        match self.rbc.take().unwrap().start(&m, rbc_cancel).await {
            Ok(mp) => {
                if mp != m {
                    error!("Leader obtained an invalid m after RBC");
                    Err(AcssError::InconsistentRbc)?; // abort ACSS
                }
            }
            Err(e) => {
                error!("Leader failed to obtain a value after RBC, error {e:?}");
                Err(AcssError::FailedRbc(e.into()))?; // abort ACSS
            }
        }

        // Continue the execution of the acss protocol as a normal participant.
        let id = self.config.id;
        self.acss_continue(
            vss_share.get_party_secrets(id).copied(),
            output,
            &broadcast.enc_shares,
            public_poly,
            rng,
        )
        .await
    }

    /// Participate in the ACSS protocol to recover a secret share.
    async fn acss_client_participate<RNG>(
        mut self,
        expected_broadcaster: PartyId,
        rbc_cancel: CancellationToken,
        output: oneshot::Sender<Hbacss0Output<CG>>,
        rng: &mut RNG,
    ) -> Result<(), Box<AcssError>>
    where
        CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
        CG::ScalarField: FqSerialize + FqDeserialize,
        RNG: RngCore + CryptoRng,
    {
        let pred = FedVerifyPredicate {
            expected_broadcaster,
            i: self.config.id,
            sk: self.config.sk,
            pk: self.config.pks[self.config.id],
            g: self.config.g,
        };

        // Get message from RBC, or abort
        let m = self
            .rbc
            .take()
            .unwrap()
            .listen(&pred, expected_broadcaster, rbc_cancel)
            .await
            .map_err(|e| {
                error!(
                    "Node `{}` failed to obtain a value after RBC, error {e:?}",
                    self.config.id
                );
                AcssError::FailedRbc(e.into()) // abort ACSS
            })?;

        // Deserialize message, or abort
        let m = bson::from_slice::<AcssBroadcastMessage<CG>>(&m).map_err(|e| {
            error!(
                "Node `{}` failed to deserialize RBC message, error {e:?}",
                self.config.id
            );
            AcssError::FailedRbc(e.into()) // abort ACSS
        })?;

        // Decrypt and validate share
        let enc_shares = &m.enc_shares;
        let shared_key = enc_shares.derive_shared_key(&self.config.sk);
        let public_poly = &m.public_poly;

        // If the share is valid, the nodes enters the reconstruction process
        // otherwise, the node enters the recovery process
        let share = feld_eval_verify(
            enc_shares,
            public_poly,
            &self.config.g,
            self.config.id,
            &shared_key,
            &self.config.pks[self.config.id],
        )
        .ok();
        self.acss_continue(share, output, &m.enc_shares, public_poly, rng)
            .await
    }

    /// Execute the agreement / implication / share recovery part of the protocol.
    async fn acss_continue<RNG>(
        self,
        share: Option<CG::ScalarField>,
        output: oneshot::Sender<Hbacss0Output<CG>>,
        enc_shares: &MultiHybridCiphertext<CG>,
        public_poly: &[CG],
        rng: &mut RNG,
    ) -> Result<(), Box<AcssError>>
    where
        CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
        CG::ScalarField: FqSerialize + FqDeserialize,
        CG::ScalarField: FftField,
        RNG: RngCore + CryptoRng,
    {
        let Self {
            sender,
            mut receiver,
            config,
            ..
        } = self;
        let hbacss0 = HbAcss0Instance {
            config: config.clone(),
            sender,
        };

        let mut state_machine = StateMachine::<CG> {
            status: AcssStatus::New,
            nodes_oks: HashMap::new(),
            nodes_readys: HashMap::new(),
            shares_recovery: HashMap::new(),
            output: Some(output), // require an option since we move output upon sending
        };

        // If the share is valid, prepare Ok message
        // otherwise, the node prepares an implicate message and enters the recovery process
        if let Some(share) = share {
            info!(
                "Node `{}` received a valid share. Starting ACSS protocol.",
                config.id
            );

            // Update state
            state_machine.status = AcssStatus::WaitingForOks(share);

            // Share is valid, send Ok to all other nodes
            if let Err(e) =
                broadcast_with_self(&AcssMessage::Ok, &config.retry_strategy, &hbacss0.sender).await
            {
                error!("Node `{}` failed to broadcast ok message: {e:?}", config.id)
            }
        } else {
            // Share is invalid
            warn!(
                "Node `{}` received an invalid share. Starting recovery protocol.",
                config.id
            );

            // Update state
            state_machine.status = AcssStatus::ShareRecovery;

            // We need to send the shared secret (K_i = [s_i] PK_d) and a proof
            // that we know an s s.t. log_{PK_d} K_i = log_G PK_i = s
            let shared_secret = enc_shares.derive_shared_key(&config.sk);
            let pi = NIZKDleqProof::<CG, sha3::Sha3_256>::prove(
                &config.sk,
                &config.g,
                &enc_shares.sender_pk,
                &config.pks[config.id],
                &shared_secret,
                &config.nizk_dleq_dst,
                rng,
            )
            .map_err(|e| AcssError::Nizk(e, "failed to compute implicate proof"))?; // unexpected error, abort ACSS

            // Share is invalid, send Implicate to all nodes
            let implicate_msg =
                AcssMessage::Implicate(ImplicateMessage::from_curvegroup(&pi, &shared_secret)?);
            if let Err(e) =
                broadcast_with_self(&implicate_msg, &config.retry_strategy, &hbacss0.sender).await
            {
                error!(
                    "Node `{}` failed to broadcast implicate message: {e:?}",
                    config.id
                )
            }
        };

        loop {
            let ReceivedMessage {
                sender, content, ..
            } = match receiver.next().await {
                Some(Ok(m)) => m,
                Some(Err(e)) => {
                    warn!("Node `{}` failed to recv: {e:?}", config.id);
                    continue;
                }
                None => {
                    error!(
                        "Node `{}` failed to recv: no more items in stream",
                        config.id
                    );
                    return Err(AcssError::NoMessages.into());
                }
            };
            let msg: AcssMessage = match bson::from_slice(&content) {
                Ok(m) => m,
                Err(e) => {
                    error!(error = ?e, "Node `{}` failed to deserialize message", config.id);
                    continue;
                }
            };

            match msg {
                AcssMessage::Ok => hbacss0.ok_handler(sender, &mut state_machine).await,

                AcssMessage::Ready => {
                    hbacss0
                        .ready_handler(sender, &mut state_machine, public_poly)
                        .await
                }

                AcssMessage::Implicate(ski) => {
                    hbacss0
                        .implicate_handler(
                            &ski,
                            enc_shares,
                            public_poly,
                            sender,
                            &mut state_machine,
                        )
                        .await
                }

                AcssMessage::ShareRecovery(shared_key) => {
                    hbacss0
                        .recovery_handler(
                            &shared_key,
                            enc_shares,
                            public_poly,
                            sender,
                            &mut state_machine,
                        )
                        .await
                }
            }

            // Exit if the ACSS is complete.
            if let AcssStatus::Complete(_) = state_machine.status {
                return Ok(());
            }
        }
    }
}

/// Predicate used to determine the validity of the share within the RBC protocol.
#[async_trait]
impl<CG> RbcPredicate for FedVerifyPredicate<CG>
where
    CG: CurveGroup + PointSerializeCompressed,
    CG::ScalarField: FqDeserialize,
    AcssBroadcastMessage<CG>: for<'de> Deserialize<'de>,
{
    async fn predicate(&self, sender: PartyId, m: &[u8]) -> bool {
        if sender != self.expected_broadcaster {
            warn!(
                "Node `{}` received a message from an unexpected broadcaster (got `{sender}`, expected `{}`",
                self.i, self.expected_broadcaster
            )
        }

        let Ok(m) = bson::from_slice::<AcssBroadcastMessage<CG>>(m) else {
            return false;
        };

        // Decrypt and validate share
        let enc_shares = &m.enc_shares;
        let shared_key = enc_shares.derive_shared_key(&self.sk);
        let public_poly = &m.public_poly;
        feld_eval_verify(
            enc_shares,
            public_poly,
            &self.g,
            self.i,
            &shared_key,
            &self.pk,
        )
        .is_ok()
    }
}

/// Verify that a hybrid encryption ciphertext can be decrypted and is a valid Feldman share for party i.
fn feld_eval_verify<CG>(
    ct: &MultiHybridCiphertext<CG>,
    public_poly: &[CG],
    g: &CG,
    i: PartyId,
    shared_key: &CG,
    recipient_pk: &CG,
) -> Result<CG::ScalarField, ()>
where
    CG: CurveGroup + PointSerializeCompressed,
    CG::ScalarField: FqDeserialize,
    AcssBroadcastMessage<CG>: for<'de> Deserialize<'de>,
{
    // Try to decrypt the ciphertext and deserialize it, or return Err(())
    let pt = ct
        .decrypt_one_with_shared_key(i.as_index(), shared_key, recipient_pk)
        .map_err(|_| ())?;
    let share = CG::ScalarField::deser(&pt).map_err(|_| ())?;

    // Try to verify the share, or return Err(())
    if feldman::eval_verify(public_poly, i, &share, g).is_ok() {
        Ok(share)
    } else {
        Err(())
    }
}

impl ImplicateMessage {
    fn from_curvegroup<CG, H>(pi: &NIZKDleqProof<CG, H>, k: &CG) -> Result<Self, Box<AcssError>>
    where
        CG: CurveGroup + PointSerializeCompressed,
        NIZKDleqProof<CG, H>: Serialize,
    {
        let pi = bson::to_vec(pi)
            .map_err(|e| Box::new(AcssError::BsonSer(e, "failed to serialize pi")))?;
        let k = k
            .ser()
            .map_err(|e| Box::new(AcssError::Ser(e, "failed to deserialize k")))?;

        Ok(Self { pi, k })
    }

    fn get_pi<CG, H>(&self) -> Result<NIZKDleqProof<CG, H>, Box<AcssError>>
    where
        CG: CurveGroup,
        NIZKDleqProof<CG, H>: for<'de> Deserialize<'de>,
    {
        bson::from_slice(&self.pi)
            .map_err(|e| AcssError::BsonDe(e, "failed to deserialize pi").into())
    }

    fn get_k<CG>(&self) -> Result<CG, Box<AcssError>>
    where
        CG: CurveGroup + PointDeserializeCompressed,
    {
        CG::deser(&self.k).map_err(|e| AcssError::Ser(e, "failed to deserialize k").into())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::PartyId;
    use crate::rbc::r4::Rbc4RoundsConfig;
    use crate::vss::acss::AcssConfig;
    use crate::vss::acss::hbacss0::{APPNAME, HbAcss0Config, NIZK_DLEQ_SUFFIX};
    use crate::{
        helpers::{lagrange_interpolate_at, u64_from_usize},
        network::RetryStrategy,
        vss::acss::Acss,
    };
    use ark_bn254::Bn254;
    use ark_ec::{Group, pairing::Pairing};
    use ark_std::UniformRand;
    use dcipher_network::topic::dispatcher::TopicDispatcher;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use rand::rngs::OsRng;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use tokio::sync::oneshot;
    use tokio::task;
    use tokio::task::JoinSet;
    use tokio_util::sync::CancellationToken;

    type G = <Bn254 as Pairing>::G1;
    type ScalarField = <<Bn254 as Pairing>::G1 as Group>::ScalarField;

    #[tokio::test]
    async fn test_acss_all_parties() {
        _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        let t = 2;
        let n = 3 * t + 1;
        let g = G::generator();

        let s = ScalarField::rand(&mut rand::thread_rng());
        let mut sks: VecDeque<ScalarField> = (1..=n)
            .map(|_| ScalarField::rand(&mut rand::thread_rng()))
            .collect();
        let pks: Vec<G> = sks.iter().map(|sk| g * sk).collect();

        let (_dispatchers, mut transports): (Vec<_>, VecDeque<_>) =
            MemoryNetwork::get_transports(PartyId::iter_all(n))
                .into_iter()
                .map(|t| {
                    let mut dispatcher = TopicDispatcher::new();
                    let tbt = dispatcher.start(t);
                    (dispatcher, tbt)
                })
                .collect();
        let mut rbcs: VecDeque<_> = PartyId::iter_all(n)
            .map(|i| Rbc4RoundsConfig::new(i, n, t, &RetryStrategy::None))
            .collect();

        let mut tasks = JoinSet::new();
        tasks.spawn({
            let transport = transports.pop_front().unwrap();
            let sk = sks.pop_front().unwrap();
            let pks = pks.clone();
            let rbc_config = rbcs.pop_front().unwrap();
            let acss_config = HbAcss0Config::<_, sha3::Sha3_256, _>::new(
                1usize.into(),
                sk,
                pks,
                rbc_config,
                n,
                t,
                g,
                RetryStrategy::None,
            );

            async move {
                let (sender, receiver) = oneshot::channel();
                let cancellation_token = CancellationToken::new();

                let acss_thread = task::spawn(async move {
                    let acss = acss_config
                        .new_instance_with_prefix("hbacss0".to_owned(), Arc::new(transport))
                        .expect("failed to create acss instance");
                    acss.deal(&s, cancellation_token, sender, &mut OsRng).await
                });

                let share = receiver.await.expect("acss thread dropped oneshot sender");
                // Wait for acss to complete
                acss_thread
                    .await
                    .expect("failed to join thread")
                    .expect("acss failed");
                (1, share)
            }
        });

        for i in 2..=n {
            let transport = transports.pop_front().unwrap();
            let sk = sks.pop_front().unwrap();
            let pks = pks.clone();
            let rbc_config = rbcs.pop_front().unwrap();
            let acss_config = HbAcss0Config::<_, sha3::Sha3_256, _>::new(
                i.into(),
                sk,
                pks,
                rbc_config,
                n,
                t,
                g,
                RetryStrategy::None,
            );

            tasks.spawn(async move {
                let (sender, receiver) = oneshot::channel();
                let cancellation_token = CancellationToken::new();
                let acss_task = task::spawn(async move {
                    let acss = acss_config
                        .new_instance_with_prefix("hbacss0".to_owned(), Arc::new(transport))
                        .expect("failed to create acss instance");
                    acss.get_share(PartyId(1), cancellation_token, sender, &mut OsRng)
                        .await
                });

                let share = receiver.await.expect("acss thread dropped oneshot sender");
                // Wait for acss to complete
                acss_task
                    .await
                    .expect("failed to join thread")
                    .expect("acss failed");
                (u64_from_usize(i), share)
            });
        }

        let mut shares = vec![];
        while let Some(res) = tasks.join_next().await {
            assert!(res.is_ok());
            let (i, out) = res.unwrap();

            shares.push((i, out.share));
        }

        let s = lagrange_interpolate_at::<G>(&shares[0..=t], 0);
        let s2 = lagrange_interpolate_at::<G>(&shares[t..=2 * t], 0);

        assert_eq!(s, s2)
    }

    #[test]
    fn should_use_correct_dst() {
        use ark_std::Zero;

        let t = 2;
        let n = 3 * t + 1;

        let rbc_config = Rbc4RoundsConfig::new(PartyId(1), n, t, &RetryStrategy::None);
        let config = HbAcss0Config::<_, sha3::Sha3_256, _>::new(
            PartyId(1),
            ark_bn254::Fr::zero(),
            vec![ark_bn254::G1Projective::zero(); n],
            Arc::new(rbc_config),
            n,
            t,
            ark_bn254::G1Projective::zero(),
            RetryStrategy::None,
        );

        let expected_dst = [APPNAME, b"_BN254G1_XMD:SHA3-256_", NIZK_DLEQ_SUFFIX, b"_"].concat();
        assert_eq!(config.nizk_dleq_dst.as_slice(), expected_dst);
    }
}
