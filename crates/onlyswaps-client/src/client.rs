//! The onlyswaps client which can be used to swap tokens from one chain to another

pub mod erc20;
pub mod routing;

use crate::client::erc20::IERC20;
use crate::client::routing::SwapRouting;
use crate::config::OnlySwapsClientConfig;
use crate::config::chain::ChainConfig;
use crate::config::token::TokenTag;
use alloy::network::Ethereum;
use alloy::primitives::{Address, FixedBytes, LogData, TxHash, U256};
use alloy::providers::{DynProvider, PendingTransactionError, Provider};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use futures_util::StreamExt;
use generated::onlyswaps::router::Router;
use generated::onlyswaps::router::Router::RouterInstance;
use std::marker::PhantomData;

pub type OnlySwapsRequestId = FixedBytes<32>;

/// An onlyswaps client to execute swaps from and to any chain.
#[derive(Clone)]
pub struct OnlySwapsClient<N = Ethereum> {
    /// onlyswaps configuration
    config: OnlySwapsClientConfig,

    _n: PhantomData<fn(N)>,
}

/// The status of an onlyswaps request
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum OnlySwapsStatus {
    /// the swap has not yet been fulfilled by a solver
    Pending,

    /// the swap has been fulfilled by a solver, but not yet verified by the dcipher network
    Completed,

    /// the swap has been fulfilled by a solver, and verified by the dcipher network
    Verified,
}

#[derive(thiserror::Error, Debug)]
pub enum OnlySwapsClientError {
    #[error("no provider for chain id {0}")]
    MissingProvider(u64),

    #[error("no config for chain id {0}")]
    UnsupportedChain(u64),

    #[error("token ({0}) not supported on chain id ({1:?})")]
    UnsupportedToken(u64, TokenTag),

    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),

    #[error("pending transaction failed")]
    PendingTransaction(#[from] PendingTransactionError),

    #[error("failed to sign tx")]
    SignTx(#[from] alloy::signers::Error),

    #[error("rpc error: {1}")]
    Rpc(
        #[source] alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
        &'static str,
    ),

    #[error("swap failed: event not in logs")]
    SwapFailedNotInLogs,

    #[error("swap request not found")]
    SwapRequestNotFound,
}

impl OnlySwapsClient {
    pub fn new(config: OnlySwapsClientConfig) -> Self {
        Self {
            config,
            _n: PhantomData,
        }
    }
}

impl OnlySwapsClient {
    /// Approve the router contract to spend tokens for upcoming swaps
    /// The amount must include the fees.
    pub async fn approve_spending(
        &self,
        chain_id: u64,
        token: TokenTag,
        amount: U256,
    ) -> Result<TxHash, OnlySwapsClientError> {
        let provider = self
            .config
            .get_ethereum_provider(chain_id)
            .ok_or(OnlySwapsClientError::MissingProvider(chain_id))?;

        let chain_config = self
            .config
            .get_chain_config(chain_id)
            .ok_or(OnlySwapsClientError::UnsupportedChain(chain_id))?;

        let token_addr = *chain_config
            .supported_tokens
            .get(&token)
            .ok_or(OnlySwapsClientError::UnsupportedToken(chain_id, token))?;

        approve_spending(chain_config, provider, token_addr, amount).await
    }

    /// Create a new swap, sending tokens to a recipient
    pub async fn swap(
        &self,
        recipient: Address,
        amount: U256,
        fee: U256,
        routing: SwapRouting,
    ) -> Result<OnlySwapsRequestId, OnlySwapsClientError> {
        let (provider, src_chain_config, src_token_addr, dst_token_addr) =
            self.swap_params(&routing)?;
        swap(
            src_chain_config,
            provider,
            recipient,
            amount,
            fee,
            routing.dst_chain,
            src_token_addr,
            dst_token_addr,
        )
        .await?
    }

    fn swap_params(
        &self,
        routing: &SwapRouting,
    ) -> Result<(&DynProvider, &ChainConfig, Address, Address), OnlySwapsClientError> {
        let provider = self
            .config
            .get_ethereum_provider(routing.src_chain)
            .ok_or(OnlySwapsClientError::MissingProvider(routing.src_chain))?;

        let src_chain_config = self
            .config
            .get_chain_config(routing.src_chain)
            .ok_or(OnlySwapsClientError::UnsupportedChain(routing.src_chain))?;

        let src_token_addr = *src_chain_config
            .supported_tokens
            .get(&routing.src_token)
            .ok_or(OnlySwapsClientError::UnsupportedToken(
                routing.src_chain,
                routing.src_token,
            ))?;

        let dst_token_addr = self
            .config
            .get_token_address(routing.dst_chain, &routing.dst_token)
            .ok_or(OnlySwapsClientError::UnsupportedToken(
                routing.dst_chain,
                routing.dst_token,
            ))?;

        Ok((provider, src_chain_config, src_token_addr, dst_token_addr))
    }

    /// Create a new swap, sending tokens to a recipient
    pub async fn approve_and_swap(
        &self,
        recipient: Address,
        amount: U256,
        fee: U256,
        routing: SwapRouting,
    ) -> Result<OnlySwapsRequestId, OnlySwapsClientError> {
        let (provider, src_chain_config, src_token_addr, dst_token_addr) =
            self.swap_params(&routing)?;

        // Approve spending of token on source chain by router contract, before swapping
        approve_spending(src_chain_config, provider, src_token_addr, amount + fee).await?;

        swap(
            src_chain_config,
            provider,
            recipient,
            amount,
            fee,
            routing.dst_chain,
            src_token_addr,
            dst_token_addr,
        )
        .await?
    }

    /// Verifies if the swap with the given request_id has been completed on the destination chain.
    pub async fn is_swap_complete(
        &self,
        request_id: OnlySwapsRequestId,
        dst_chain: u64,
    ) -> Result<bool, OnlySwapsClientError> {
        let dst_router = self
            .config
            .get_router_instance(dst_chain)
            .ok_or(OnlySwapsClientError::UnsupportedChain(dst_chain))?;

        // Different chains, can't multicall, but start both futures at the same time
        let swap_receipt = dst_router
            .getSwapRequestReceipt(request_id)
            .call()
            .await
            .map_err(|e| {
                OnlySwapsClientError::Contract(
                    e,
                    "failed to execute getSwapRequestReceipt RPC static call",
                )
            })?;

        if swap_receipt.requestId.is_zero() {
            // if request is not found on destination chain, requestId == 0 => swap not yet fulfilled
            Ok(false)
        } else {
            // swap complete
            Ok(true)
        }
    }

    /// Verifies if the swap with the given request_id has been verified on the destination chain.
    /// A verified swap implies that the swap has been completed on the destination chain.
    pub async fn is_swap_verified(
        &self,
        request_id: OnlySwapsRequestId,
        src_chain: u64,
    ) -> Result<bool, OnlySwapsClientError> {
        let src_router = self
            .config
            .get_router_instance(src_chain)
            .ok_or(OnlySwapsClientError::UnsupportedChain(src_chain))?;

        let swap_params = src_router
            .getSwapRequestParameters(request_id)
            .call()
            .await
            .map_err(|e| {
                OnlySwapsClientError::Contract(
                    e,
                    "failed to execute getSwapRequestReceipt RPC static call",
                )
            })?;

        // nonce should be non-zero, otherwise swap not found
        if swap_params.nonce.is_zero() {
            Err(OnlySwapsClientError::SwapRequestNotFound)?
        }

        // executed is true when the solver has completed the transfer, and
        // it has been verified by the dcipher network
        Ok(swap_params.executed)
    }

    /// Obtain the current status of a swap.
    pub async fn status(
        &self,
        request_id: OnlySwapsRequestId,
        src_chain: u64,
        dst_chain: u64,
    ) -> Result<OnlySwapsStatus, OnlySwapsClientError> {
        // Different chains, can't multicall, but start both futures at the same time
        let (verified_res, completed_res) = futures_util::future::join(
            self.is_swap_verified(request_id, src_chain),
            self.is_swap_complete(request_id, dst_chain),
        )
        .await;

        let (verified, completed) = match (verified_res, completed_res) {
            (Ok(verified), Ok(completed)) => (verified, completed),
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        };

        match (verified, completed) {
            // verified, completed
            (true, true) => Ok(OnlySwapsStatus::Verified),
            // not verified, completed
            (false, true) => Ok(OnlySwapsStatus::Completed),
            // not verified, not completed
            (false, false) => Ok(OnlySwapsStatus::Pending),
            // any other combination should not happen
            _ => panic!(
                "(verified, completed) = ({verified}, {completed}) status should not be possible"
            ),
        }
    }

    /// Wait until a swap is completed on the destination chain
    pub async fn wait_until_complete(
        &self,
        request_id: OnlySwapsRequestId,
        dst_chain: u64,
    ) -> Result<(), OnlySwapsClientError> {
        let dst_router = self
            .config
            .get_router_instance(dst_chain)
            .ok_or(OnlySwapsClientError::UnsupportedChain(dst_chain))?;

        // Issue a registration for upcoming events
        let mut filter = dst_router
            .SwapRequestFulfilled_filter()
            .topic1(request_id)
            .watch()
            .await
            .map_err(|e| {
                OnlySwapsClientError::Contract(
                    e.into(),
                    "failed to watch SwapRequestFulfilled event",
                )
            })?
            .into_stream();

        // If the event was in the past, issue an RPC call
        if self.is_swap_complete(request_id, dst_chain).await? {
            return Ok(());
        }

        // Swap not yet completed, wait for it through the filter
        let (swap_completed_event, _) = filter
            .next()
            .await
            .expect("empty event stream empty!! provider dropped?")
            .map_err(|e| {
                OnlySwapsClientError::Contract(
                    alloy::contract::Error::AbiError(e.into()),
                    "failed to obtain obtain SwapRequestFulfilled event occurrence",
                )
            })?;
        assert_eq!(
            request_id, swap_completed_event.requestId,
            "detected an event for a different request id, filter broken?"
        );

        Ok(())
    }

    /// Wait until a swap reaches a specific status
    pub async fn wait_until_verified(
        &self,
        request_id: OnlySwapsRequestId,
        src_chain: u64,
    ) -> Result<(), OnlySwapsClientError> {
        let src_router = self
            .config
            .get_router_instance(src_chain)
            .ok_or(OnlySwapsClientError::UnsupportedChain(src_chain))?;

        // Issue a registration for upcoming events
        let mut filter = src_router
            .SolverPayoutFulfilled_filter()
            .topic1(request_id)
            .watch()
            .await
            .map_err(|e| {
                OnlySwapsClientError::Contract(
                    e.into(),
                    "failed to watch SolverPayoutFulfilled event",
                )
            })?
            .into_stream();

        // If the event was in the past, issue an RPC call
        if self.is_swap_verified(request_id, src_chain).await? {
            return Ok(());
        }

        // Swap not yet verified, wait for it through the filter
        let (swap_verified_event, _) = filter
            .next()
            .await
            .expect("empty event stream empty!! provider dropped?")
            .map_err(|e| {
                OnlySwapsClientError::Contract(
                    alloy::contract::Error::AbiError(e.into()),
                    "failed to obtain SolverPayoutFulfilled event occurrence",
                )
            })?;
        assert_eq!(
            request_id, swap_verified_event.requestId,
            "detected an event for a different request id, filter broken?"
        );

        Ok(())
    }
}

/// Recovers the request id from a log containing a SwapRequested event
///
/// # Panics
/// If the log returns more than one SwapRequested event
fn request_id_from_swap_logs<'l>(
    logs: impl IntoIterator<Item = &'l Log<LogData>>,
) -> Option<OnlySwapsRequestId> {
    let swap_requested_logs: Vec<_> = logs
        .into_iter()
        .filter_map(|log| Router::SwapRequested::decode_log_validate(&log.inner).ok())
        .collect();

    if swap_requested_logs.is_empty() {
        // Event not in log
        None?
    } else if swap_requested_logs.len() > 1 {
        // More than one event in log?? Could be a bug / bug-inducing.
        tracing::error!(
            ?swap_requested_logs,
            "Got multiple swap_requested logs in a single receipt"
        );
        panic!("Got multiple swap_requested logs in a single receipt");
    };

    Some(swap_requested_logs[0].requestId)
}

/// Approve the router contract to spend tokens for upcoming swaps.
/// The amount must include the fees.
async fn approve_spending(
    chain_config: &ChainConfig,
    provider: impl Provider,
    token_addr: Address,
    amount: U256,
) -> Result<TxHash, OnlySwapsClientError> {
    let ierc20 = IERC20::new(token_addr, provider);
    let call = ierc20.approve(chain_config.router_address, amount);

    let _ = tracing::trace_span!("approve_call", ?call).entered();
    tracing::debug!(%token_addr, router_address = %chain_config.router_address, %amount, "Sending approve(router_address, amount) transaction for ERC20 token");
    let tx_hash = call
        .send()
        .await
        .map_err(|e| OnlySwapsClientError::Contract(e, "failed to send approve tx"))?
        .with_required_confirmations(chain_config.required_confirmations)
        .with_timeout(Some(chain_config.timeout))
        .watch()
        .await?;
    tracing::debug!(?tx_hash, "approve tx mined");

    Ok(tx_hash)
}

#[allow(clippy::too_many_arguments)]
async fn swap(
    src_chain_config: &ChainConfig,
    provider: &DynProvider,
    recipient: Address,
    amount: U256,
    fee: U256,
    dst_chain: u64,
    src_token_addr: Address,
    dst_token_addr: Address,
) -> Result<Result<OnlySwapsRequestId, OnlySwapsClientError>, OnlySwapsClientError> {
    let router = RouterInstance::new(src_chain_config.router_address, provider);
    let call = router.requestCrossChainSwap(
        src_token_addr,
        dst_token_addr,
        amount,
        fee,
        U256::from(dst_chain),
        recipient,
    );

    // Do an RPC call first to make sure it works before sending a tx
    tracing::debug!(?call, "Executing requestCrossChainSwap RPC call");
    let _ = call.clone().call().await.map_err(|e| {
        OnlySwapsClientError::Contract(e, "failed to execute requestCrossChainSwap RPC static call")
    })?;

    // RPC call worked, sign TX and send it
    tracing::debug!(?call, "Sending requestCrossChainSwap transaction");
    let pending_tx = call.send().await.map_err(|e| {
        OnlySwapsClientError::Contract(e, "failed to send requestCrossChainSwap transaction")
    })?;
    let receipt = pending_tx
        .with_required_confirmations(src_chain_config.required_confirmations)
        .with_timeout(Some(src_chain_config.timeout))
        .get_receipt()
        .await?;
    tracing::debug!(?receipt, "Got receipt for requestCrossChainSwap");

    // Parse the logs to recover the request id of the swap
    let request_id = request_id_from_swap_logs(receipt.logs())
        .ok_or(OnlySwapsClientError::SwapFailedNotInLogs)?;
    Ok(Ok(request_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Bytes, address};

    #[test]
    fn should_find_request_id_from_swap_logs() {
        let expected_request_id =
            "0xeb31af76010d214bc5528caefcaff389bc66837969284c152096af56b79352eb"
                .parse()
                .unwrap();
        let log = Log {
            inner: alloy::primitives::Log {
                address: address!("0xc69dd549b037215ba1ea9866ffa59603862bf986"),
                data: LogData::new(
                    vec![
                        "0xe2f0160fbe779abb246276d49dac0cb97c86df7cb72a510bb9cf21486f5c82af"
                            .parse()
                            .unwrap(),
                        expected_request_id,
                        "0x0000000000000000000000000000000000000000000000000000000000014a34"
                            .parse()
                            .unwrap(),
                        "0x000000000000000000000000000000000000000000000000000000000000a869"
                            .parse()
                            .unwrap(),
                    ],
                    Bytes::new(),
                )
                .unwrap(),
            },
            block_hash: Some(
                "0x06557bfad2fb95662b462727b3de0168585a746e78c7f80ea8b536576b3f3f75"
                    .parse()
                    .unwrap(),
            ),
            block_number: Some(32589031),
            block_timestamp: Some(1760946350),
            transaction_hash: Some(
                "0xe9231c3fce0b51c65f35e5d62675802997b287815504d98d48b2ebce477a8c73"
                    .parse()
                    .unwrap(),
            ),
            transaction_index: Some(6),
            log_index: Some(6),
            removed: false,
        };

        let request_id =
            request_id_from_swap_logs(&[log]).expect("failed to find request_id from logs");
        assert_eq!(request_id, expected_request_id);
    }

    mod ci_only {
        use super::*;
        use crate::config::chain::{AVAX_FUJI, BASE_SEPOLIA};
        use alloy::network::{EthereumWallet, NetworkWallet};
        use alloy::primitives::utils::Unit;
        use alloy::providers::{ProviderBuilder, WsConnect};
        use alloy::signers::local::PrivateKeySigner;
        use std::sync::LazyLock;
        use std::time::Duration;

        const TESTNETS_PRIVATE_KEY_ENV: &str = "TESTNETS_PRIVATE_KEY";
        const BASE_SEPOLIA_RPC_URL_ENV: &str = "BASE_SEPOLIA_RPC_URL";
        const AVALANCHE_FUJI_RPC_URL_ENV: &str = "AVALANCHE_FUJI_RPC_URL";
        const SWAP_TIMEOUT: Duration = Duration::from_millis(60000); // 60s
        static SWAP_AMOUNT: LazyLock<U256> = LazyLock::new(|| U256::from(1) * Unit::ETHER.wei());
        static SWAP_FEE: LazyLock<U256> = LazyLock::new(|| U256::from(1) * Unit::ETHER.wei());

        async fn default_client() -> (EthereumWallet, OnlySwapsClient) {
            let testnet_signer: PrivateKeySigner = std::env::var(TESTNETS_PRIVATE_KEY_ENV)
                .expect("testnet private key should be set")
                .parse()
                .unwrap();
            let testnet_wallet = EthereumWallet::from(testnet_signer);

            let mut config = OnlySwapsClientConfig::empty();
            config.add_ethereum_chain(
                BASE_SEPOLIA.to_owned(),
                ProviderBuilder::new()
                    .wallet(testnet_wallet.clone())
                    .connect_ws(WsConnect::new(
                        std::env::var(BASE_SEPOLIA_RPC_URL_ENV)
                            .expect("base sepolia rpc url should be set"),
                    ))
                    .await
                    .expect("invalid provider"),
            );
            config.add_ethereum_chain(
                AVAX_FUJI.to_owned(),
                ProviderBuilder::new()
                    .wallet(testnet_wallet.clone())
                    .connect_ws(WsConnect::new(
                        std::env::var(AVALANCHE_FUJI_RPC_URL_ENV)
                            .expect("avalanche fuji rpc url should be set"),
                    ))
                    .await
                    .expect("invalid provider"),
            );

            let client = OnlySwapsClient::new(config);
            (testnet_wallet, client)
        }

        #[tokio::test]
        #[ignore = "ci-only"] // only run in ci
        async fn swap_rusd_base_sepolia_to_avalanche_fuji() {
            let _ = tracing_subscriber::FmtSubscriber::builder()
                .with_max_level(tracing::Level::DEBUG)
                .try_init();

            let routing = SwapRouting::new_same_token(
                BASE_SEPOLIA.chain_id,
                AVAX_FUJI.chain_id,
                TokenTag::RUSD,
            );
            swap_and_verify_with_timeout(routing, *SWAP_AMOUNT, *SWAP_FEE, SWAP_TIMEOUT).await;
        }

        #[tokio::test]
        #[ignore = "ci-only"] // only run in ci
        async fn swap_rusd_avalanche_fuji_to_base_sepolia() {
            let _ = tracing_subscriber::FmtSubscriber::builder()
                .with_max_level(tracing::Level::DEBUG)
                .try_init();

            let routing = SwapRouting::new_same_token(
                AVAX_FUJI.chain_id,
                BASE_SEPOLIA.chain_id,
                TokenTag::RUSD,
            );
            swap_and_verify_with_timeout(routing, *SWAP_AMOUNT, *SWAP_FEE, SWAP_TIMEOUT).await;
        }

        async fn swap_and_verify_with_timeout(
            routing: SwapRouting,
            swap_amount: U256,
            swap_fee: U256,
            swap_timeout: Duration,
        ) {
            let (testnet_wallet, client) = default_client().await;
            let swap_id = client
                .approve_and_swap(
                    NetworkWallet::<Ethereum>::default_signer_address(&testnet_wallet),
                    swap_amount,
                    swap_fee,
                    routing,
                )
                .await
                .expect("failed to approve and swap");

            // Wait for swap to be verified, up to a set timeout
            tokio::time::timeout(
                swap_timeout,
                client.wait_until_verified(swap_id, routing.src_chain),
            )
            .await
            .expect("swap verification timed out")
            .expect("failed to check verification status of swap");
        }
    }
}
