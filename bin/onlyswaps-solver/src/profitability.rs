//! Ensures the profitability of a trade for a solver.

mod market_data;

use crate::model::Trade;
use crate::price_feed::TokenPriceFeed;
use crate::profitability::market_data::{MarketData, fetch_trade_market_data};
use alloy::consensus::constants::ETH_TO_WEI;
use alloy::primitives::U256;
use alloy::providers::Provider;
use anyhow::Context;
use bigdecimal::num_bigint::BigUint;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

const NATIVE_EVM_TOKEN_UNIT: f64 = 1e18;

/// A maximum bound to prevent too much precision loss
const MAX_REASONABLE_GAS_COST_WEI: u128 = ETH_TO_WEI;

/// A maximum bound to prevent too much precision loss
const MAX_REASONABLE_TOKEN_PRICE_USD: f64 = 10_000_000.0;

/// Ensures that a trade is profitable by comparing the incurred costs in equivalent currencies using
/// a price feed.
#[tracing::instrument(level = "warn", skip_all, fields(trade.src_chain_id, trade.request_id))]
pub async fn profitability_breaker(
    trade: &Trade,
    gas_estimate: u64,
    price_feed: &impl TokenPriceFeed,
    provider: &impl Provider,
) -> anyhow::Result<bool> {
    let gas_cost_upper_bound = estimate_gas_cost(provider, gas_estimate).await?;
    let market_data = fetch_trade_market_data(
        trade.src_chain_id,
        trade.token_in_addr,
        trade.dest_chain_id,
        price_feed,
        MAX_REASONABLE_TOKEN_PRICE_USD,
    )
    .await?;

    let fulfillment =
        FulfillmentData::evaluate(gas_cost_upper_bound, trade.solver_fee, &market_data)?;

    if fulfillment.is_profitable() {
        Ok(true)
    } else {
        tracing::warn!(
            fulfillment_cost = fulfillment.cost,
            fulfillment_reward = fulfillment.reward,
            "Trade not profitable"
        );
        Ok(false)
    }
}

/// Estimate the current gas costs from the provider.
async fn estimate_gas_cost(provider: &impl Provider, gas_estimate: u64) -> anyhow::Result<u128> {
    let gas_cost = match provider.estimate_eip1559_fees().await {
        Ok(fees) => u128::from(gas_estimate).checked_mul(fees.max_fee_per_gas),
        Err(e) => {
            tracing::warn!(
                error = ?e,
                "Failed to estimate eip1559 fees, falling back to legacy estimation"
            );
            let gas_price = provider
                .get_gas_price()
                .await
                .context("failed to get gas price")?;
            u128::from(gas_estimate).checked_mul(gas_price)
        }
    }
    .context("overflow while computing gas cost upper bound")?;

    anyhow::ensure!(
        gas_cost < MAX_REASONABLE_GAS_COST_WEI,
        "gas cost unreasonably high: {} wei (max: {} wei)",
        gas_cost,
        MAX_REASONABLE_GAS_COST_WEI
    );
    Ok(gas_cost)
}

struct FulfillmentData {
    cost: f64,
    reward: f64,
}

impl FulfillmentData {
    fn evaluate(
        gas_cost_upper_bound: u128,
        solver_fee: U256,
        market_data: &MarketData,
    ) -> anyhow::Result<FulfillmentData> {
        // Precision analysis:
        //  - `gas_cost_upper_bound` (u128) = gas_limit (u64) * gas_cost (u128)
        //  - native token decimals = 1e18 ~= 2^60
        //  - `native_value_dst` (f64) = native token value in USD
        //
        // Experimentally, gas limit ~2^19, gas cost ~2^29, so gas_cost_upper_bound ~2^48.
        // This fits in f64 without loss. The most expensive native coin (BTC) is ~$100k ~2^17 USD.
        // Multiplying: 2^48 × 2^17 = 2^65, well within f64 range.
        // Dividing by 1e18 ~2^60 gives 2^5 (~$32) with minimal relative loss (~2^-53).
        let cost =
            (gas_cost_upper_bound as f64) * market_data.native_value_dst / NATIVE_EVM_TOKEN_UNIT;

        // Since the solver fee can be arbitrarily large, and often larger than 2**64, use an arbitrary precision lib
        let solver_fee =
            BigDecimal::from_biguint(BigUint::from_bytes_be(&solver_fee.to_be_bytes::<32>()), 0);

        let reward = solver_fee
            * BigDecimal::from_f64(market_data.token_value_src)
                .context("token value did not fit in f64")?
            / BigDecimal::from_u16(10)
                .unwrap()
                .powi(market_data.token_decimals_src as i64);

        // Let's assume that the reward fits a f64, if not, we're rich
        let reward = reward.to_f64().context("reward did not fit in a f64")?;

        anyhow::ensure!(cost.is_finite(), "computed fulfillment cost is not finite");
        anyhow::ensure!(
            reward.is_finite(),
            "computed fulfillment reward is not finite"
        );

        Ok(FulfillmentData { cost, reward })
    }

    fn is_profitable(&self) -> bool {
        self.cost < self.reward
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_real_input() {
        // Inputs from a real trade on avalanche: https://snowtrace.io/tx/0xdce85e94ffa5fe3a48874139cb1c4f2734536b607c239edb1788d8e68cbc6933?chainid=43114
        // Params on base:
        // [ swapRequestParameters(bytes32) method Response ]
        // sender   address :  0xafe394B3198AB80C69d280ef4f5905A0647e0e97
        // recipient   address :  0xafe394B3198AB80C69d280ef4f5905A0647e0e97
        // tokenIn   address :  0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2
        // tokenOut   address :  0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7
        // amountOut   uint256 :  109725
        // srcChainId   uint256 :  8453
        // dstChainId   uint256 :  43114
        // verificationFee   uint256 :  275
        // solverFee   uint256 :  40_000
        // nonce   uint256 :  2
        // executed   bool :  true
        // requestedAt   uint256 :  1763674791

        let gas_cost_upper_bound = 402_724u128 * 314_900_000u128; // 0.3149 nAVAX
        let solver_fee = U256::from(40_000u128);
        let market_data = MarketData {
            native_value_dst: 13.82845f64,
            token_value_src: 0.997642,
            token_decimals_src: 6,
        };
        let data = FulfillmentData::evaluate(gas_cost_upper_bound, solver_fee, &market_data)
            .expect("to evaluate successfully");
        assert!(data.is_profitable(), "trade should be profitable");
    }

    #[test]
    fn evaluate_unprofitable() {
        // Tweaked inputs from a real trade on avalanche: https://snowtrace.io/tx/0xdce85e94ffa5fe3a48874139cb1c4f2734536b607c239edb1788d8e68cbc6933?chainid=43114
        // Params on base:
        // [ swapRequestParameters(bytes32) method Response ]
        // sender   address :  0xafe394B3198AB80C69d280ef4f5905A0647e0e97
        // recipient   address :  0xafe394B3198AB80C69d280ef4f5905A0647e0e97
        // tokenIn   address :  0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2
        // tokenOut   address :  0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7
        // amountOut   uint256 :  109725
        // srcChainId   uint256 :  8453
        // dstChainId   uint256 :  43114
        // verificationFee   uint256 :  275
        // solverFee   uint256 :  40_000
        // nonce   uint256 :  2
        // executed   bool :  true
        // requestedAt   uint256 :  1763674791

        let gas_cost_upper_bound = 402_724u128 * 314_900_000u128; // 0.3149 nAVAX
        let solver_fee = U256::from(40_000u128);
        let market_data = MarketData {
            native_value_dst: 13.82845f64 * 100f64, // we now say that the price of AVAX is suddenly 100x more expensive
            token_value_src: 0.997642,
            token_decimals_src: 6,
        };
        let data = FulfillmentData::evaluate(gas_cost_upper_bound, solver_fee, &market_data)
            .expect("to evaluate successfully");
        assert!(!data.is_profitable(), "trade should not be profitable");
    }
}
