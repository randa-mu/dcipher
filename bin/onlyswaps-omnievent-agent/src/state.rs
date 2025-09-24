use crate::network_bus::NetworkBus;
use crate::omnievent::{StateType, StateUpdate};
use crate::serde::{LongNumber, ShortNumber};
use alloy::primitives::{Address, FixedBytes};
use alloy::providers::DynProvider;
use serde::Serialize;

pub(crate) struct StateMachine {
    network_bus: NetworkBus<DynProvider>,
    state: AppState,
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct AppState {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Transaction {
    pub request_id: FixedBytes<32>,
    pub src_chain_id: ShortNumber,
    pub dest_chain_id: ShortNumber,
    pub sender: Address,
    pub recipient: Address,
    pub amount: LongNumber,
    pub solver_fee: LongNumber,
    pub state: String,
    pub solver: Option<Address>,
    pub requested_time: ShortNumber,
    pub solved_time: Option<ShortNumber>,
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

    pub async fn apply_state(&mut self, update: StateUpdate) -> anyhow::Result<AppState> {
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
            StateType::Requested => {
                self.state.transactions.push(Transaction {
                    request_id,
                    src_chain_id: params.srcChainId.into(),
                    dest_chain_id: params.dstChainId.into(),
                    sender: params.sender,
                    recipient: params.recipient,
                    amount: params.amountOut.into(),
                    solver_fee: params.solverFee.into(),
                    state: "requested".to_string(),
                    solver: None,
                    requested_time: params.requestedAt.into(),
                    solved_time: None,
                });
            }
            StateType::FeeUpdated => {
                for t in &mut self.state.transactions {
                    if t.request_id == request_id {
                        t.solver_fee = params.solverFee.into()
                    }
                }
            }
            StateType::Verified => {
                for t in &mut self.state.transactions {
                    if t.request_id == request_id {
                        t.state = "verified".to_string();
                    }
                }
            }
            StateType::Fulfilled => {} // impossible because we handle it early
        }

        Ok(self.state.clone())
    }

    async fn apply_fulfilled_state(
        &mut self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<AppState> {
        let client =
            self.network_bus.networks.get(&chain_id).expect(
                "got a chain_id for a network we don't support - this shouldn't be possible",
            );
        let receipt = client.fetch_receipt(request_id).await?;
        for t in &mut self.state.transactions {
            if t.request_id == request_id {
                t.state = "fulfilled".to_string();
                t.solver = Some(receipt.solver);
                t.solved_time = Some(receipt.fulfilledAt.into());
            }
        }
        Ok(self.state.clone())
    }
}
