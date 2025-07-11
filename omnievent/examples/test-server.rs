use alloy::network::Ethereum;
use alloy::node_bindings::Anvil;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use clap::Parser;
use omnievent::event_manager::EventManager;
use omnievent::event_manager::db::sql::sqlite::SqliteEventDatabase;
use omnievent::grpc::OmniEventServiceImpl;
use omnievent::proto_types::omni_event_service_server::OmniEventServiceServer;
use std::net::IpAddr;
use std::sync::Arc;
use superalloy::provider::MultiProvider;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser, Debug)]
#[command(about = "omnievent listener server")]
pub struct Config {
    /// Database connection string
    #[arg(short = 'd', long = "database", default_value = "sqlite::memory:")]
    pub database: String,

    /// IP address to listen on
    #[arg(short = 'l', long = "listen", default_value = "127.0.0.1")]
    pub listen_address: IpAddr,

    /// Port number to listen on
    #[arg(short = 'p', long = "port", default_value = "8080", value_parser = clap::value_parser!(u16).range(1..=65535))]
    pub port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Config {
        listen_address,
        port,
        database,
    } = Config::parse();

    let anvil_chain_1337 = Anvil::new()
        .chain_id(1337)
        .port(1337u16)
        .args(vec!["--mnemonic-seed-unsafe", "0"])
        .spawn();
    let anvil_chain_1338 = Anvil::new()
        .chain_id(1338)
        .port(1338u16)
        .args(vec!["--mnemonic-seed-unsafe", "0"])
        .spawn();

    tracing::info!("anvil 1337: {}", anvil_chain_1337.endpoint());
    tracing::info!("anvil 1338: {}", anvil_chain_1338.endpoint());
    tracing::info!(
        "funded wallet: 0x836fd4eecd5fc23eb480581cf91f638b5dacfa6ffa3a931b1f0421a5d58cfa5a"
    );

    let wallet_1337 = anvil_chain_1337
        .wallet()
        .expect("anvil should have a wallet");
    let wallet_1338 = anvil_chain_1338
        .wallet()
        .expect("anvil should have a wallet");

    let provider_1337 = ProviderBuilder::new()
        .with_gas_estimation()
        .wallet(wallet_1337)
        .connect_ws(WsConnect::new(anvil_chain_1337.ws_endpoint()))
        .await?
        .erased();

    let provider_1338 = ProviderBuilder::new()
        .with_gas_estimation()
        .wallet(wallet_1338)
        .connect_ws(WsConnect::new(anvil_chain_1338.ws_endpoint()))
        .await?
        .erased();

    let emitter_1337 = test_contracts::deploy_event_emitter(provider_1337.clone()).await;
    let emitter_1338 = test_contracts::deploy_event_emitter(provider_1338.clone()).await;
    assert_eq!(emitter_1337.address(), emitter_1338.address());
    tracing::info!("emitter contract deployed at {}", emitter_1337.address());

    // Create a multi provider
    let mut multi_provider = MultiProvider::empty();
    multi_provider.extend::<Ethereum>([(1337, provider_1337), (1338, provider_1338)]);

    // Start event manager
    let db = SqliteEventDatabase::connect(&database).await?;
    db.maybe_initialize_schema().await?;
    let mut event_manager = EventManager::new(Arc::new(multi_provider), db);
    event_manager.start();

    let omnievent = OmniEventServiceImpl::new(Arc::new(event_manager));
    tracing::info!("omnievent service listening on {listen_address}:{port}");
    tonic::transport::Server::builder()
        .add_service(OmniEventServiceServer::new(omnievent))
        .serve((listen_address, port).into())
        .await?;

    Ok(())
}

pub(crate) mod test_contracts {
    use crate::test_contracts::EventEmitter::EventEmitterInstance;
    use alloy::network::Network;
    use alloy::providers::Provider;

    alloy::sol! {
        #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506102b88061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c80632536f1271461003857806385986fbf14610054575b5f5ffd5b610052600480360381019061004d919061015d565b610070565b005b61006e600480360381019061006991906101db565b6100ad565b005b7f500918a1acf84fe22df8e73c039449df2f37619cf220d2a4d382cddec5e088e182826040516100a1929190610260565b60405180910390a15050565b803373ffffffffffffffffffffffffffffffffffffffff167f4b90d6788928d63c1821907a6a8b95f40d26562d8fe41b105f7489db9966dfcb60405160405180910390a350565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261011d5761011c6100fc565b5b8235905067ffffffffffffffff81111561013a57610139610100565b5b60208301915083600182028301111561015657610155610104565b5b9250929050565b5f5f60208385031215610173576101726100f4565b5b5f83013567ffffffffffffffff8111156101905761018f6100f8565b5b61019c85828601610108565b92509250509250929050565b5f819050919050565b6101ba816101a8565b81146101c4575f5ffd5b50565b5f813590506101d5816101b1565b92915050565b5f602082840312156101f0576101ef6100f4565b5b5f6101fd848285016101c7565b91505092915050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f61023f8385610206565b935061024c838584610216565b61025583610224565b840190509392505050565b5f6020820190508181035f830152610279818486610234565b9050939250505056fea264697066735822122072d465f321fd429d946ed082399488934996cb85bb7d84ada834d631d02511b064736f6c634300081e0033")]
        contract EventEmitter {
            event StringEmitted(string value);
            event Subscribed(address indexed subscriber, uint256 indexed subId);

            function emitString(string calldata _value) external {
                emit StringEmitted(_value);
            }

            function emitSubscribed(uint256 calldata _sub_id) external {
                emit Subscribed(msg.sender, _sub_id);
            }
        }
    }

    pub async fn deploy_event_emitter<P, N>(provider: P) -> EventEmitterInstance<P, N>
    where
        P: Provider<N>,
        N: Network,
    {
        EventEmitterInstance::deploy(provider).await.unwrap()
    }
}
