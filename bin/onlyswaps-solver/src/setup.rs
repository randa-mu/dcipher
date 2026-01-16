use crate::network::Network;
use alloy::primitives::{Address, U160, U256, address};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use anyhow::Context;
use futures::TryFutureExt;
use generated::onlyswaps::ierc20::IERC20::IERC20Instance;
use onlyswaps_client::client::OnlySwapsClient;
use std::collections::HashMap;
use std::io::{Write, stdin};

const PERMIT2_DEFAULT_ADDRESS: Address = address!("0x000000000022D473030F116dDEE9F6B43aC78BA3");

pub async fn setup_allowances(
    client: &OnlySwapsClient,
    networks: &HashMap<u64, Network<DynProvider>>,
) -> anyhow::Result<()> {
    let allowances = fetch_allowance_details(client, networks).await?;

    println!("The following allowances are required:");
    println!("{:<10} {:<44} Permit2", "Chain", "Token Address");
    println!("{}", "-".repeat(90));

    let mut calls = vec![];
    for (chain_id, details) in allowances {
        let net = networks
            .get(&chain_id)
            .context("tried setting allowance for unknown chain??")?;

        // all tokens on a network share the same provider
        let Some(provider) = net.tokens.first().map(|t| t.provider()) else {
            continue;
        };
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .with_cached_nonce_management() // ensure the provider uses a cached nonce for concurrency
            .connect_provider(provider);

        for (amount, token) in details
            .current_allowances
            .into_iter()
            .zip(net.tokens.iter())
        {
            let token_address = *token.address();
            if amount > U256::from(U160::MAX) {
                // we consider anything up to 2**160 a max allowance
                continue;
            }

            let permit2_display = if details.permit2_address == PERMIT2_DEFAULT_ADDRESS {
                "Default".to_string()
            } else {
                format!("Custom ({})", details.permit2_address)
            };

            println!(
                "{chain_id:<10} {:<44} {permit2_display}",
                token_address.to_string(),
            );
            calls.push((
                chain_id,
                IERC20Instance::new(token_address, provider.clone()),
                details.permit2_address,
                net.own_addr,
            ));
        }
    }

    if calls.is_empty() {
        println!("\nNo allowances required.");
        return Ok(());
    }

    print!("\nProceed? [y/n]: ");
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    if buffer.trim().to_lowercase().as_str() == "n" {
        return Ok(());
    }

    println!("Sending txs...");
    let results = futures::future::join_all(calls.into_iter().map(
        async move |(chain_id, token, permit2_address, own_addr)| {
            println!(
                "> [Chain {chain_id}] sending approve for {}...",
                token.address()
            );

            let approve = token.approve(permit2_address, U256::MAX).from(own_addr);
            let res = approve
                .send()
                .and_then(async move |tx_watch| Ok(tx_watch.watch().await?))
                .await
                .inspect(|_| {
                    println!(
                        "> [Chain {chain_id}] approval for {} sent successfully",
                        token.address()
                    );
                })
                .inspect_err(|_| {
                    println!(
                        "> [Chain {chain_id}] approval for {} failed",
                        token.address()
                    );
                });

            (chain_id, *token.address(), res)
        },
    ))
    .await;

    println!("\nTransaction results:");
    println!("{:<10} {:<44} Result", "Chain", "Token Address");
    println!("{}", "-".repeat(90));
    for (chain_id, token, res) in results {
        print!("{chain_id:<10} {:<44} ", token.to_string());
        match res {
            Ok(tx_hash) => println!("mined in {tx_hash}"),
            Err(e) => println!("tx failed: {e:?}"),
        }
    }

    Ok(())
}

struct AllowanceDetails {
    permit2_address: Address,
    current_allowances: Vec<U256>,
}

/// Prepare the solver by fetching the current token allowances on each network
async fn fetch_allowance_details(
    client: &OnlySwapsClient,
    networks: &HashMap<u64, Network<DynProvider>>,
) -> anyhow::Result<HashMap<u64, AllowanceDetails>> {
    // First, fetch the permit2 addresses
    let permit2_addresses: HashMap<_, _> = client
        .fetch_permit2_addresses(
            networks
                .iter()
                .map(|(id, net)| (*id, net.permit2_relayer_address)),
        )
        .await
        .context("failed to get permit2 addresses")?
        .collect();

    let current_allowances_per_network =
        futures::future::try_join_all(networks.iter().map(async |(&chain_id, net)| {
            let permit2_addr = *permit2_addresses
                .get(&chain_id)
                .with_context(|| format!("failed to get permit2 address of chain {chain_id}"))?;
            let mut token_addresses = vec![];
            let mut multicall = net.router.provider().multicall().dynamic();
            for token in net.tokens.iter() {
                token_addresses.push(*token.address());
                multicall = multicall.add_dynamic(token.allowance(net.own_addr, permit2_addr));
            }

            multicall
                .aggregate()
                .await
                .map(move |allowances| {
                    (
                        chain_id,
                        AllowanceDetails {
                            permit2_address: permit2_addr,
                            current_allowances: allowances,
                        },
                    )
                })
                .context("multicall failed")
        }))
        .await
        .context("failed to get current allowances")?;

    Ok(HashMap::from_iter(current_allowances_per_network))
}
