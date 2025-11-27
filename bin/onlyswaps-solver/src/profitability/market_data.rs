//! Fetch market data.

use crate::price_feed::TokenPriceFeed;
use alloy::primitives::{Address, U256};
use anyhow::Context;
use futures::FutureExt;

#[derive(Copy, Clone, Debug)]
pub(crate) struct MarketData {
    pub native_value_dst: f64,
    pub token_value_src: f64,
    pub token_decimals_src: u8,
    pub token_value_dst: f64,
    pub token_decimals_dst: u8,
}

impl MarketData {
    fn new_validated(
        native_value_dst: f64,
        token_value_src: f64,
        token_decimals_src: u8,
        token_value_dst: f64,
        token_decimals_dst: u8,
        max_price: f64,
    ) -> anyhow::Result<Self> {
        validate_price(native_value_dst, max_price)
            .context("failed to validate native_value_dst")?;
        validate_price(token_value_src, max_price).context("failed to validate token_value_src")?;
        validate_price(token_value_dst, max_price).context("failed to validate token_value_dst")?;
        Ok(Self {
            native_value_dst,
            token_value_src,
            token_decimals_src,
            token_value_dst,
            token_decimals_dst,
        })
    }
}

/// Fetch the current market price of the destination's native token, and the source token.
pub(crate) async fn fetch_trade_market_data(
    src_chain_id: U256,
    token_in_addr: Address,
    token_out_addr: Address,
    dest_chain_id: U256,
    price_feed: &impl TokenPriceFeed,
    max_price: f64,
) -> anyhow::Result<MarketData> {
    let dst_chain_id: u64 = dest_chain_id
        .try_into()
        .context("dest chain id does not fit in u64")?;
    let src_chain_id: u64 = src_chain_id
        .try_into()
        .context("src chain id does not fit in u64")?;

    let native_value_dst = price_feed
        .native_value(dst_chain_id)
        .map(|out| out.context("failed to fetch destination native value"));

    let token_value_src = price_feed
        .token_value(src_chain_id, token_in_addr.to_string())
        .map(|out| out.context("failed to fetch source token value"));

    let token_decimals_src = price_feed
        .token_decimals(src_chain_id, token_in_addr.to_string())
        .map(|out| out.context("failed to fetch source token decimals"));

    let token_value_dst = price_feed
        .token_value(dst_chain_id, token_out_addr.to_string())
        .map(|out| out.context("failed to fetch destination token value"));

    let token_decimals_dst = price_feed
        .token_decimals(dst_chain_id, token_out_addr.to_string())
        .map(|out| out.context("failed to fetch destination token decimals"));

    let (
        native_value_dst,
        token_value_src,
        token_decimals_src,
        token_value_dst,
        token_decimals_dst,
    ) = futures::try_join!(
        native_value_dst,
        token_value_src,
        token_decimals_src,
        token_value_dst,
        token_decimals_dst
    )?;

    MarketData::new_validated(
        native_value_dst,
        token_value_src,
        token_decimals_src,
        token_value_dst,
        token_decimals_dst,
        max_price,
    )
}

fn validate_price(price: f64, max_price: f64) -> anyhow::Result<()> {
    anyhow::ensure!(
        price.is_finite() && price >= 0.0,
        "invalid token price: {}",
        price
    );

    anyhow::ensure!(
        price < max_price,
        "token price unreasonably high: ${} (max: ${})",
        price,
        max_price
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::ChainId;

    #[derive(thiserror::Error, Debug)]
    #[error("error")]
    struct Err;
    struct FixedPriceFeed(f64, u8);

    impl TokenPriceFeed for FixedPriceFeed {
        type Error = Err;

        async fn native_value(&self, _chain_id: ChainId) -> Result<f64, Self::Error> {
            Ok(self.0)
        }

        async fn token_decimals(
            &self,
            _chain_id: ChainId,
            _token_address: String,
        ) -> Result<u8, Self::Error> {
            Ok(self.1)
        }

        async fn token_price_batched(
            &self,
            _chain_id: ChainId,
            token_addresses: impl IntoIterator<Item = String, IntoIter: Send> + Send,
        ) -> Result<Vec<f64>, Self::Error> {
            Ok(token_addresses.into_iter().map(|_| self.0).collect())
        }
    }

    #[tokio::test]
    async fn should_not_validate_bad_market_data() {
        fetch_trade_market_data(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            &FixedPriceFeed(f64::MAX, 6),
            1_000_000.,
        )
        .await
        .expect_err("should not validate");
    }

    #[tokio::test]
    async fn should_validate_market_data() {
        fetch_trade_market_data(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            &FixedPriceFeed(1.0, 6),
            1_000_000.,
        )
        .await
        .expect("should validate");
    }
}
