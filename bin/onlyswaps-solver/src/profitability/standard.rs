//! A standard [`ProfitabilityEstimator`] that ensures that trades are always profitable by relying
//! on a [`TokenPriceFeed`] to comparing the fulfillment vs reward cost.

use crate::model::Trade;
use crate::price_feed::TokenPriceFeed;
use crate::profitability::ProfitabilityEstimator;
use crate::profitability::market_data::{MarketData, fetch_trade_market_data};
use alloy::primitives::U256;
use anyhow::Context;
use bigdecimal::num_bigint::BigUint;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

const NATIVE_EVM_TOKEN_UNIT: f64 = 1e18;

/// A maximum bound to prevent too much precision loss
const MAX_REASONABLE_TOKEN_PRICE_USD: f64 = 10_000_000.0;

pub struct StdProfitabilityEstimator<PF> {
    price_feed: PF,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct StdProfitabilityEstimatorError(#[from] anyhow::Error);

impl<PF> StdProfitabilityEstimator<PF> {
    pub fn new(price_feed: PF) -> Self {
        Self { price_feed }
    }
}

impl<PF: TokenPriceFeed + Sync> ProfitabilityEstimator for StdProfitabilityEstimator<PF> {
    type Error = StdProfitabilityEstimatorError;

    async fn is_profitable(
        &self,
        trade: &Trade,
        gas_estimate: u64,
        gas_cost: u128,
    ) -> Result<bool, Self::Error> {
        Ok(profitability_breaker(trade, gas_estimate, gas_cost, &self.price_feed).await?)
    }
}

/// Ensures that a trade is profitable by comparing the incurred costs in equivalent currencies using
/// a price feed.
#[tracing::instrument(level = "warn", skip_all, fields(trade.src_chain_id, trade.request_id))]
async fn profitability_breaker(
    trade: &Trade,
    gas_estimate: u64,
    gas_cost: u128,
    price_feed: &impl TokenPriceFeed,
) -> anyhow::Result<bool> {
    let gas_cost_upper_bound = u128::from(gas_estimate)
        .checked_mul(gas_cost)
        .context("gas cost overflow")?;
    let market_data = fetch_trade_market_data(
        trade.src_chain_id,
        trade.token_in_addr,
        trade.token_out_addr,
        trade.dest_chain_id,
        price_feed,
        MAX_REASONABLE_TOKEN_PRICE_USD,
    )
    .await?;

    let fulfillment = FulfillmentData::evaluate(
        gas_cost_upper_bound,
        trade.solver_refund_amount,
        trade.amount_out,
        &market_data,
    )?;

    if fulfillment.is_profitable() {
        tracing::debug!(
            fulfillment_cost = fulfillment.cost,
            fulfillment_reward = fulfillment.reward,
            "Trade is profitable"
        );
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

struct FulfillmentData {
    cost: f64,
    reward: f64,
}

impl FulfillmentData {
    fn evaluate(
        gas_cost_upper_bound: u128,
        solver_refund_amount: U256,
        amount_out: U256,
        market_data: &MarketData,
    ) -> anyhow::Result<FulfillmentData> {
        // Use BigDecimals for arbitrary precision instead of f64

        // Compute the tx fulfillment cost
        let tx_cost = BigDecimal::from(gas_cost_upper_bound)
            * BigDecimal::from_f64(market_data.native_value_dst)
                .context("native value did not fit in f64")?
            / NATIVE_EVM_TOKEN_UNIT;

        // Compute the cost of sending amount_out tokens
        let amount_out_cost = u256_to_bigdecimal(&amount_out)
            * BigDecimal::from_f64(market_data.token_value_dst)
                .context("token value did not fit in f64")?
            / BigDecimal::from(10).powi(market_data.token_decimals_dst as i64);

        let reward = u256_to_bigdecimal(&solver_refund_amount)
            * BigDecimal::from_f64(market_data.token_value_src)
                .context("token value did not fit in f64")?
            / BigDecimal::from(10).powi(market_data.token_decimals_src as i64);

        // Let's assume that both the USD cost & reward fit in f64s. Highly unlikely to have a fulfillment
        // with a value greater than a f64
        let cost = (tx_cost + amount_out_cost)
            .to_f64()
            .context("cost did not fit in a f64")?;
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

fn u256_to_bigdecimal(v: &U256) -> BigDecimal {
    // no clean ruint to BigDecimal conversion
    BigDecimal::from_biguint(BigUint::from_bytes_be(&v.to_be_bytes::<32>()), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{ChainId, address};
    use std::collections::HashMap;

    struct FakePriceFeed {
        native_values: HashMap<ChainId, f64>,
        token_decimals: HashMap<(ChainId, String), u8>,
        token_prices: HashMap<(ChainId, String), f64>,
    }

    impl FakePriceFeed {
        fn from_trade_data(trade: &Trade, market_data: &MarketData) -> Self {
            let token_in_addr = trade.token_in_addr.to_string();
            let token_out_addr = trade.token_out_addr.to_string();

            let src_chain = trade.src_chain_id.try_into().unwrap();
            let dst_chain = trade.dest_chain_id.try_into().unwrap();

            FakePriceFeed {
                token_prices: HashMap::from_iter([
                    (
                        (src_chain, token_in_addr.clone()),
                        market_data.token_value_src,
                    ),
                    (
                        (dst_chain, token_out_addr.clone()),
                        market_data.token_value_dst,
                    ),
                ]),
                token_decimals: HashMap::from_iter([
                    ((src_chain, token_in_addr), market_data.token_decimals_src),
                    ((dst_chain, token_out_addr), market_data.token_decimals_dst),
                ]),
                native_values: HashMap::from_iter([(dst_chain, market_data.native_value_dst)]),
            }
        }
    }

    #[derive(thiserror::Error, Debug)]
    #[error("fake price feed error")]
    struct FakeError;

    impl TokenPriceFeed for FakePriceFeed {
        type Error = FakeError;

        fn native_value(
            &self,
            chain_id: ChainId,
        ) -> impl Future<Output = Result<f64, Self::Error>> + Send {
            std::future::ready(self.native_values.get(&chain_id).copied().ok_or(FakeError))
        }

        fn token_decimals(
            &self,
            chain_id: ChainId,
            token_address: String,
        ) -> impl Future<Output = Result<u8, Self::Error>> + Send {
            std::future::ready(
                self.token_decimals
                    .get(&(chain_id, token_address))
                    .copied()
                    .ok_or(FakeError),
            )
        }

        fn token_price_batched(
            &self,
            chain_id: ChainId,
            token_addresses: impl IntoIterator<Item = String, IntoIter: Send> + Send,
        ) -> impl Future<Output = Result<Vec<f64>, Self::Error>> + Send {
            std::future::ready(
                token_addresses
                    .into_iter()
                    .map(|token| {
                        self.token_prices
                            .get(&(chain_id, token))
                            .copied()
                            .ok_or(FakeError)
                    })
                    .collect(),
            )
        }
    }

    fn real_trade() -> (Trade, u64, u128, MarketData) {
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

        let trade = Trade {
            request_id: "0xc19a45e6e47db297aa1ef996c1e29b74cf3c5a4e11035eae115f00eeb1a81a6c"
                .parse()
                .unwrap(),
            token_in_addr: address!("0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2"),
            token_out_addr: address!("0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7"),
            src_chain_id: U256::from(8453),
            dest_chain_id: U256::from(43114),
            sender_addr: address!("0xafe394B3198AB80C69d280ef4f5905A0647e0e97"),
            recipient_addr: address!("0xafe394B3198AB80C69d280ef4f5905A0647e0e97"),
            amount_in: U256::from(110_000),
            amount_out: U256::from(109_725),
            solver_fee: U256::from(40_000),
            solver_refund_amount: U256::from(149_725),
            nonce: U256::from(2),
            pre_hooks: vec![],
            post_hooks: vec![],
        };
        let gas_estimate = 402_724;
        let gas_price = 314_900_000; // 0.3149 nAVAX

        // Market data at the time of the trade
        let market_data = MarketData {
            native_value_dst: 13.82845f64,
            token_value_src: 0.997642,
            token_decimals_src: 6,
            token_value_dst: 0.997642,
            token_decimals_dst: 6,
        };

        (trade, gas_estimate, gas_price, market_data)
    }

    #[test]
    fn evaluate_real_input() {
        let (trade, gas_estimate, gas_price, market_data) = real_trade();
        let gas_cost_upper_bound = gas_estimate as u128 * gas_price;

        let data = FulfillmentData::evaluate(
            gas_cost_upper_bound,
            trade.solver_refund_amount,
            trade.amount_out,
            &market_data,
        )
        .expect("to evaluate successfully");
        assert!(data.is_profitable(), "trade should be profitable");
    }

    #[test]
    fn evaluate_unprofitable() {
        let (trade, gas_estimate, gas_price, mut market_data) = real_trade();
        let gas_cost_upper_bound = gas_estimate as u128 * gas_price;
        market_data.native_value_dst *= 100f64; // we now say that the price of AVAX is suddenly 100x more expensive

        let data = FulfillmentData::evaluate(
            gas_cost_upper_bound,
            trade.solver_refund_amount,
            trade.amount_out,
            &market_data,
        )
        .expect("to evaluate successfully");
        assert!(!data.is_profitable(), "trade should not be profitable");
    }

    #[tokio::test]
    async fn evaluate_real_input_with_price_feed() {
        let (trade, gas_estimate, gas_price, market_data) = real_trade();

        let price_feed = FakePriceFeed::from_trade_data(&trade, &market_data);
        let estimator = StdProfitabilityEstimator::new(price_feed);
        let profitable = estimator
            .is_profitable(&trade, gas_estimate, gas_price)
            .await
            .expect("to evaluate with Ok");
        assert!(profitable, "trade should be profitable");
    }

    #[tokio::test]
    async fn evaluate_unprofitable_high_native_with_price_feed() {
        let (trade, gas_estimate, gas_price, mut market_data) = real_trade();
        market_data.native_value_dst *= 100f64; // we now say that the price of AVAX is suddenly 100x more expensive

        let price_feed = FakePriceFeed::from_trade_data(&trade, &market_data);
        let estimator = StdProfitabilityEstimator::new(price_feed);
        let profitable = estimator
            .is_profitable(&trade, gas_estimate, gas_price)
            .await
            .expect("to evaluate with Ok");
        assert!(!profitable, "trade should be unprofitable");
    }

    #[tokio::test]
    async fn evaluate_unprofitable_high_out_with_price_feed() {
        let (trade, gas_estimate, gas_price, mut market_data) = real_trade();
        market_data.token_value_dst *= 2f64; // we now say that the price of the destination token is suddenly 2x more expensive

        let price_feed = FakePriceFeed::from_trade_data(&trade, &market_data);
        let estimator = StdProfitabilityEstimator::new(price_feed);
        let profitable = estimator
            .is_profitable(&trade, gas_estimate, gas_price)
            .await
            .expect("to evaluate with Ok");
        assert!(!profitable, "trade should be unprofitable");
    }
}
