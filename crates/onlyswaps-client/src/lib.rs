//! Crate providing an only swaps client which can be used to swap tokens from one chain to another
//! using the dcipher network.
//!
//! # Usage example with fee estimator
//! ```no_run
//! use onlyswaps_client::FeeEstimator;
//! use onlyswaps_client::client::OnlySwapsClient;
//! use onlyswaps_client::client::OnlySwapsRequestBuilder;
//! use onlyswaps_client::client::routing::SwapRouting;
//! use onlyswaps_client::config::OnlySwapsClientConfig;
//! use onlyswaps_client::config::chain::{BASE_SEPOLIA, AVAX_FUJI};
//! use onlyswaps_client::config::token::TokenTag;
//! use alloy::network::EthereumWallet;
//! use alloy::providers::{ProviderBuilder, WsConnect};
//! use alloy::primitives::U256;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut config = OnlySwapsClientConfig::empty();
//!     let wallet = EthereumWallet::default();
//!
//!     // Add chains with their providers
//!     config.add_ethereum_chain(
//!         BASE_SEPOLIA.to_owned(),
//!         ProviderBuilder::new()
//!             .wallet(wallet.clone())
//!             .connect_ws(WsConnect::new("wss://base-sepolia-rpc.publicnode.com"))
//!             .await?
//!     );
//!
//!     config.add_ethereum_chain(
//!         AVAX_FUJI.to_owned(),
//!         ProviderBuilder::new()
//!             .wallet(wallet)
//!             .connect_ws(WsConnect::new("wss://avalanche-fuji-c-chain-rpc.publicnode.com"))
//!             .await?
//!     );
//!
//!     // Create client
//!     let client = OnlySwapsClient::new(config);
//!
//!     // Create a new swaps reque st
//!     let request = OnlySwapsRequestBuilder::new()
//!         .recipient("0x00".parse().expect("a valid address"))
//!         .route(SwapRouting::new_same_token_from_configs(&BASE_SEPOLIA, &AVAX_FUJI, &TokenTag::RUSD))
//!         .exact_amount(U256::from(1_000_000_000_000_000_000u128), &FeeEstimator::default())
//!         .await?
//!         .build()
//!         .expect("a valid builder");
//!
//!     // Execute swap with approval
//!     let receipt = client.approve_and_swap(request).await?;
//!     println!("Swap initiated with ID: {}", receipt.request_id);
//!
//!     // Wait for verification
//!     client.wait_until_verified(&receipt).await?;
//!     println!("Swap verified!");
//!
//!     Ok(())
//! }
//! ```

pub mod client;

pub mod config;

#[cfg(feature = "fee-estimator")]
pub mod fee_estimator;

#[cfg(feature = "fee-estimator")]
pub use fee_estimator::*;

#[cfg(feature = "permit2")]
pub mod permit2;
