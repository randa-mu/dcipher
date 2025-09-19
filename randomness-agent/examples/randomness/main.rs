mod arguments_parser;
mod healthcheck;

use crate::arguments_parser::{NodesConfiguration, RandomnessAgentArgs, RandomnessAgentConfig};
use crate::healthcheck::start_api;
use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder, WalletProvider};
use alloy::signers::local::PrivateKeySigner;
use ark_ec::pairing::Pairing;
use dcipher_agents::agents::randomness::RandomnessAgent;
use dcipher_agents::agents::randomness::fulfiller::RandomnessFulfiller;
use dcipher_agents::fulfiller::ticker::OneshotStopper;
use dcipher_agents::fulfiller::{RequestChannel, Stopper, TickerBasedFulfiller};
use dcipher_agents::signature_sender::{SignatureRequest, SignatureSenderFulfillerConfig};
use dcipher_network::transports::libp2p::{Libp2pNode, Libp2pNodeConfig};
use dcipher_signer::bls::{BlsPairingSigner, BlsSigner, BlsThresholdSigner};
use dcipher_signer::dsigner::{
    ApplicationArgs, ApplicationRandomnessArgs, BlsSignatureAlgorithm, BlsSignatureCurve,
    BlsSignatureHash, SignatureAlgorithm,
};
use generated::randomness::randomness_sender::RandomnessSender;
use generated::randomness::signature_sender::SignatureSender;
use randomness_agent::{
    BLS12_381_COMPRESSED_RANDOMNESS_SCHEME_ID, BLS12_381_RANDOMNESS_SCHEME_ID,
    BN254_RANDOMNESS_SCHEME_ID, NotifyTicker, run_agent,
};
use std::time::Duration;
use superalloy::provider::create_provider_with_retry;
use superalloy::retry::RetryStrategy;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::Layer;
use tracing_subscriber::prelude::*;
use utils::serialize::point::{
    PointDeserializeCompressed, PointSerializeCompressed, PointSerializeUncompressed,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let RandomnessAgentConfig {
        mut config,
        bn254_nodes_config,
        bls12_381_nodes_config,
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

    struct ServiceComponents<P, BLS> {
        ticker: NotifyTicker,
        libp2p_node: Libp2pNode<u16>,
        ts_stopper: CancellationToken,
        stopper: OneshotStopper,
        agent: RandomnessAgent<BLS, P>,
    }

    // BN254 agent using uncompressed points
    let mut bn254 = {
        // Create a fulfiller
        let (ticker, libp2p_node, ts_stopper, stopper, channel) = create_threshold_fulfiller(
            &config,
            BlsPairingSigner::new_bn254(config.key_config.bn254_key.0),
            SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bn254G1,
                hash: BlsSignatureHash::Keccak256,
                compression: false,
            }),
            &bn254_nodes_config.unwrap_or_default(),
            signature_sender_contract.clone(),
            randomness_sender_contract.clone(),
        )?;

        // Create a new randomness agent
        let agent = RandomnessAgent::new(
            BN254_RANDOMNESS_SCHEME_ID,
            config.chain.sync_batch_size,
            channel,
            signature_sender_contract_ro.clone(),
        );
        ServiceComponents {
            ticker,
            libp2p_node,
            ts_stopper,
            stopper,
            agent,
        }
    };

    // BLS12-381 agent using uncompressed points
    let mut bls12_381 = {
        // Create a fulfiller
        let (ticker, libp2p_node, ts_stopper, stopper, channel) = create_threshold_fulfiller(
            &config,
            BlsPairingSigner::new_bls12_381(config.key_config.bls12_381_key.0),
            SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bls12_381G1,
                hash: BlsSignatureHash::Sha256,
                compression: false,
            }),
            &bls12_381_nodes_config.clone().unwrap_or_default(),
            signature_sender_contract.clone(),
            randomness_sender_contract.clone(),
        )?;

        // Create a new randomness agent
        let agent = RandomnessAgent::new(
            BLS12_381_RANDOMNESS_SCHEME_ID,
            config.chain.sync_batch_size,
            channel,
            signature_sender_contract_ro.clone(),
        );
        ServiceComponents {
            ticker,
            libp2p_node,
            ts_stopper,
            stopper,
            agent,
        }
    };

    // BLS12-381 agent using compressed points
    let mut bls12_381_c = {
        // Create a fulfiller
        let (ticker, libp2p_node, ts_stopper, stopper, channel) = create_threshold_fulfiller(
            &config,
            BlsPairingSigner::new_bls12_381(config.key_config.bls12_381_key.0),
            SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bls12_381G1,
                hash: BlsSignatureHash::Sha256,
                compression: true,
            }),
            &bls12_381_nodes_config.unwrap_or_default(),
            signature_sender_contract.clone(),
            randomness_sender_contract.clone(),
        )?;

        // Create a new randomness agent
        let agent = RandomnessAgent::new(
            BLS12_381_COMPRESSED_RANDOMNESS_SCHEME_ID,
            config.chain.sync_batch_size,
            channel,
            signature_sender_contract_ro.clone(),
        );
        ServiceComponents {
            ticker,
            libp2p_node,
            ts_stopper,
            stopper,
            agent,
        }
    };

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

        err = run_agent(&mut bn254.agent, bn254.ticker, signature_sender_contract_ro.clone()) => {
            eprintln!("bn254 agent stopped unexpectedly...");
            err // return Result
        },

        err = run_agent(&mut bls12_381.agent, bls12_381.ticker, signature_sender_contract_ro.clone()) => {
            eprintln!("bls12_381 agent stopped unexpectedly...");
            err // return Result
        },

        err = run_agent(&mut bls12_381_c.agent, bls12_381_c.ticker, signature_sender_contract_ro.clone()) => {
            eprintln!("bls12_381_c agent stopped unexpectedly...");
            err // return Result
        },

        err = start_api(config.healthcheck_listen_addr, config.healthcheck_port) => {
            eprintln!("healthcheck stopped unexpectedly...");
            err // return Result
        }
    };

    // Stop the various components
    if let Err(e) = bn254.libp2p_node.stop().await {
        tracing::error!(error = ?e, "Failed to stop bn254 libp2p node");
    }
    bn254.ts_stopper.cancel();
    bn254.stopper.stop().await;

    if let Err(e) = bls12_381.libp2p_node.stop().await {
        tracing::error!(error = ?e, "Failed to stop bls12_381 libp2p node");
    }
    bls12_381.ts_stopper.cancel();
    bls12_381.stopper.stop().await;

    if let Err(e) = bls12_381_c.libp2p_node.stop().await {
        tracing::error!(error = ?e, "Failed to stop bls12_381_c libp2p node");
    }
    bls12_381_c.ts_stopper.cancel();
    bls12_381_c.stopper.stop().await;

    res
}

fn create_threshold_fulfiller<'lt_in, 'lt_out, P, BLS>(
    args: &'lt_in RandomnessAgentArgs,
    signer: BLS,
    algorithm: SignatureAlgorithm,
    nodes_config: &'lt_in NodesConfiguration<<BLS::E as Pairing>::G2Affine>,
    signature_sender_contract: SignatureSender::SignatureSenderInstance<P>,
    randomness_sender_contract: RandomnessSender::RandomnessSenderInstance<P>,
) -> anyhow::Result<(
    NotifyTicker,
    Libp2pNode<u16>,
    CancellationToken,
    OneshotStopper,
    impl RequestChannel<Request = SignatureRequest> + 'lt_out,
)>
where
    P: Provider + WalletProvider + Clone + 'static,
    BLS: BlsSigner + Clone + Send + Sync + 'static,
    <BLS::E as Pairing>::G1Affine:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
    <BLS::E as Pairing>::G2Affine:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
{
    // Get per-nodes config
    let (mut pks_g2, addresses, peer_ids, short_ids): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) =
        nodes_config
            .nodes
            .iter()
            .cloned()
            .map(|c| {
                (
                    (c.node_id.get(), c.bls_pk),
                    c.address,
                    c.peer_id,
                    c.node_id.get(),
                )
            })
            .collect();

    // Add own pk to the list if required
    if pks_g2.len() == usize::from(args.key_config.n.get() - 1) {
        let pk = signer.g2_public_key();
        pks_g2.push((args.key_config.node_id.get(), pk));
    }

    // Create a libp2p transport and start it
    let mut libp2p_node = Libp2pNodeConfig::new(
        args.libp2p.libp2p_key.clone().into(),
        args.key_config.node_id.get(),
        addresses,
        peer_ids,
        short_ids,
    )
    .run(args.libp2p.libp2p_listen_addr.clone())?;

    let signer = BlsThresholdSigner::new(
        signer,
        args.key_config.n.get(),
        args.key_config.t.get(),
        args.key_config.node_id.get(),
        Default::default(),
        pks_g2.into_iter().collect(),
    );
    let (ts_stopper, signer) = signer.run(
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
    let fulfiller = SignatureSenderFulfillerConfig::<<BLS::E as Pairing>::G1, _, _>::new_fulfiller(
        signer,
        algorithm,
        ApplicationArgs::Randomness(ApplicationRandomnessArgs {
            chain_id: args
                .chain
                .chain_id
                .expect("chain id must have been set at this point"),
        }),
        signature_tx_fulfiller,
        args.chain.max_tx_per_tick,
        args.chain.tx_retry_strategy,
    );

    let ticker = NotifyTicker::default();
    let (stopper, channel) = fulfiller.run(ticker.clone());
    Ok((ticker, libp2p_node, ts_stopper, stopper, channel))
}
