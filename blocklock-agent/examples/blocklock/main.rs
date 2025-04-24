mod arguments_parser;
mod healthcheck;

use crate::arguments_parser::{BlocklockArgs, BlocklockConfig, NodesConfiguration};
use crate::healthcheck::healthcheck;
use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use ark_ec::{AffineRepr, CurveGroup};
use blocklock_agent::{BLOCKLOCK_SCHEME_ID, NotifyTicker, run_agent};
use dcipher_agents::agents::blocklock::agent::{BlocklockAgent, BlocklockAgentSavedState};
use dcipher_agents::agents::blocklock::contracts::BlocklockSender;
use dcipher_agents::agents::blocklock::fulfiller::BlocklockFulfiller;
use dcipher_agents::decryption_sender::contracts::DecryptionSender;
use dcipher_agents::decryption_sender::threshold_signer::ThresholdSigner;
use dcipher_agents::decryption_sender::{DecryptionRequest, DecryptionSenderFulfillerConfig};
use dcipher_agents::fulfiller::{RequestChannel, Stopper, TickerBasedFulfiller};
use dcipher_agents::ibe_helper::IbeIdentityOnBn254G1Suite;
use std::time::Duration;
use superalloy::provider::create_provider_with_retry;
use superalloy::retry::RetryStrategy;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::FmtSubscriber;

fn create_threshold_fulfiller<'lt_in, 'lt_out, P>(
    args: &'lt_in BlocklockArgs,
    nodes_config: &'lt_in NodesConfiguration,
    decryption_sender_contract: DecryptionSender::DecryptionSenderInstance<(), P>,
    blocklock_sender_contract: BlocklockSender::BlocklockSenderInstance<(), P>,
) -> anyhow::Result<(
    NotifyTicker,
    CancellationToken,
    impl Stopper + 'lt_out,
    impl RequestChannel<Request = DecryptionRequest> + 'lt_out,
)>
where
    P: Provider + Clone + 'static,
{
    // Parse key
    let sk: ark_bn254::Fr = args.key_config.bls_key.to_owned().into();

    // Get per-nodes config
    let (mut pks, addresses, peer_ids, short_ids): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = nodes_config
        .nodes
        .iter()
        .cloned()
        .map(|c| (c.bls_pk, c.address, c.peer_id, c.node_id.get()))
        .collect();

    // Add own pk to the list if required
    if pks.len() == usize::from(args.key_config.n.get() - 1) {
        let pk = ark_bn254::G2Affine::generator() * sk;
        pks.insert(
            usize::from(args.key_config.node_id.get() - 1),
            pk.into_affine(),
        );
    }

    // Create a threshold signer
    let cs = IbeIdentityOnBn254G1Suite::new(
        b"BLOCKLOCK",
        args.chain
            .chain_id
            .expect("chain id must have been set here"),
    );
    let ts = ThresholdSigner::new(
        cs,
        sk,
        args.key_config.n.get(),
        args.key_config.t.get(),
        args.key_config.node_id.get(),
        pks,
    );

    let (ts_stopper, signing_registry) = ts.run(
        args.libp2p.libp2p_key.clone().into(),
        args.libp2p.libp2p_listen_addr.clone(),
        addresses,
        peer_ids,
        short_ids,
    );

    // Create a transaction fulfiller
    let single_call_tx_fulfiller = BlocklockFulfiller::new(
        decryption_sender_contract,
        blocklock_sender_contract,
        args.chain.min_confirmations,
        Duration::from_secs(args.chain.confirmations_timeout_secs),
        args.chain.base_gas_price_wei,
    );

    // Create a ticker-based fulfiller
    let fulfiller = DecryptionSenderFulfillerConfig::new_fulfiller(
        signing_registry,
        single_call_tx_fulfiller,
        args.chain.max_tx_per_tick,
        dcipher_agents::fulfiller::RetryStrategy::Never,
    );

    let ticker = NotifyTicker::default();
    let (stopper, channel) = fulfiller.run(ticker.clone());
    Ok((ticker, ts_stopper, stopper, channel))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let BlocklockConfig {
        mut config,
        nodes_config,
    } = BlocklockConfig::parse()?;

    // Set logging options
    FmtSubscriber::builder()
        .with_max_level(config.log_level)
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
        .on_provider(ro_provider.clone());
    let decryption_sender_contract_ro =
        DecryptionSender::new(config.chain.decryption_sender_addr, ro_provider);
    let decryption_sender_contract =
        DecryptionSender::new(config.chain.decryption_sender_addr, provider.clone());
    let blocklock_sender_contract =
        BlocklockSender::new(config.chain.decryption_sender_addr, provider.clone());

    // If chain id is none, fetch it from the provider
    if config.chain.chain_id.is_none() {
        let chain_id = provider.get_chain_id().await?;
        config.chain.chain_id.replace(chain_id);
    }

    // Create a fulfiller
    let (ticker, ts_stopper, stopper, channel) = create_threshold_fulfiller(
        &config,
        &nodes_config.unwrap_or_default(),
        decryption_sender_contract.clone(),
        blocklock_sender_contract,
    )?;

    // Create the blocklock agent from a saved state
    let saved_state = std::fs::read(&config.state_file).unwrap_or_default();
    let saved_state: BlocklockAgentSavedState =
        serde_json::from_slice(&saved_state).unwrap_or_default();
    let mut agent = BlocklockAgent::from_state(
        BLOCKLOCK_SCHEME_ID,
        config.chain.sync_batch_size,
        channel,
        decryption_sender_contract_ro.clone(),
        saved_state,
    )
    .await?;

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
            ts_stopper.cancel();
            Ok(())
        },

        _ = tokio::signal::ctrl_c() => {
            println!("received ctrl+c, shutting down...");
            Ok(())
        },

        err = run_agent(&mut agent, ticker, decryption_sender_contract_ro) => {
            eprintln!("agent stopped unexpectedly...");
            err // return Result
        },

        err = healthcheck(&config.healthcheck_listen_addr, config.healthcheck_port) => {
            eprintln!("healthcheck stopped unexpectedly...");
            err // return Result
        }
    };

    // Stop the various components
    ts_stopper.cancel();
    stopper.stop().await;

    // On success, save the state of the agent
    if res.is_ok() {
        let saved_state = agent.save_state();
        let saved_state = serde_json::to_string_pretty(&saved_state)?;
        std::fs::write(&config.state_file, saved_state)?;
        println!(
            "Saved blocklock agent state to: {}",
            config.state_file.display()
        );
    }

    res
}
