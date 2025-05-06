mod arguments_parser;
mod healthcheck;

use crate::arguments_parser::{NodesConfiguration, RandomnessAgentArgs, RandomnessAgentConfig};
use crate::healthcheck::healthcheck;
use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder, WalletProvider};
use alloy::signers::local::PrivateKeySigner;
use ark_ec::{AffineRepr, CurveGroup};
use dcipher_agents::agents::randomness::RandomnessAgent;
use dcipher_agents::fulfiller::{RequestChannel, Stopper, TickerBasedFulfiller};
use dcipher_agents::signature_sender::contracts::SignatureSender;
use dcipher_agents::signature_sender::fulfiller::SignatureFulfiller;
use dcipher_agents::signature_sender::{SignatureRequest, SignatureSenderFulfillerConfig};
use dcipher_agents::signer::BN254SignatureOnG1Signer;
use dcipher_agents::signer::threshold_signer::ThresholdSigner;
use randomness_agent::{NotifyTicker, RANDOMNESS_SCHEME_ID, run_agent};
use std::time::Duration;
use superalloy::provider::create_provider_with_retry;
use superalloy::retry::RetryStrategy;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::FmtSubscriber;

fn create_threshold_fulfiller<'lt_in, 'lt_out, P>(
    args: &'lt_in RandomnessAgentArgs,
    nodes_config: &'lt_in NodesConfiguration,
    signature_sender_contract: SignatureSender::SignatureSenderInstance<P>,
) -> anyhow::Result<(
    NotifyTicker,
    CancellationToken,
    impl Stopper + 'lt_out,
    impl RequestChannel<Request = SignatureRequest> + 'lt_out,
)>
where
    P: Provider + WalletProvider + Clone + 'static,
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
    let dst = format!(
        "dcipher-randomness-v01-BN254G1_XMD:KECCAK-256_SVDW_RO_0x{:064x}_",
        args.chain
            .chain_id
            .expect("chain id must have been set here")
    )
    .into_bytes();
    let cs = BN254SignatureOnG1Signer::new(sk, dst);
    let ts = ThresholdSigner::new(
        cs,
        args.key_config.n.get(),
        args.key_config.t.get(),
        args.key_config.node_id.get(),
        pks,
    );

    let (ts_stopper, signer) = ts.run(
        args.libp2p.libp2p_key.clone().into(),
        args.libp2p.libp2p_listen_addr.clone(),
        addresses,
        peer_ids,
        short_ids,
    );

    // Create a transaction fulfiller
    let single_call_tx_fulfiller = SignatureFulfiller::new(
        signature_sender_contract,
        args.chain.min_confirmations,
        Duration::from_secs(args.chain.confirmations_timeout_secs),
        args.chain.gas_buffer_percent,
    );

    // Create a ticker-based fulfiller
    let fulfiller = SignatureSenderFulfillerConfig::new_fulfiller(
        signer,
        single_call_tx_fulfiller,
        args.chain.max_tx_per_tick,
        args.chain.tx_retry_strategy,
    );

    let ticker = NotifyTicker::default();
    let (stopper, channel) = fulfiller.run(ticker.clone());
    Ok((ticker, ts_stopper, stopper, channel))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let RandomnessAgentConfig {
        mut config,
        nodes_config,
    } = RandomnessAgentConfig::parse()?;

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
        .connect_provider(ro_provider.clone());
    let signature_sender_contract_ro =
        SignatureSender::new(config.chain.signature_sender_addr, ro_provider);
    let signature_sender_contract =
        SignatureSender::new(config.chain.signature_sender_addr, provider.clone());

    // If chain id is none, fetch it from the provider
    if config.chain.chain_id.is_none() {
        let chain_id = provider.get_chain_id().await?;
        config.chain.chain_id.replace(chain_id);
    }

    // Create a fulfiller
    let (ticker, ts_stopper, stopper, channel) = create_threshold_fulfiller(
        &config,
        &nodes_config.unwrap_or_default(),
        signature_sender_contract.clone(),
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
            ts_stopper.cancel();
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

        err = healthcheck(&config.healthcheck_listen_addr, config.healthcheck_port) => {
            eprintln!("healthcheck stopped unexpectedly...");
            err // return Result
        }
    };

    // Stop the various components
    ts_stopper.cancel();
    stopper.stop().await;

    res
}
