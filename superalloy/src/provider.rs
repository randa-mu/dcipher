use crate::retry::{RetryStrategy, with_retry};
use alloy::consensus::BlockHeader;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::providers::{Identity, Provider, ProviderBuilder, RootProvider, WsConnect};
use alloy::transports::http::reqwest;
use alloy::transports::{RpcError, TransportError, TransportErrorKind};
use futures::Stream;
use futures_util::{StreamExt, stream};
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateProviderError {
    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error("unknown transport scheme specified in rpc url")]
    UnsupportedScheme,
}

pub type RecommendedProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

/// Creates a http / websocket provider based on the rpc url provided.
pub async fn create_provider_with_retry(
    rpc_url: reqwest::Url,
    retry_strategy: RetryStrategy,
) -> Result<RecommendedProvider, CreateProviderError> {
    let build_provider = || ProviderBuilder::default().with_recommended_fillers();

    match rpc_url.scheme() {
        "http" | "https" => {
            let provider_fn = async || Ok(build_provider().connect_http(rpc_url.clone()));
            with_retry(provider_fn, retry_strategy).await
        }
        "ws" | "wss" => {
            let provider_fn = async || {
                Ok({
                    let ws = WsConnect::new(rpc_url.clone());
                    build_provider().connect_ws(ws).await?
                })
            };
            with_retry(provider_fn, retry_strategy).await
        }
        _ => Err(CreateProviderError::UnsupportedScheme)?,
    }
}

pub type BlockNumberStream = Pin<Box<dyn Stream<Item = Result<u64, WatchError>>>>;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error(transparent)]
    Rpc(#[from] RpcError<TransportErrorKind>),

    #[error("polling interval not specified while required for http polling")]
    MissingPollingInterval,
}

/// We need to box the output since we return divergent stream implementations
pub async fn watch_block_numbers(
    provider: &RecommendedProvider,
    polling_interval: Option<Duration>,
) -> Result<BlockNumberStream, WatchError> {
    // First, try to create a stream using subscribe_blocks. This fails if we are using an http provider
    let subscribe_res = provider.subscribe_blocks().await;
    match subscribe_res {
        Ok(sub) => Ok(Box::pin(sub.into_stream().map(|h| Ok(h.number())))),
        Err(RpcError::Transport(TransportErrorKind::PubsubUnavailable)) => {
            let Some(polling_interval) = polling_interval else {
                Err(WatchError::MissingPollingInterval)?
            };

            Ok(Box::pin(
                poll_block_numbers(provider.clone(), polling_interval).await,
            ))
        }
        Err(e) => Err(e)?,
    }
}

async fn poll_block_numbers(
    provider: RecommendedProvider,
    polling_interval: Duration,
) -> impl Stream<Item = Result<u64, WatchError>> {
    let last_block = Arc::new(AtomicU64::new(0));
    stream::repeat_with(move || {
        let last_block = last_block.clone();
        let provider = provider.clone();

        async move {
            // Wait for the specified interval
            tokio::time::sleep(polling_interval).await;

            // Query the latest block number
            match provider.get_block_number().await {
                Ok(block_number) => {
                    let last_block_u64 = last_block.load(Ordering::Relaxed);
                    if last_block_u64 == 0 {
                        // First block => store it _without advertising_ to match the behaviour of PubSub RPCs.
                        last_block.store(block_number, Ordering::Relaxed);
                        None
                    } else if block_number > last_block_u64 {
                        // New block => store it and advertise
                        last_block.store(block_number, Ordering::Relaxed);
                        Some(Ok(block_number))
                    } else {
                        // Same block => do not advertise it
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching block number: {}", e);
                    Some(Err(WatchError::Rpc(e)))
                }
            }
        }
    })
    .filter_map(async move |f| f.await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::Provider;

    const FILECOIN_CALIBNET_URL: &str = "https://api.calibration.node.glif.io/rpc/v1";
    const ETHEREUM_MAINNET_WS_URL: &str = "wss://mainnet.gateway.tenderly.co";

    #[tokio::test]
    async fn create_provider() {
        let retry_strategy = RetryStrategy::ConstantBackoff {
            retries: 5,
            backoff: Duration::from_millis(500),
        };

        let filecoin_calibnet_url = FILECOIN_CALIBNET_URL.parse().unwrap();
        let filecoin_calibnet_chainid = 314159;
        let filecoin_calibnet_provider =
            create_provider_with_retry(filecoin_calibnet_url, retry_strategy)
                .await
                .unwrap();
        assert_eq!(
            filecoin_calibnet_provider.get_chain_id().await.unwrap(),
            filecoin_calibnet_chainid
        );

        let ethereum_mainnet_url = ETHEREUM_MAINNET_WS_URL.parse().unwrap();
        let ethereum_mainnet_chainid = 1;
        let ethereum_mainnet_provider =
            create_provider_with_retry(ethereum_mainnet_url, retry_strategy)
                .await
                .unwrap();
        assert_eq!(
            ethereum_mainnet_provider.get_chain_id().await.unwrap(),
            ethereum_mainnet_chainid
        );

        let invalid_rpc_url = "ftp://0xrpc.io/eth".parse().unwrap();
        let unsupported_provider_scheme_res =
            create_provider_with_retry(invalid_rpc_url, retry_strategy).await;
        assert!(matches!(
            unsupported_provider_scheme_res,
            Err(CreateProviderError::UnsupportedScheme)
        ));
    }

    #[tokio::test]
    async fn watch_block_numbers_wss() {
        let retry_strategy = RetryStrategy::ConstantBackoff {
            retries: 5,
            backoff: Duration::from_millis(500),
        };

        let ethereum_mainnet_url = ETHEREUM_MAINNET_WS_URL.parse().unwrap();
        let ethereum_mainnet_provider =
            create_provider_with_retry(ethereum_mainnet_url, retry_strategy)
                .await
                .unwrap();

        let mut block_number_stream = watch_block_numbers(&ethereum_mainnet_provider, None)
            .await
            .unwrap();
        let curr_block = block_number_stream.next().await.unwrap().unwrap();
        let next_block = block_number_stream.next().await.unwrap().unwrap();
        assert_eq!(curr_block + 1, next_block);
    }

    #[tokio::test]
    async fn watch_block_numbers_http() {
        let retry_strategy = RetryStrategy::ConstantBackoff {
            retries: 5,
            backoff: Duration::from_millis(500),
        };

        let filecoin_calibnet_url = FILECOIN_CALIBNET_URL.parse().unwrap();
        let filecoin_calibnet_provider =
            create_provider_with_retry(filecoin_calibnet_url, retry_strategy)
                .await
                .unwrap();

        let mut block_number_stream =
            watch_block_numbers(&filecoin_calibnet_provider, Some(Duration::from_secs(2)))
                .await
                .unwrap();
        let curr_block = block_number_stream.next().await.unwrap().unwrap();
        let next_block = block_number_stream.next().await.unwrap().unwrap();
        assert_eq!(curr_block + 1, next_block);
    }
}
