mod app;
mod config;
mod executor;
mod fee_adapter;
pub(crate) mod gasless;
mod model;
mod network;
pub mod price_feed;
mod profitability;
mod setup;
mod solver;
mod util;

use crate::app::{App, OmniEventBoxService};
use crate::config::{AppConfig, CliArgs, Command};
use crate::network::Network;
use crate::setup::setup_allowances;
use ::config::file::load_config_file;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use alloy::network::Ethereum;
use alloy::providers::DynProvider;
use alloy::signers::local::PrivateKeySigner;
use anyhow::anyhow;
use clap::Parser;
use dotenv::dotenv;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::NopDatabase;
use omnievent::grpc::OmniEventServiceImpl;
use omnievent::proto_types::omni_event_service_server::OmniEventServiceServer;
use onlyswaps_client::client::OnlySwapsClient;
use onlyswaps_client::config::OnlySwapsClientConfig;
use onlyswaps_client::config::chain::ChainConfig;
use onlyswaps_client::config::token::TokenTag;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use superalloy::provider::MultiProvider;
use tonic::transport::Endpoint;
use tower::ServiceExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = CliArgs::parse();
    let command = cli.command();
    let config: AppConfig = load_config_file(cli.config_path)?;
    let private_key_signer: PrivateKeySigner = cli.private_key.parse()?;
    let networks = Network::create_many(&cli.private_key, &config.networks).await?;

    match command {
        Command::Run => run(config, private_key_signer, networks).await,
        Command::Setup => setup(networks).await,
    }
}

async fn run(
    config: AppConfig,
    private_key_signer: PrivateKeySigner,
    networks: HashMap<u64, Network<DynProvider>>,
) -> anyhow::Result<()> {
    let healthcheck_server = HealthcheckServer::new(
        config.agent.healthcheck_listen_addr,
        config.agent.healthcheck_port,
    )
    .await?;
    init_monitoring(&config.agent)?;

    let (service, maybe_manager) =
        get_omnievent_service(config.omnievent.endpoint.clone(), &networks).await?;

    let client = create_onlyswaps_client(&config, &networks);

    // start some healthcheck and signal handlers
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    // listen for alllll the things!
    let out = tokio::select! {
        res = App::start(private_key_signer, client, networks, &config.timeout, &config.profitability, service) => {
            match res {
                Ok(_) => Err(anyhow!("event listener stopped unexpectedly")),
                Err(e) => Err(anyhow!("event listener stopped unexpectedly: {}", e))
            }
        }

        res = healthcheck_server.start() => {
            match res {
                Ok(_) => Err(anyhow!("http server stopped unexpectedly")),
                Err(e) => Err(anyhow!("http server stopped unexpectedly: {}", e))
            }
        }

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
    };

    if let Some(arc_manager) = maybe_manager
        && let Some(manager) = Arc::into_inner(arc_manager)
    {
        // ignore stop errors
        match tokio::time::timeout(std::time::Duration::from_secs(1), manager.stop()).await {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => tracing::error!(error = ?e, "Error while stopping omnievent manager"),
            Err(_) => tracing::error!("Failed to stop omnievent within 1s timeout"),
        }
    }

    out
}

async fn setup(networks: HashMap<u64, Network<DynProvider>>) -> anyhow::Result<()> {
    setup_allowances(&networks).await
}

type ArcManager = Arc<EventManager<MultiProvider<u64>, NopDatabase>>;

/// Create an omnievent service either by relying on an external endpoint, or by initialising our own
/// omnievent local service.
async fn get_omnievent_service(
    maybe_endpoint: Option<
        impl TryInto<Endpoint, Error: std::error::Error + Send + Sync + 'static>,
    >,
    networks: &HashMap<u64, Network<DynProvider>>,
) -> anyhow::Result<(OmniEventBoxService, Option<ArcManager>)> {
    if let Some(endpoint) = maybe_endpoint {
        // endpoint specified, we connect to an existing omnievent service
        let endpoint: Endpoint = endpoint.try_into()?;
        let service = endpoint.connect().await?;

        return Ok((service.map_err(Into::into).boxed(), None));
    }

    // No endpoint specified, start our own omnievent service
    let mut multi_provider = MultiProvider::empty();
    multi_provider.extend::<Ethereum>(
        networks
            .iter()
            .map(|(&chain_id, net)| (chain_id, net.provider.clone())),
    );

    let mut event_manager = EventManager::new(multi_provider, NopDatabase); // no need to store events
    event_manager.start();

    let event_manager = Arc::new(event_manager);
    let omnievent_service = OmniEventServiceImpl::new(event_manager.clone());
    let service = ServiceExt::<axum::http::Request<tonic::body::Body>>::map_err(
        OmniEventServiceServer::new(omnievent_service),
        Into::into,
    )
    .boxed();

    Ok((service, Some(event_manager)))
}

fn create_onlyswaps_client(
    config: &AppConfig,
    networks: &HashMap<u64, Network<DynProvider>>,
) -> OnlySwapsClient {
    let mut only_config = OnlySwapsClientConfig::empty();
    for (chain_id, net) in networks {
        let tokens = HashMap::from_iter(net.tokens.iter().map(|t| {
            // Just tag the token with its address
            let name = Cow::Owned(t.address().to_string());
            (TokenTag::Other(name), *t.address())
        }));

        let chain = ChainConfig::new(
            *chain_id,
            *net.router.address(),
            tokens,
            config.timeout.request_timeout,
            1,
        );
        only_config.add_ethereum_chain_dyn(chain, net.provider.clone(), Some(net.own_addr));
    }

    OnlySwapsClient::new(only_config)
}
