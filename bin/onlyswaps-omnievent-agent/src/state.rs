use crate::network_bus::NetworkBus;
use crate::omnievent::StateUpdate;
use alloy::primitives::{FixedBytes, U256};
use alloy::providers::DynProvider;

pub(crate) struct StateMachine {
    network_bus: NetworkBus<DynProvider>,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    pub request_id: FixedBytes<32>,
    pub amount: U256,
    pub solver_fee: U256,
    pub state: String,
}

impl StateMachine {
    pub fn new(network_bus: NetworkBus<DynProvider>) -> Self {
        Self {
            network_bus,
            transactions: Vec::new(),
        }
    }

    pub async fn apply_state(&mut self, update: StateUpdate) -> anyhow::Result<Vec<Transaction>> {
        match update {
            StateUpdate::Requested {
                request_id,
                chain_id,
            } => {
                let client = self.network_bus.networks.get(&chain_id).expect(
                    "got a chain_id for a network we don't support - this shouldn't be possible",
                );
                let params = client.fetch_parameters(request_id).await?;
                self.transactions.push(Transaction {
                    request_id,
                    amount: params.amountOut,
                    solver_fee: params.solverFee,
                    state: "requested".to_string(),
                });
            }
            StateUpdate::FeeUpdated {
                chain_id,
                request_id,
            } => {
                let client = self.network_bus.networks.get(&chain_id).expect(
                    "got a chain_id for a network we don't support - this shouldn't be possible",
                );
                let params = client.fetch_parameters(request_id).await?;

                for t in &mut self.transactions {
                    if t.request_id == request_id {
                        t.solver_fee = params.solverFee
                    }
                }
            }
            StateUpdate::Fulfilled { request_id } => {
                for t in &mut self.transactions {
                    if t.request_id == request_id {
                        t.state = "fulfilled".to_string();
                    }
                }
            }
            StateUpdate::Verified { request_id } => {
                for t in &mut self.transactions {
                    if t.request_id == request_id {
                        t.state = "verified".to_string();
                    }
                }
            }
        }

        Ok(self.transactions.clone())
    }
}
