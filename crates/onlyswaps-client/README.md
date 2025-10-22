# onlyswaps-client

A Rust client library for interacting with the only swaps cross-chain token swap protocol, part of the dcipher ecosystem.

## Overview

`onlyswaps-client` provides a high-level interface for performing cross-chain token swaps using the OnlySwaps protocol. It handles token approvals, swap requests, and verification tracking across multiple blockchain networks.

## Quick Start

```rust
use onlyswaps_client::client::OnlySwapsClient;
use onlyswaps_client::client::routing::SwapRouting;
use onlyswaps_client::config::OnlySwapsClientConfig;
use onlyswaps_client::config::chain::{BASE_SEPOLIA, AVAX_FUJI};
use onlyswaps_client::config::token::TokenTag;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::primitives::U256;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let mut config = OnlySwapsClientConfig::empty();

    // Add chains with their providers
    config.add_ethereum_chain(
        BASE_SEPOLIA.to_owned(),
        ProviderBuilder::new()
            .wallet(your_wallet)
            .connect_ws(WsConnect::new(your_rpc_url))
            .await?
    );

    config.add_ethereum_chain(
        AVAX_FUJI.to_owned(),
        ProviderBuilder::new()
            .wallet(your_wallet)
            .connect_ws(WsConnect::new(your_rpc_url))
            .await?
    );

    // Create client
    let client = OnlySwapsClient::new(config);

    // Define swap routing
    let routing = SwapRouting::new_same_token(
        BASE_SEPOLIA.chain_id,
        AVAX_FUJI.chain_id,
        TokenTag::RUSD,
    );

    // Execute swap with approval
    let swap_id = client.approve_and_swap(
        recipient_address,
        U256::from(1000000), // amount
        U256::from(10000),   // fee
        routing,
    ).await?;

    println!("Swap initiated with ID: {}", swap_id);

    // Wait for verification
    client.wait_until_verified(swap_id, routing.src_chain).await?;
    println!("Swap verified!");

    Ok(())
}
```
