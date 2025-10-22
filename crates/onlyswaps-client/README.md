# onlyswaps-client

A Rust client library for interacting with the only swaps cross-chain token swap protocol, part of the dcipher ecosystem.

## Overview

`onlyswaps-client` provides a high-level interface for performing cross-chain token swaps using the OnlySwaps protocol. It handles token approvals, swap requests, and verification tracking across multiple blockchain networks.

## Quick Start

```rust
use onlyswaps_client::FeeEstimator;
use onlyswaps_client::client::OnlySwapsClient;
use onlyswaps_client::client::OnlySwapsRequestBuilder;
use onlyswaps_client::client::routing::SwapRouting;
use onlyswaps_client::config::OnlySwapsClientConfig;
use onlyswaps_client::config::chain::{BASE_SEPOLIA, AVAX_FUJI};
use onlyswaps_client::config::token::TokenTag;
use alloy::network::EthereumWallet;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::primitives::U256;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = OnlySwapsClientConfig::empty();
    let wallet = EthereumWallet::default();

    // Add chains with their providers
    config.add_ethereum_chain(
        BASE_SEPOLIA.to_owned(),
        ProviderBuilder::new()
            .wallet(wallet.clone())
            .connect_ws(WsConnect::new("wss://base-sepolia-rpc.publicnode.com"))
            .await?
    );

    config.add_ethereum_chain(
        AVAX_FUJI.to_owned(),
        ProviderBuilder::new()
            .wallet(wallet)
            .connect_ws(WsConnect::new("wss://avalanche-fuji-c-chain-rpc.publicnode.com"))
            .await?
    );

    // Create client
    let client = OnlySwapsClient::new(config);

    // Create a new swaps reque st
    let request = OnlySwapsRequestBuilder::new()
        .recipient("0x00".parse().expect("a valid address"))
        .route(SwapRouting::new_same_token_from_configs(&BASE_SEPOLIA, &AVAX_FUJI, &TokenTag::RUSD))
        .exact_amount(U256::from(1_000_000_000_000_000_000u128), &FeeEstimator::default())
        .await?
        .build()
        .expect("a valid builder");

    // Execute swap with approval
    let swap_id = client.approve_and_swap(request).await?;
    println!("Swap initiated with ID: {}", swap_id);

    // Wait for verification
    client.wait_until_verified(swap_id, request.route.src_chain).await?;
    println!("Swap verified!");

    Ok(())
}
```
