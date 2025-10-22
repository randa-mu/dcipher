//! Client for the OnlySwaps fee estimator service.

mod api_definitions;

pub use api_definitions::FeeEstimateResponse as FeeEstimate;

use alloy::primitives::{Address, U256};
use api_definitions::*;

pub const FEE_ESTIMATOR_ENDPOINT: &str = "https://fees.onlyswaps.dcipher.network/fees";

/// Client for estimating swap fees
#[derive(Clone, Debug)]
pub struct FeeEstimator {
    client: reqwest::Client,
    fee_endpoint: String,
}

/// A fee estimator error
pub type FeeEstimatorError = reqwest::Error;

impl Default for FeeEstimator {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            fee_endpoint: FEE_ESTIMATOR_ENDPOINT.to_owned(),
        }
    }
}

impl FeeEstimator {
    /// Creates a new fee estimator with the specified endpoint URL
    pub fn new(fee_endpoint: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            fee_endpoint,
        }
    }

    /// Estimates fees for a cross-chain swap
    pub async fn estimate_fees(
        &self,
        src_chain_id: u64,
        dest_chain_id: u64,
        amount: U256,
        src_token: Address,
        dest_token: Address,
    ) -> Result<FeeEstimate, FeeEstimatorError> {
        let request = FeeEstimateRequest {
            src_chain_id,
            dest_chain_id,
            amount,
            src_token,
            dest_token,
        };

        self.client
            .post(&self.fee_endpoint)
            .json(&request)
            .send()
            .await?
            .json::<FeeEstimateResponse>()
            .await
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::config::chain::{AVAX_FUJI, BASE_SEPOLIA};
    use crate::config::token::TokenTag;

    #[tokio::test]
    async fn test_fee_estimator_default_api() {
        let estimator = FeeEstimator::default();

        let src_chain = BASE_SEPOLIA.chain_id;
        let src_token = *BASE_SEPOLIA.supported_tokens.get(&TokenTag::RUSD).unwrap();

        let dst_chain = AVAX_FUJI.chain_id;
        let dst_token = *AVAX_FUJI.supported_tokens.get(&TokenTag::RUSD).unwrap();
        let amount = U256::from(1_000_000_000_000_000_000u64);

        let response = estimator
            .estimate_fees(src_chain, dst_chain, amount, src_token, dst_token)
            .await
            .expect("Should successfully get fee estimate");

        // Consistency check: approval should be >= transfer amount
        assert!(response.approval_amount >= response.transfer_amount);
    }
}
