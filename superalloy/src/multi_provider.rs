//! Module that allows receiving events from multiple RPC providers concurrently.

use crate::provider::{RecommendedProvider, WatchError, watch_block_numbers};
use alloy::network::Ethereum;
use alloy::providers::{Provider, RootProvider};
use futures_util::future::join_all;
use futures_util::{Stream, StreamExt, stream};
use std::time::Duration;

/// Helper to obtain streams from multiple RPC providers.
pub struct MultiProvider<K> {
    providers: Vec<ProviderWithKey<K>>,
}

impl<K: Clone> Clone for MultiProvider<K> {
    fn clone(&self) -> Self {
        Self {
            providers: self.providers.clone(),
        }
    }
}

struct ProviderWithKey<K>(K, RecommendedProvider);

impl<K: Clone> Clone for ProviderWithKey<K> {
    fn clone(&self) -> Self {
        ProviderWithKey(self.0.clone(), self.1.clone())
    }
}

impl<K> MultiProvider<K> {
    pub fn empty() -> MultiProvider<K> {
        MultiProvider { providers: vec![] }
    }

    pub fn add(&mut self, key: K, provider: RecommendedProvider) {
        self.providers.push(ProviderWithKey(key, provider));
    }
}

impl<K> Provider for MultiProvider<K>
where
    K: Send + Sync,
{
    fn root(&self) -> &RootProvider<Ethereum> {
        self.providers[0].1.root()
    }
}

impl<K> FromIterator<(K, RecommendedProvider)> for MultiProvider<K> {
    fn from_iter<T: IntoIterator<Item = (K, RecommendedProvider)>>(iter: T) -> MultiProvider<K> {
        Self {
            providers: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl<K> MultiProvider<K>
where
    K: Copy,
{
    /// Watch for new block numbers.
    /// Returns a stream only if all providers were successful.
    pub async fn watch_block_numbers(
        &mut self,
        duration: Option<Duration>,
    ) -> Result<impl Stream<Item = (K, Result<u64, WatchError>)> + Unpin + Sized, Vec<WatchError>>
    {
        // Join all block number streams into a joint stream.
        let streams: Vec<_> = join_all(self.providers.iter().map(
            |ProviderWithKey(key, provider)| async {
                let res_stream = watch_block_numbers(provider, duration).await;
                res_stream.map(|stream| stream.map(|block_number| (*key, block_number)))
            },
        ))
        .await;

        // Check if there were any errors, return a list of errored providers if so
        let (oks, errs): (Vec<_>, Vec<_>) = streams.into_iter().partition(Result::is_ok);
        if !errs.is_empty() {
            Err(errs
                .into_iter()
                .map(|res| res.err().expect("must be err"))
                .collect::<Vec<_>>())?
        }

        Ok(stream::SelectAll::from_iter(
            oks.into_iter().map(Result::unwrap),
        ))
    }
}

impl<K> From<(K, RecommendedProvider)> for ProviderWithKey<K> {
    fn from(provider: (K, RecommendedProvider)) -> ProviderWithKey<K> {
        Self(provider.0, provider.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::create_provider_with_retry;
    use crate::retry::RetryStrategy;

    #[tokio::test]
    async fn test_watch_block_numbers() {
        let rpc_providers = [
            "wss://mainnet.gateway.tenderly.co",
            "https://rpc.poolz.finance/eth",
        ];

        let mut multi = MultiProvider::empty();
        multi.add(
            rpc_providers[0],
            create_provider_with_retry(rpc_providers[0].parse().unwrap(), RetryStrategy::None)
                .await
                .unwrap(),
        );
        multi.add(
            rpc_providers[1],
            create_provider_with_retry(rpc_providers[1].parse().unwrap(), RetryStrategy::None)
                .await
                .unwrap(),
        );

        let mut stream = multi
            .watch_block_numbers(Some(Duration::from_secs(2)))
            .await
            .unwrap();
        let (rpc1, block_number1) = stream.next().await.unwrap();
        let (rpc2, block_number2) = stream.next().await.unwrap();
        // We should obtain the same block number from two different rpc providers
        assert_eq!(block_number1.unwrap(), block_number2.unwrap());
        assert_ne!(rpc1, rpc2);
    }
}
