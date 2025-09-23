use crate::network_bus::NetworkBus;
use crate::omnievent::StateUpdate;
use alloy::primitives::FixedBytes;
use alloy::providers::DynProvider;

pub(crate) struct StateMachine {
    network_bus: NetworkBus<DynProvider>,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    request_id: FixedBytes<32>,
    state: String,
}

impl StateMachine {
    pub fn new(network_bus: NetworkBus<DynProvider>) -> Self {
        Self {
            network_bus,
            transactions: Vec::new(),
        }
    }

    pub async fn apply_state(&mut self, update: StateUpdate) -> Vec<Transaction> {
        match update {
            StateUpdate::Requested {
                request_id,
                chain_id,
            } => {
                self.transactions.push(Transaction {
                    request_id,
                    state: "requested".to_string(),
                });
            }
            StateUpdate::FeeUpdated { .. } => {
                // do nothing for now
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

        self.transactions.clone()
    }
}
