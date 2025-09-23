use config::network::NetworkConfig;
use omnievent::proto_types::{BlockSafety, EventField, RegisterNewEventRequest};

pub(crate) fn create_swap_requested(network_config: &NetworkConfig) -> RegisterNewEventRequest {
    RegisterNewEventRequest {
        chain_id: network_config.chain_id,
        address: network_config.router_address.to_vec().into(),
        event_name: "SwapRequested".to_string(),
        fields: vec![
            EventField {
                sol_type: "bytes32".to_string(),
                indexed: true,
            },
            EventField {
                sol_type: "uint256".to_string(),
                indexed: true,
            },
            EventField {
                sol_type: "uint256".to_string(),
                indexed: true,
            },
        ],
        block_safety: BlockSafety::Latest.into(),
    }
}

pub(crate) fn create_fee_updated_event(network_config: &NetworkConfig) -> RegisterNewEventRequest {
    RegisterNewEventRequest {
        chain_id: network_config.chain_id,
        address: network_config.router_address.to_vec().into(),
        event_name: "SwapRequestSolverFeeUpdated".to_string(),
        fields: vec![EventField {
            sol_type: "bytes32".to_string(),
            indexed: true,
        }],
        block_safety: BlockSafety::Latest.into(),
    }
}
pub(crate) fn create_swap_fulfilled(network_config: &NetworkConfig) -> RegisterNewEventRequest {
    RegisterNewEventRequest {
        chain_id: network_config.chain_id,
        address: network_config.router_address.to_vec().into(),
        event_name: "SwapFulfilled".to_string(),
        fields: vec![
            EventField {
                sol_type: "bytes32".to_string(),
                indexed: true,
            },
            EventField {
                sol_type: "uint256".to_string(),
                indexed: true,
            },
            EventField {
                sol_type: "uint256".to_string(),
                indexed: true,
            },
        ],
        block_safety: BlockSafety::Latest.into(),
    }
}

pub(crate) fn create_swap_verified(network_config: &NetworkConfig) -> RegisterNewEventRequest {
    RegisterNewEventRequest {
        chain_id: network_config.chain_id,
        address: network_config.router_address.to_vec().into(),
        event_name: "SolverPayoutFulfilled".to_string(),
        fields: vec![EventField {
            sol_type: "bytes32".to_string(),
            indexed: true,
        }],
        block_safety: BlockSafety::Latest.into(),
    }
}
