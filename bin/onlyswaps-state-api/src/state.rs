use crate::network_bus::NetworkBus;
use crate::omnievent::{StateType, StateUpdate, StateUpdateSource};
use crate::serde::{LongNumber, ShortNumber};
use alloy::primitives::{Address, B256, FixedBytes, TxHash, U256};
use alloy::providers::DynProvider;
use generated::onlyswaps::i_router::IRouter::SwapRequestParametersWithHooks;
use std::hash::HashMap;
use serde::Serialize;
use std::fmt::Display;
use std::time::SystemTime;

type RequestId = B256;

pub(crate) struct StateMachine {
    network_bus: NetworkBus<DynProvider>,
    state: AppState,
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct AppState {
    pub transactions: HashMap<RequestId, SwapTransaction>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct SwapTransaction {
    pub request_id: RequestId,
    pub src_chain_id: ShortNumber,
    pub dest_chain_id: ShortNumber,
    pub sender: Address,
    pub recipient: Address,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: LongNumber,
    pub amount_out: LongNumber,
    pub verification_fee: LongNumber,
    pub solver_fee: LongNumber,
    pub state: String,
    pub solver: Option<Address>,
    pub requested_time: ShortNumber,
    pub solved_time: Option<ShortNumber>,
    pub verified_time: Option<ShortNumber>,
    pub requested_tx: TxHash,
    pub solved_tx: Option<TxHash>,
    pub verified_tx: Option<TxHash>,
}

impl StateMachine {
    pub fn new(network_bus: NetworkBus<DynProvider>) -> Self {
        Self {
            network_bus,
            state: AppState {
                transactions: Default::default(),
            },
        }
    }

    // apply state calls the RPCs to get the relevant params to update details about a transaction.
    // if we had perfect consumption of events, we could simply mutate transactions, but we can't be
    // sure of that so we rewrite the full tx details on each event just to ensure a full dataset
    pub async fn apply_state(&mut self, update: StateUpdate) -> anyhow::Result<AppState> {
        tracing::debug!(
            update = ?update,
            "applying state update"
        );
        let StateUpdate {
            chain_id,
            request_id,
            state_type,
            tx_hash,
            #[cfg_attr(not(feature = "metrics"), allow(unused))]
            source,
        } = update;

        // fulfilled requests need to go to the dest chain, so to reduce duplicate work
        // let's deal with them first
        if state_type == StateType::Fulfilled {
            return self
                .apply_fulfilled_state(chain_id, request_id, tx_hash, source)
                .await;
        }

        let client =
            self.network_bus.networks.get(&chain_id).expect(
                "got a chain_id for a network we don't support - this shouldn't be possible",
            );

        let params = client.fetch_parameters(request_id).await?;
        match state_type {
            StateType::Requested | StateType::FeeUpdated => {
                // need to preserve original tx hash
                let maybe_tx_hash = self
                    .state
                    .transactions
                    .remove(&request_id)
                    .map(|tx| tx.requested_tx);

                self.state.transactions.insert(
                    request_id,
                    SwapTransaction {
                        request_id,
                        src_chain_id: params.srcChainId.into(),
                        dest_chain_id: params.dstChainId.into(),
                        sender: params.sender,
                        recipient: params.recipient,
                        token_in: params.tokenIn,
                        token_out: params.tokenOut,
                        amount_in: calculate_amount_in(&params).into(),
                        amount_out: params.amountOut.into(),
                        solver_fee: params.solverFee.into(),
                        verification_fee: params.verificationFee.into(),
                        state: SwapState::Submitted.to_string(),
                        solver: None,
                        requested_time: params.requestedAt.into(),
                        solved_time: None,
                        verified_time: None,
                        // use original tx_hash in case of a FeeUpdated event
                        requested_tx: maybe_tx_hash.unwrap_or(tx_hash),
                        solved_tx: None,
                        verified_tx: None,
                    },
                );

                #[cfg(feature = "metrics")]
                // update metrics if the event is fresh
                if let StateUpdateSource::UpcomingStream = source {
                    if let StateType::FeeUpdated = state_type {
                        super::metrics::Metrics::report_fee_updated(
                            params.srcChainId,
                            params.dstChainId,
                            params.tokenIn,
                            params.tokenOut,
                        );
                    } else if let StateType::Requested = state_type {
                        super::metrics::Metrics::report_swap_requested(
                            params.srcChainId,
                            params.dstChainId,
                            params.tokenIn,
                            params.tokenOut,
                        );
                    }
                }
            }
            StateType::Verified => {
                let (maybe_solver, maybe_solved_time, maybe_requested_tx, maybe_solved_tx) = self
                    .state
                    .transactions
                    .remove(&request_id)
                    .map(|it| (it.solver, it.solved_time, it.requested_tx, it.solved_tx))
                    .unwrap_or_default();

                self.state.transactions.insert(
                    request_id,
                    SwapTransaction {
                        request_id,
                        src_chain_id: params.srcChainId.into(),
                        dest_chain_id: params.dstChainId.into(),
                        sender: params.sender,
                        recipient: params.recipient,
                        token_in: params.tokenIn,
                        token_out: params.tokenOut,
                        amount_in: calculate_amount_in(&params).into(),
                        amount_out: params.amountOut.into(),
                        solver_fee: params.solverFee.into(),
                        verification_fee: params.verificationFee.into(),
                        state: SwapState::Verified.to_string(),
                        requested_time: params.requestedAt.into(),
                        solved_time: maybe_solved_time,
                        solver: maybe_solver,
                        verified_time: Some(ShortNumber(now()?)),
                        requested_tx: maybe_requested_tx,
                        solved_tx: maybe_solved_tx,
                        verified_tx: Some(tx_hash),
                    },
                );

                #[cfg(feature = "metrics")]
                // report metrics if the event is fresh
                if let StateUpdateSource::UpcomingStream = source {
                    super::metrics::Metrics::report_swap_verified(
                        params.srcChainId,
                        params.dstChainId,
                        params.tokenIn,
                        params.tokenOut,
                    );
                }
            }
            StateType::Fulfilled => unreachable!("impossible because we handle it early"),
        }

        Ok(self.state.clone())
    }

    async fn apply_fulfilled_state(
        &mut self,
        chain_id: u64,
        request_id: FixedBytes<32>,
        tx_hash: TxHash,
        #[cfg_attr(not(feature = "metrics"), allow(unused))] // only used with metrics feature
        source: StateUpdateSource,
    ) -> anyhow::Result<AppState> {
        // we get details from the dest chain first
        let dest_chain_client =
            self.network_bus.networks.get(&chain_id).expect(
                "got a chain_id for a network we don't support - this shouldn't be possible",
            );
        let receipt = dest_chain_client.fetch_receipt(request_id).await?;

        // and use them to get the src chain details
        let src_client = self
            .network_bus
            .networks
            .get(&receipt.srcChainId.try_into()?)
            .expect("got a chain_id for a network we don't support - this shouldn't be possible");
        let params = src_client.fetch_parameters(request_id).await?;

        let (maybe_verified_time, maybe_requested_tx, maybe_verified_tx) = self
            .state
            .transactions
            .remove(&request_id)
            .map(|it| (it.verified_time, it.requested_tx, it.verified_tx))
            .unwrap_or_default();

        // then build the transaction
        self.state.transactions.insert(
            request_id,
            SwapTransaction {
                request_id,
                src_chain_id: params.srcChainId.into(),
                dest_chain_id: params.dstChainId.into(),
                sender: params.sender,
                recipient: params.recipient,
                token_in: params.tokenIn,
                token_out: params.tokenOut,
                amount_in: calculate_amount_in(&params).into(),
                amount_out: params.amountOut.into(),
                solver_fee: params.solverFee.into(),
                verification_fee: params.verificationFee.into(),
                solver: Some(receipt.solver),
                state: SwapState::Fulfilled.to_string(),
                requested_time: params.requestedAt.into(),
                solved_time: Some(receipt.fulfilledAt.into()),
                verified_time: maybe_verified_time,
                requested_tx: maybe_requested_tx,
                solved_tx: Some(tx_hash),
                verified_tx: maybe_verified_tx,
            },
        );

        #[cfg(feature = "metrics")]
        // report metrics if the event is fresh
        if let StateUpdateSource::UpcomingStream = source {
            super::metrics::Metrics::report_swap_fulfilled(
                params.srcChainId,
                params.dstChainId,
                params.tokenIn,
                params.tokenOut,
            );
        }

        Ok(self.state.clone())
    }
}

fn calculate_amount_in(params: &SwapRequestParametersWithHooks) -> U256 {
    params.amountOut + params.solverFee + params.verificationFee
}

fn now() -> anyhow::Result<U256> {
    Ok(U256::from(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
    ))
}

enum SwapState {
    Submitted,
    Fulfilled,
    Verified,
}
impl Display for SwapState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SwapState::Submitted => "submitted".to_string(),
            SwapState::Fulfilled => "fulfilled".to_string(),
            SwapState::Verified => "verified".to_string(),
        };
        write!(f, "{}", str)
    }
}
