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
        #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b5061044e8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c80632536f1271461004357806385986fbf1461005f578063d104723a1461007b575b5f5ffd5b61005d600480360381019061005891906101da565b610097565b005b61007960048036038101906100749190610258565b6100d4565b005b61009560048036038101906100909190610310565b61011b565b005b7f500918a1acf84fe22df8e73c039449df2f37619cf220d2a4d382cddec5e088e182826040516100c89291906103ce565b60405180910390a15050565b803373ffffffffffffffffffffffffffffffffffffffff167f4b90d6788928d63c1821907a6a8b95f40d26562d8fe41b105f7489db9966dfcb60405160405180910390a350565b8173ffffffffffffffffffffffffffffffffffffffff1683857fc83fb1112417b2c6f38082f57a7cbc310fb31193b164c603669f3b691ba9a43e8460405161016391906103ff565b60405180910390a450505050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261019a57610199610179565b5b8235905067ffffffffffffffff8111156101b7576101b661017d565b5b6020830191508360018202830111156101d3576101d2610181565b5b9250929050565b5f5f602083850312156101f0576101ef610171565b5b5f83013567ffffffffffffffff81111561020d5761020c610175565b5b61021985828601610185565b92509250509250929050565b5f819050919050565b61023781610225565b8114610241575f5ffd5b50565b5f813590506102528161022e565b92915050565b5f6020828403121561026d5761026c610171565b5b5f61027a84828501610244565b91505092915050565b5f819050919050565b61029581610283565b811461029f575f5ffd5b50565b5f813590506102b08161028c565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6102df826102b6565b9050919050565b6102ef816102d5565b81146102f9575f5ffd5b50565b5f8135905061030a816102e6565b92915050565b5f5f5f5f6080858703121561032857610327610171565b5b5f610335878288016102a2565b945050602061034687828801610244565b9350506040610357878288016102fc565b925050606061036887828801610244565b91505092959194509250565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f6103ad8385610374565b93506103ba838584610384565b6103c383610392565b840190509392505050565b5f6020820190508181035f8301526103e78184866103a2565b90509392505050565b6103f981610225565b82525050565b5f6020820190506104125f8301846103f0565b9291505056fea26469706673582212208dc4b402f628b4084048e8996c4d09434ba374ec0f59d809d14844c1e9d1aaa264736f6c634300081e0033")]
        contract EventEmitter {
            event StringEmitted(string value);
            event Subscribed(address indexed subscriber, uint256 indexed subId);
            event BridgeReceipt(
                bytes32 indexed requestId, uint256 indexed srcChainId, address indexed solver, uint256 amountOut
            );

            function emitBridgeReceipt(bytes32 requestId, uint256 srcChainId, address solver, uint256 amountOut) external {
                emit BridgeReceipt(requestId, srcChainId, solver, amountOut);
            }

            function emitString(string calldata _value) external {
                emit StringEmitted(_value);
            }

            function emitSubscribed(uint256 _sub_id) external {
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
