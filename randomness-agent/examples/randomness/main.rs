mod arguments_parser;
mod healthcheck;

use crate::arguments_parser::{NodesConfiguration, RandomnessAgentArgs, RandomnessAgentConfig};
use crate::healthcheck::start_api;
use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder, WalletProvider};
use alloy::signers::local::PrivateKeySigner;
use ark_ec::{AffineRepr, CurveGroup};
use dcipher_agents::agents::randomness::RandomnessAgent;
use dcipher_agents::agents::randomness::contracts::RandomnessSender;
use dcipher_agents::agents::randomness::fulfiller::RandomnessFulfiller;
use dcipher_agents::fulfiller::{RequestChannel, Stopper, TickerBasedFulfiller};
use dcipher_agents::signature_sender::contracts::SignatureSender;
use dcipher_agents::signature_sender::{SignatureRequest, SignatureSenderFulfillerConfig};
use dcipher_agents::signer::BLS12_381SignatureOnG1Signer;
use dcipher_agents::signer::threshold_signer::ThresholdSigner;
use dcipher_network::transports::libp2p::{Libp2pNode, Libp2pNodeConfig};
use randomness_agent::{NotifyTicker, RANDOMNESS_SCHEME_ID, run_agent};
use std::time::Duration;
use superalloy::provider::create_provider_with_retry;
use superalloy::retry::RetryStrategy;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::Layer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let RandomnessAgentConfig {
        mut config,
        nodes_config,
    } = RandomnessAgentConfig::parse()?;

    // Set logging options
    let json_layer = if config.log_json {
        tracing_subscriber::fmt::layer().json().boxed()
    } else {
        tracing_subscriber::fmt::layer().boxed()
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.log_level))
        .with(json_layer)
        .init();

    // Create a wallet
    let signer: PrivateKeySigner = config.chain.tx_private_key.parse()?;
    let wallet = EthereumWallet::from(signer);

    // Create provider and instantiate the decryption sender contract
    let ro_provider =
        create_provider_with_retry(config.chain.rpc_url.clone(), RetryStrategy::None).await?;
    let provider = ProviderBuilder::default()
        .with_recommended_fillers()
        .wallet(wallet)
        .connect_provider(ro_provider.clone());
    let signature_sender_contract_ro =
        SignatureSender::new(config.chain.signature_sender_addr, ro_provider);
    let signature_sender_contract =
        SignatureSender::new(config.chain.signature_sender_addr, provider.clone());
    let randomness_sender_contract =
        RandomnessSender::new(config.chain.randomness_sender_addr, provider.clone());

    // If chain id is none, fetch it from the provider
    if config.chain.chain_id.is_none() {
        let chain_id = provider.get_chain_id().await?;
        config.chain.chain_id.replace(chain_id);
    }

    // Create a fulfiller
    let (ticker, libp2p_node, ts_stopper, stopper, channel) = create_threshold_fulfiller(
        &config,
        &nodes_config.unwrap_or_default(),
        signature_sender_contract.clone(),
        randomness_sender_contract,
    )?;

    // Create a new randomness agent
    let mut agent = RandomnessAgent::new(
        RANDOMNESS_SCHEME_ID,
        config.chain.sync_batch_size,
        channel,
        signature_sender_contract_ro.clone(),
    );

    // Setup some signals
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    // Execute the agent and the healthcheck
    let res = tokio::select! {
        _ = sigterm.recv() => {
            println!("received SIGTERM, shutting down...");
            Ok(())
        },

        _ = sigint.recv() => {
            println!("received SIGINT, shutting down...");
            Ok(())
        },

        _ = tokio::signal::ctrl_c() => {
            println!("received ctrl+c, shutting down...");
            Ok(())
        },

        err = run_agent(&mut agent, ticker, signature_sender_contract_ro) => {
            eprintln!("agent stopped unexpectedly...");
            err // return Result
        },

        err = start_api(config.healthcheck_listen_addr, config.healthcheck_port) => {
            eprintln!("healthcheck stopped unexpectedly...");
            err // return Result
        }
    };

    // Stop the various components
    if let Err(e) = libp2p_node.stop().await {
        tracing::error!(error = ?e, "Failed to stop libp2p node");
    }
    ts_stopper.cancel();
    stopper.stop().await;

    res
}

fn create_threshold_fulfiller<'lt_in, 'lt_out, P>(
    args: &'lt_in RandomnessAgentArgs,
    nodes_config: &'lt_in NodesConfiguration,
    signature_sender_contract: SignatureSender::SignatureSenderInstance<P>,
    randomness_sender_contract: RandomnessSender::RandomnessSenderInstance<P>,
) -> anyhow::Result<(
    NotifyTicker,
    Libp2pNode<u16>,
    CancellationToken,
    impl Stopper + 'lt_out,
    impl RequestChannel<Request = SignatureRequest> + 'lt_out,
)>
where
    P: Provider + WalletProvider + Clone + 'static,
{
    // Parse key
    let sk: ark_bls12_381::Fr = args.key_config.bls_key.to_owned().into();

    // Get per-nodes config
    let (mut pks, addresses, peer_ids, short_ids): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = nodes_config
        .nodes
        .iter()
        .cloned()
        .map(|c| (c.bls_pk, c.address, c.peer_id, c.node_id.get()))
        .collect();

    // Add own pk to the list if required
    if pks.len() == usize::from(args.key_config.n.get() - 1) {
        let pk = ark_bls12_381::G2Affine::generator() * sk;
        pks.insert(
            usize::from(args.key_config.node_id.get() - 1),
            pk.into_affine(),
        );
    }

    // Create a threshold signer
    let dst = format!(
        "dcipher-randomness-v01-BLS12381G1_XMD:SHA-256_SSWU_RO_0x{:064x}_",
        args.chain
            .chain_id
            .expect("chain id must have been set here")
    )
    .into_bytes();
    let cs = BLS12_381SignatureOnG1Signer::new(sk, dst);
    let ts = ThresholdSigner::new(
        cs,
        args.key_config.n.get(),
        args.key_config.t.get(),
        args.key_config.node_id.get(),
        pks,
    );

    // Create a libp2p transport and start it
    let mut libp2p_node = Libp2pNodeConfig::new(
        args.libp2p.libp2p_key.clone().into(),
        args.key_config.node_id.get(),
        addresses,
        peer_ids,
        short_ids,
    )
    .run(args.libp2p.libp2p_listen_addr.clone())?;

    let (ts_stopper, signer) = ts.run(
        libp2p_node
            .get_transport()
            .expect("newly created node should have a transport"),
    );

    // Create a transaction fulfiller
    let mut signature_tx_fulfiller = RandomnessFulfiller::new(
        signature_sender_contract,
        randomness_sender_contract,
        args.chain.min_confirmations,
        Duration::from_secs(args.chain.confirmations_timeout_secs),
        args.chain.gas_buffer_percent,
        args.chain.gas_price_buffer_percent,
        args.chain.profit_threshold,
    );

    if args.chain.tx_fulfillment_disabled {
        // Disable the transaction fufillment of the request if requested.
        signature_tx_fulfiller.set_simulate_tx();
    }

    // Create a ticker-based fulfiller
    let fulfiller = SignatureSenderFulfillerConfig::new_fulfiller(
        signer,
        signature_tx_fulfiller,
        args.chain.max_tx_per_tick,
        args.chain.tx_retry_strategy,
    );

    let ticker = NotifyTicker::default();
    let (stopper, channel) = fulfiller.run(ticker.clone());
    Ok((ticker, libp2p_node, ts_stopper, stopper, channel))
}
