//! Definitions used to specify & build swap requests.

use crate::client::routing::SwapRouting;
use crate::config::chain::ChainConfig;
use crate::config::token::TokenTag;
use alloy::primitives::{Address, U256};

/// Details required to execute a swap with only swaps
#[derive(Copy, Clone, Debug)]
pub struct OnlySwapsRequest {
    pub recipient: Address,
    pub amount: U256,
    pub fee: U256,
    pub route: SwapRouting,
}

/// A builder for constructing an [`OnlySwapsRequest`].
///
/// All required fields
/// must be set before calling [`build`](Self::build), which returns `None` if
/// any required field is missing.
///
/// # Examples
/// Manually setting amount and fees.
/// ```
/// use alloy::primitives::{Address, U256};
/// use onlyswaps_client::client::OnlySwapsRequestBuilder;
/// use onlyswaps_client::client::routing::SwapRouting;
/// use onlyswaps_client::config::chain::{AVAX_FUJI, BASE_SEPOLIA};
/// use onlyswaps_client::config::token::TokenTag;
///
/// #[tokio::main]
/// async fn main() {
///     let request = OnlySwapsRequestBuilder::new()
///         .recipient(Address::default()) // do not use that address
///         .amount(U256::from(1_000_000_000_000_000_000u128)) // 1e18
///         .solver_fee(U256::from(1_000_000_000_000_000_000u128)) // 1e18
///         .route(SwapRouting::new_same_token_from_configs(&BASE_SEPOLIA, &AVAX_FUJI, &TokenTag::RUSD))
///         .build()
///         .expect("a valid builder");
/// }
/// ```
///
/// Automatic solver fee estimation
/// ```
/// use alloy::primitives::{Address, U256};
/// use onlyswaps_client::client::OnlySwapsRequestBuilder;
/// use onlyswaps_client::client::routing::SwapRouting;
/// use onlyswaps_client::config::chain::{AVAX_FUJI, BASE_SEPOLIA};
/// use onlyswaps_client::config::token::TokenTag;
/// use onlyswaps_client::FeeEstimator;
///
/// #[tokio::main]
/// async fn main() {
///     let request = OnlySwapsRequestBuilder::new()
///         .recipient(Address::default()) // do not use that address
///         .amount(U256::from(1_000_000_000_000_000_000u128)) // 1e18
///         .route(SwapRouting::new_same_token_from_configs(&BASE_SEPOLIA, &AVAX_FUJI, &TokenTag::RUSD))
///         .estimate_fee(&FeeEstimator::default())
///         .await
///         .expect("fee estimation to work")
///         .build()
///         .expect("a valid builder");
/// }
/// ```
///
/// Swapping an exact amount.
/// ```
/// use alloy::primitives::{Address, U256};
/// use onlyswaps_client::client::OnlySwapsRequestBuilder;
/// use onlyswaps_client::client::routing::SwapRouting;
/// use onlyswaps_client::config::chain::{AVAX_FUJI, BASE_SEPOLIA};
/// use onlyswaps_client::config::token::TokenTag;
/// use onlyswaps_client::FeeEstimator;
///
/// #[tokio::main]
/// async fn main() {
///     let request = OnlySwapsRequestBuilder::new()
///         .recipient(Address::default()) // do not use that address
///         .route(SwapRouting::new_same_token_from_configs(&BASE_SEPOLIA, &AVAX_FUJI, &TokenTag::RUSD))
///         .exact_amount(U256::from(1_000_000_000_000_000_000u128), &FeeEstimator::default())
///         .await
///         .expect("fee estimation to work")
///         .build()
///         .expect("a valid builder");
/// }
/// ```
#[derive(Clone, Default)]
pub struct OnlySwapsRequestBuilder {
    recipient: Option<Address>,
    amount: Option<U256>,
    solver_fee: Option<U256>,
    src_chain: Option<u64>,
    dst_chain: Option<u64>,
    src_token: Option<Address>,
    dst_token: Option<Address>,
}

#[derive(thiserror::Error, Debug)]
pub enum OnlySwapsRequestBuilderError {
    #[error("failed to calculate amount due to overflow")]
    IntOverflow,

    #[cfg(feature = "fee-estimator")]
    #[error("failed to obtain a fee estimate")]
    FeeEstimate(#[from] crate::FeeEstimatorError),
}

impl OnlySwapsRequestBuilder {
    /// Creates a new builder with all fields unset.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the recipient address for the swap.
    pub fn recipient(mut self, recipient: Address) -> Self {
        self.recipient = Some(recipient);
        self
    }

    /// Sets the maximum amount for the swap. Note that you will receive less than amount on
    /// the destination chain as this does not take the network fee into account.
    pub fn amount(mut self, amount: U256) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the solver fee for the swap.
    pub fn solver_fee(mut self, solver_fee: U256) -> Self {
        self.solver_fee = Some(solver_fee);
        self
    }

    /// Sets the source chain ID.
    pub fn source_chain(mut self, src_chain: u64) -> Self {
        self.src_chain = Some(src_chain);
        self
    }

    /// Sets the destination chain ID.
    pub fn destination_chain(mut self, dst_chain: u64) -> Self {
        self.dst_chain = Some(dst_chain);
        self
    }

    /// Sets the source token tag.
    pub fn source_token(mut self, src_token: Address) -> Self {
        self.src_token = Some(src_token);
        self
    }

    /// Sets the destination token tag.
    pub fn destination_token(mut self, dst_token: Address) -> Self {
        self.dst_token = Some(dst_token);
        self
    }

    /// Sets routing parameters from an existing [`SwapRouting`].
    ///
    /// This is a convenience method that sets source/destination chains and tokens
    /// from a routing object in a single call.
    pub fn route(mut self, route: SwapRouting) -> Self {
        self.src_chain = Some(route.src_chain);
        self.dst_chain = Some(route.dst_chain);
        self.src_token = Some(route.src_token);
        self.dst_token = Some(route.dst_token);
        self
    }

    /// Specifies the source chain parameters using a [`ChainConfig`]
    pub fn source_chain_config(mut self, src_chain: &ChainConfig, src_token: &TokenTag) -> Self {
        let src_token = src_chain
            .supported_tokens
            .get(src_token)
            .expect("invalid token for source chain");
        self.src_chain = Some(src_chain.chain_id);
        self.src_token = Some(*src_token);
        self
    }

    /// Specifies the source chain parameters using a [`ChainConfig`]
    pub fn destination_chain_config(
        mut self,
        dst_chain: &ChainConfig,
        dst_token: &TokenTag,
    ) -> Self {
        let dst_token = dst_chain
            .supported_tokens
            .get(dst_token)
            .expect("invalid token for source chain");
        self.dst_chain = Some(dst_chain.chain_id);
        self.dst_token = Some(*dst_token);
        self
    }

    /// Estimate the solver fee using the fee estimator
    #[cfg(feature = "fee-estimator")]
    pub async fn estimate_fee(
        mut self,
        estimator: &crate::FeeEstimator,
    ) -> Result<Self, OnlySwapsRequestBuilderError> {
        let amount = self
            .amount
            .expect("cannot estimate fees without a desired amount");
        let src_chain = self
            .src_chain
            .expect("cannot estimate fees without a source chain");
        let src_token = self
            .src_token
            .expect("cannot estimate fees without a source token");
        let dst_chain = self
            .dst_chain
            .expect("cannot estimate fees without a destination chain");
        let dst_token = self.dst_token.unwrap_or(src_token);

        let estimate = estimator
            .estimate_fees(src_chain, dst_chain, amount, src_token, dst_token)
            .await?;
        self.solver_fee = Some(estimate.fees.solver);
        Ok(self)
    }

    /// Swap an exact amount of tokens, dynamically setting both amount and fee
    #[cfg(feature = "fee-estimator")]
    pub async fn exact_amount(
        mut self,
        exact_amount: U256,
        estimator: &crate::FeeEstimator,
    ) -> Result<Self, OnlySwapsRequestBuilderError> {
        let src_chain = self
            .src_chain
            .expect("cannot estimate fees without a source chain");
        let src_token = self
            .src_token
            .expect("cannot estimate fees without a source token");
        let dst_chain = self
            .dst_chain
            .expect("cannot estimate fees without a destination chain");
        let dst_token = self.dst_token.unwrap_or(src_token);

        // Do a first call to get BPS values
        let bps = estimator
            .estimate_fees(src_chain, dst_chain, exact_amount, src_token, dst_token)
            .await?;

        // In solidity-land, amount is actually equal to exact_amount + network fee where
        // network fee = amount * BPS / BPS_DIVISOR.
        // To swap an exact amount, we need to set amount dynamically as,
        // amount = exact_amount / (1 - BPS / BPS_DIVISOR)
        //        = exact_amount * BPS_DIVISOR / (BPS_DIVISOR - BPS) (to prevent floating point errors)
        let amount = || {
            let num = exact_amount.checked_mul(U256::from(bps.src.bps_divisor))?;
            let denum = bps.src.bps_divisor.checked_sub(bps.src.bps)?;

            Some(num / U256::from(denum))
        };
        let amount = amount().ok_or(OnlySwapsRequestBuilderError::IntOverflow)?;

        // Get the actual solver fee estimate with the updated amount
        let estimate = estimator
            .estimate_fees(src_chain, dst_chain, amount, src_token, dst_token)
            .await?;

        self.amount = Some(amount);
        self.solver_fee = Some(estimate.fees.solver);
        Ok(self)
    }

    /// Builds an [`OnlySwapsRequest`] from the configured parameters.
    ///
    /// Returns `None` if any required field is missing (recipient, amount, fee,
    /// source_chain, destination_chain, or source_token).
    ///
    /// If no destination token is specified, the source token is used as the destination.
    pub fn build(self) -> Option<OnlySwapsRequest> {
        // if dst token not specified, use source token
        let src_token = self.src_token?;
        let dst_token = self.dst_token.unwrap_or(src_token);

        Some(OnlySwapsRequest {
            amount: self.amount?,
            fee: self.solver_fee?,
            recipient: self.recipient?,
            route: SwapRouting {
                src_token,
                dst_token,
                src_chain: self.src_chain?,
                dst_chain: self.dst_chain?,
            },
        })
    }
}
