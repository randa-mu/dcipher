use crate::chain_state::NetworkBus;
use crate::chain_state_pending::{RequestId, Verification};
use crate::chain_state_resolver::{ChainState, ChainStateResolver};
use crate::config::AppConfig;
use crate::evaluator::Evaluator;
use crate::retry_runtime::RetrySender;
use crate::signing::{NetworkedSigner, OnlySwapsSigner, SignedVerification};
use crate::transport::create_libp2p_transport;
use alloy::providers::DynProvider;
use anyhow::Context;
use ark_bn254::Bn254;
use async_trait::async_trait;
use dcipher_signer::bls::BlsPairingSigner;
use std::sync::Arc;

#[async_trait]
pub trait ControlPlane {
    async fn resolve_state(
        &self,
        verification: Verification<RequestId>,
    ) -> anyhow::Result<ResolvedState>;
    async fn evaluate_state(&self, evaluation: ResolvedState) -> anyhow::Result<ResolvedState>;
    async fn sign_state(&self, valid_state: ResolvedState) -> anyhow::Result<SignedVerification>;
    async fn submit_state(
        &self,
        verification: SignedVerification,
    ) -> anyhow::Result<SignedVerification>;
    async fn handle_error(&self, verification: &VerificationError, retry: RetrySender<Event>);
}
#[derive(Debug, Clone)]
pub struct ResolvedState {
    pub verification: Verification<RequestId>,
    pub chain_state: ChainState,
}

#[derive(Eq, PartialEq, Clone)]
pub(crate) enum Event {
    NewVerification(Verification<RequestId>),
}

pub enum VerificationError {
    Resolve(Verification<RequestId>),
    Evaluate(ResolvedState),
    Sign(ResolvedState),
    Submit(SignedVerification),
}

pub struct DefaultControlPlane {
    // the network bus provides smart contract access across all configured blockchains
    network_bus: Arc<NetworkBus<DynProvider>>,

    // the `resolver` fetches the current src and dest states from a given request_id so we can evaluate
    // whether a swap has truly been completed and should be signed off
    state_resolver: ChainStateResolver,

    // the `signer` encapsulates everything related to gossiping, verifying, and aggregating partial
    // signatures to/from other committee members using libp2p.
    signer: OnlySwapsSigner<NetworkedSigner<BlsPairingSigner<Bn254>>>,
}

impl DefaultControlPlane {
    pub async fn new(
        app_config: &AppConfig,
        network_bus: Arc<NetworkBus<DynProvider>>,
    ) -> anyhow::Result<Self> {
        let state_resolver = ChainStateResolver::new(network_bus.clone());

        let transport = create_libp2p_transport(
            &app_config.longterm_secret.libp2p_sk,
            &app_config.committee_config,
        )?;
        let networked_signer = NetworkedSigner::new(app_config, transport)?;
        let signer = OnlySwapsSigner::new(networked_signer);
        tracing::info!(
            multiaddr = app_config.listen_addr.to_string(),
            n = app_config.committee_config.n,
            signing_threshold = app_config.committee_config.signing_threshold,
            "threshold signer created"
        );
        Ok(Self {
            signer,
            network_bus,
            state_resolver,
        })
    }
}
#[async_trait]
impl ControlPlane for DefaultControlPlane {
    async fn resolve_state(
        &self,
        verification: Verification<RequestId>,
    ) -> anyhow::Result<ResolvedState> {
        let chain_state = self.state_resolver.resolve_state(&verification).await?;
        Ok(ResolvedState {
            chain_state,
            verification,
        })
    }

    async fn evaluate_state(&self, evaluation: ResolvedState) -> anyhow::Result<ResolvedState> {
        let chain_state = Evaluator::evaluate(evaluation.chain_state)?;
        Ok(ResolvedState {
            chain_state,
            verification: evaluation.verification,
        })
    }

    async fn sign_state(&self, s: ResolvedState) -> anyhow::Result<SignedVerification> {
        let solver = s.chain_state.transfer_receipt.solver;
        let src_chain_id = s.chain_state.swap_params.srcChainId;
        let signature = self
            .signer
            .sign(&solver, &s.chain_state.swap_params)
            .await
            .context("signing failed somehow")?;

        Ok(SignedVerification {
            request_id: s.verification.request_id,
            src_chain_id,
            solver,
            signature,
        })
    }

    async fn submit_state(
        &self,
        verification: SignedVerification,
    ) -> anyhow::Result<SignedVerification> {
        self.network_bus.submit_verification(&verification).await?;
        Ok(verification)
    }

    async fn handle_error(
        &self,
        err: &VerificationError,
        retry: RetrySender<Event>,
    ) {
        match err {
            VerificationError::Resolve(verification) => retry
                .send(verification.clone().into())
                .await
                .expect("error sending on retry channel"),

            VerificationError::Evaluate(state) => retry
                .send(state.verification.clone().into())
                .await
                .expect("error sending on retry channel"),

            VerificationError::Sign(state) => retry
                .send(state.verification.clone().into())
                .await
                .expect("error sending on retry channel"),

            VerificationError::Submit(submit) => tracing::error!(
                src_chain_id = submit.src_chain_id.to_string(),
                request_id = submit.request_id.to_string(),
                "dont support resubmit just yet :("
            ),
        }
    }
}

impl From<Verification<RequestId>> for Event {
    fn from(value: Verification<RequestId>) -> Self {
        Self::NewVerification(value)
    }
}
