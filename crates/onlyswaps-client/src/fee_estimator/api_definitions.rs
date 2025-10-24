//! Various definitions required by the onlyswaps-fees-api

use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

/// Request payload for fee estimation
#[derive(Debug, Serialize)]
pub struct FeeEstimateRequest {
    pub src_chain_id: u64,
    pub dest_chain_id: u64,
    pub amount: U256,
    pub src_token: Address,
    pub dest_token: Address,
}

/// Fee details for a specific chain
#[derive(Debug, Deserialize)]
pub struct ChainFeeDetails {
    /// Currency symbol (e.g., "AVAX", "ETH")
    pub currency: String,
    /// Token decimals
    pub decimals: u8,
    /// Swap fee in token's smallest unit
    pub swap_fee: String,
    /// Relay fee in token's smallest unit
    pub relay_fee: String,
    /// Basis points fee percentage
    pub bps: u64,
    /// Divisor for basis points calculation
    pub bps_divisor: u64,
}

/// Breakdown of total fees
#[derive(Debug, Deserialize)]
pub struct FeeBreakdown {
    /// Solver fee
    pub solver: U256,
    /// Network fee
    pub network: U256,
    /// Total fee (solver + network)
    pub total: U256,
}

/// Response from the fee estimation endpoint
#[derive(Debug, Deserialize)]
pub struct FeeEstimateResponse {
    /// Fee details for the source chain
    pub src: ChainFeeDetails,
    /// Fee details for the destination chain
    pub dest: ChainFeeDetails,
    /// Breakdown of fees
    pub fees: FeeBreakdown,
    /// Amount that will be transferred after fees
    pub transfer_amount: U256,
    /// Amount that needs to be approved for the swap
    pub approval_amount: U256,
    /// Unix timestamp of the estimate
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_schema_matches_api_contract() {
        // This is the actual response format from the API documentation
        let api_response = r#"
        {
          "src": {
            "currency": "ETH",
            "decimals": 18,
            "swap_fee": "569385310752",
            "relay_fee": "147861052",
            "bps": 50,
            "bps_divisor": 10000
          },
          "dest": {
            "currency": "AVAX",
            "decimals": 18,
            "swap_fee": "841392",
            "relay_fee": "62788",
            "bps": 50,
            "bps_divisor": 10000
          },
          "fees": {
            "solver": "40000000000000000",
            "network": "9800000000000000",
            "total": "49800000000000000"
          },
          "transfer_amount": "1960000000000000000",
          "approval_amount": "2000000000000000000",
          "timestamp": 1761137499
        }"#;

        // If this fails, our struct definitions don't match the API
        let _response: FeeEstimateResponse = serde_json::from_str(api_response)
            .expect("Our structs should be able to deserialize the API response");
    }
}
