pub mod coingecko;

use alloy::primitives::ChainId;
use futures::FutureExt;

/// Allows to fetch the USD value of various tokens on various chains.
pub trait TokenPriceFeed {
    type Error: std::error::Error + Send + Sync + 'static;

    fn native_value(
        &self,
        chain_id: ChainId,
    ) -> impl Future<Output = Result<f64, Self::Error>> + Send;

    fn token_decimals(
        &self,
        chain_id: ChainId,
        token_address: String,
    ) -> impl Future<Output = Result<u8, Self::Error>> + Send;

    fn token_value(
        &self,
        chain_id: ChainId,
        token_address: String,
    ) -> impl Future<Output = Result<f64, Self::Error>> + Send {
        self.token_price_batched(chain_id, std::iter::once(token_address))
            .map(|out| Ok(out?[0]))
    }

    fn token_price_batched(
        &self,
        chain_id: ChainId,
        token_addresses: impl IntoIterator<Item = String, IntoIter: Send> + Send,
    ) -> impl Future<Output = Result<Vec<f64>, Self::Error>> + Send;
}
