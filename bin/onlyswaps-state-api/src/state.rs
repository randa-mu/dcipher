use crate::network_bus::NetworkBus;
use crate::omnievent::{StateType, StateUpdate};
use crate::serde::{LongNumber, ShortNumber};
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::DynProvider;
use generated::onlyswaps::router::IRouter::SwapRequestParameters;
use serde::Serialize;
use std::fmt::Display;
use std::time::SystemTime;

pub(crate) struct StateMachine {
    network_bus: NetworkBus<DynProvider>,
    state: AppState,
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct AppState {
    pub transactions: Vec<SwapTransaction>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SwapTransaction {
    pub request_id: FixedBytes<32>,
    pub src_chain_id: ShortNumber,
    pub dest_chain_id: ShortNumber,
    pub sender: Address,
    pub recipient: Address,
    pub amount_in: LongNumber,
    pub amount_out: LongNumber,
    pub verification_fee: LongNumber,
    pub solver_fee: LongNumber,
    pub state: String,
    pub solver: Option<Address>,
    pub requested_time: ShortNumber,
    pub solved_time: Option<ShortNumber>,
    pub verified_time: Option<ShortNumber>,
}

impl StateMachine {
    pub fn new(network_bus: NetworkBus<DynProvider>) -> Self {
        Self {
            network_bus,
            state: AppState {
                transactions: Vec::new(),
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
        } = update;

        // fulfilled requests need to go to the dest chain, so to reduce duplicate work
        // let's deal with them first
        if state_type == StateType::Fulfilled {
            return self.apply_fulfilled_state(chain_id, request_id).await;
        }

        let client =
            self.network_bus.networks.get(&chain_id).expect(
                "got a chain_id for a network we don't support - this shouldn't be possible",
            );

        let params = client.fetch_parameters(request_id).await?;
        match state_type {
            StateType::Requested | StateType::FeeUpdated => {
                self.state
                    .transactions
                    .retain(|t| t.request_id != request_id);
                self.state.transactions.push(SwapTransaction {
                    request_id,
                    src_chain_id: params.srcChainId.into(),
                    dest_chain_id: params.dstChainId.into(),
                    sender: params.sender,
                    recipient: params.recipient,
                    amount_in: calculate_amount_in(&params).into(),
                    amount_out: params.amountOut.into(),
                    solver_fee: params.solverFee.into(),
                    verification_fee: params.verificationFee.into(),
                    state: SwapState::Submitted.to_string(),
                    solver: None,
                    requested_time: params.requestedAt.into(),
                    solved_time: None,
                    verified_time: None,
                });
            }
            StateType::Verified => {
                let (maybe_solver, maybe_solved_time) = self
                    .state
                    .transactions
                    .iter()
                    .find(|t| t.request_id == request_id)
                    .map(|it| (it.solver, it.solved_time.clone()))
                    .unwrap_or((None, None));

                self.state
                    .transactions
                    .retain(|t| t.request_id != request_id);

                self.state.transactions.push(SwapTransaction {
                    request_id,
                    src_chain_id: params.srcChainId.into(),
                    dest_chain_id: params.dstChainId.into(),
                    sender: params.sender,
                    recipient: params.recipient,
                    amount_in: calculate_amount_in(&params).into(),
                    amount_out: params.amountOut.into(),
                    solver_fee: params.solverFee.into(),
                    verification_fee: params.verificationFee.into(),
                    state: SwapState::Verified.to_string(),
                    requested_time: params.requestedAt.into(),
                    solved_time: maybe_solved_time,
                    solver: maybe_solver,
                    verified_time: Some(ShortNumber(now()?)),
                });
            }
            StateType::Fulfilled => unreachable!("impossible because we handle it early"),
        }

        Ok(self.state.clone())
    }

    async fn apply_fulfilled_state(
        &mut self,
        chain_id: u64,
        request_id: FixedBytes<32>,
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

        // then build the transaction
        self.state
            .transactions
            .retain(|t| t.request_id != request_id);
        self.state.transactions.push(SwapTransaction {
            request_id,
            src_chain_id: params.srcChainId.into(),
            dest_chain_id: params.dstChainId.into(),
            sender: params.sender,
            recipient: params.recipient,
            amount_in: calculate_amount_in(&params).into(),
            amount_out: params.amountOut.into(),
            solver_fee: params.solverFee.into(),
            verification_fee: params.verificationFee.into(),
            solver: Some(receipt.solver),
            state: SwapState::Fulfilled.to_string(),
            requested_time: params.requestedAt.into(),
            solved_time: Some(receipt.fulfilledAt.into()),
            verified_time: None,
        });

        Ok(self.state.clone())
    }
}

fn calculate_amount_in(params: &SwapRequestParameters) -> U256 {
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
