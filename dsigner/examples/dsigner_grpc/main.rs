use crate::arguments_parser::{BlsSchemeConfig, DSignerConfig, NetworkConfig, SchemeConfigType};
use anyhow::anyhow;
use ark_ec::pairing::Pairing;
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::topic::dispatcher::{TopicBasedTransportImpl, TopicDispatcher};
use dcipher_network::transports::libp2p::transport::Libp2pSender;
use dcipher_network::transports::libp2p::{Libp2pNode, Libp2pNodeConfig};
use dcipher_signer::bls::{BlsPairingSigner, BlsSigner, BlsThresholdSigner, BlsVerifier};
use dcipher_signer::dsigner::DSignerScheme;
use dsigner::proto_types::d_signer_service_server::DSignerServiceServer;
use dsigner::server::DSignerSchemeManager;
use dsigner::server::grpc::DSignerServiceImpl;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::dst::NamedCurveGroup;
use utils::hash_to_curve::CustomHashToCurve;

mod arguments_parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let DSignerConfig {
        config,
        network_config,
        schemes_config,
    } = DSignerConfig::parse()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    if network_config.id != schemes_config.node_id {
        return Err(anyhow!(
            "the network id does not match the schemes config id"
        ));
    }

    let transport = get_libp2p_transports(&network_config)?;

    let mut manager = DSignerSchemeManager::new();
    for (scheme_id, scheme) in schemes_config.schemes {
        let transport = transport
            .topic_transport
            .get_transport_for(scheme_id.clone())
            .ok_or(anyhow!("failed to get transport for scheme"))?;
        let dsigner_scheme: Arc<dyn DSignerScheme + Send + Sync + 'static> = match scheme {
            SchemeConfigType::Bn254(bls) => {
                let signer = get_bls_signer(schemes_config.node_id.get(), bls);
                let (_, dsigner_scheme) = signer.run(transport);
                Arc::new(dsigner_scheme)
            }
            SchemeConfigType::Bls12_381(bls) => {
                let signer = get_bls_signer(schemes_config.node_id.get(), bls);
                let (_, dsigner_scheme) = signer.run(transport);
                Arc::new(dsigner_scheme)
            }
        };

        manager.register_scheme_mut(scheme_id.into(), dsigner_scheme);
    }

    let dsigner_service = DSignerServiceImpl::new(Arc::new(manager));
    tonic::transport::Server::builder()
        .add_service(DSignerServiceServer::new(dsigner_service))
        .serve((IpAddr::from_str("127.0.0.1")?, 8090).into())
        .await?;

    tracing::info!("Stopping libp2p dispatcher...");
    transport.topic_dispatcher.stop().await;

    tracing::info!("Stopping libp2p transport...");
    if let Err(e) = transport.node.stop().await {
        tracing::error!(error = ?e, "Failed to stop libp2p node");
    }

    Ok(())
}

fn get_bls_signer<E>(
    node_id: u16,
    bls_config: BlsSchemeConfig<E>,
) -> BlsThresholdSigner<BlsPairingSigner<E>>
where
    E: Pairing,
    E::G1: CustomHashToCurve + NamedCurveGroup,
    E::G2: CustomHashToCurve + NamedCurveGroup,
    BlsPairingSigner<E>: BlsSigner + BlsVerifier<E = E>,
{
    let (pks_g1, pks_g2) = bls_config
        .nodes
        .into_iter()
        .map(|n| ((n.id.get(), n.pk_g1), (n.id.get(), n.pk_g2)))
        .collect();

    let signer = BlsPairingSigner::new(bls_config.sk.0);
    BlsThresholdSigner::new(
        signer,
        bls_config.n.get(),
        bls_config.t.get(),
        node_id,
        pks_g1,
        pks_g2,
    )
}

type TopicTransport = TopicBasedTransportImpl<Libp2pSender<u16>>;

struct Libp2pTransports {
    node: Libp2pNode<u16>,
    topic_dispatcher: TopicDispatcher,
    topic_transport: Arc<TopicTransport>,
}

fn get_libp2p_transports(network_config: &NetworkConfig) -> anyhow::Result<Libp2pTransports> {
    // Make sure that the identifiers are unique
    let (peer_addrs, peer_ids, short_ids): (Vec<_>, Vec<_>, Vec<_>) = network_config
        .peers
        .iter()
        .map(|p| (p.multiaddr.clone(), p.peer_id, p.id.get()))
        .collect();

    let mut node = Libp2pNodeConfig::new(
        network_config.libp2p_key.0.clone(),
        network_config.id.get(),
        peer_addrs,
        peer_ids,
        short_ids,
    )
    .run(network_config.libp2p_listen_addr.clone())
    .map_err(|e| {
        tracing::error!("Failed to start libp2p network: {e:?}");
        e
    })?;

    // Start libp2p transport
    tracing::info!("Starting libp2p networking");
    let transport = node
        .get_transport()
        .ok_or(anyhow!("failed to get topic transport"))?;

    let mut topic_dispatcher = TopicDispatcher::new();
    let topic_transport = topic_dispatcher.start(transport).into();

    Ok(Libp2pTransports {
        node,
        topic_transport,
        topic_dispatcher,
    })
}
