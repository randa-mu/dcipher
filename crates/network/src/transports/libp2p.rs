//! Libp2p node that can be used to broadcast and receive arbitrary messages using floodsub and a
//! peer whitelist.

mod dialer;
mod events_handler;
pub mod metrics;
mod point_to_point;
pub mod transport;

use crate::PartyIdentifier;
use crate::transports::libp2p::dialer::PeriodicDialBehaviour;
use crate::transports::libp2p::events_handler::EventsHandler;
use crate::transports::libp2p::point_to_point::{
    DcipherPoint2PointMessageCodec, POINT_TO_POINT_PROTOCOL,
};
use crate::transports::libp2p::transport::Libp2pTransport;
use itertools::izip;
use libp2p::allow_block_list::AllowedPeers;
use libp2p::identity::Keypair;
use libp2p::swarm::NetworkBehaviour;
use libp2p::swarm::dial_opts::{DialOpts, PeerCondition};
use libp2p::{
    Multiaddr, PeerId, Swarm, allow_block_list, floodsub, noise, ping, request_response, tcp, yamux,
};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

const LIBP2P_MAIN_TOPIC: &str = "main";
const DEFAULT_REDIAL_INTERVAL: Duration = Duration::from_secs(2 * 60); // 2mins

/// Holds configuration parameters and obtain a [`Libp2pNode`] by running
/// [`Self::run`](Libp2pNodeConfig::run).
pub struct Libp2pNodeConfig<ID> {
    key: Keypair,
    short_id: ID,
    peers: PeerDetails<ID>,
    redial_interval: Duration,
}

/// A libp2p node actively running in a background task.
pub struct Libp2pNode<ID>
where
    ID: PartyIdentifier,
{
    events_handler_handle: JoinHandle<Result<(), Libp2pNodeError>>,
    transport: Option<Libp2pTransport<ID>>,
    cancel: CancellationToken,
}

#[derive(Error, Debug)]
pub enum Libp2pNodeError {
    #[error("libp2p transport sender has been dropped")]
    SenderDropped,

    #[error("invalid multiaddr")]
    MultiAddr(#[from] libp2p::multiaddr::Error),

    #[error("libp2p transport error: {1}")]
    TransportError(
        #[source] Box<dyn std::error::Error + Send + Sync>,
        &'static str,
    ),

    #[error("libp2p noise protocol error")]
    Noise(#[from] noise::Error),

    #[error("failed to join background task")]
    Join(#[from] tokio::task::JoinError),
}

impl<ID: PartyIdentifier> Libp2pNodeConfig<ID> {
    /// Create a new libp2p node.
    pub fn new(
        key: Keypair,
        short_id: ID,
        peer_addrs: Vec<Multiaddr>,
        peer_ids: Vec<PeerId>,
        peer_short_ids: Vec<ID>,
    ) -> Self {
        if peer_addrs.len() != peer_ids.len() || peer_addrs.len() != peer_short_ids.len() {
            panic!("run requires all inputs array to be of equal length");
        }

        // Build the peers, removing oneself if necessary
        let peers =
            FromIterator::from_iter(izip!(peer_ids, peer_addrs, peer_short_ids).filter_map(
                |(peer_id, multiaddr, peer_short_id)| {
                    if short_id == peer_short_id {
                        None
                    } else {
                        Some(PeerDetail::new(peer_id, peer_short_id, vec![multiaddr]))
                    }
                },
            ));

        Self {
            key,
            short_id,
            peers,
            redial_interval: DEFAULT_REDIAL_INTERVAL,
        }
    }

    /// Set a custom redial interval
    pub fn redial_interval(&mut self, redial_interval: Duration) -> &mut Self {
        self.redial_interval = redial_interval;
        self
    }

    /// Runs a new libp2p node that listens on `listen_addr`, forwards messages from the swarm to `tx_received_messages`,
    /// and broadcasts messages from `rx_messages_to_send` to the swarm.
    pub fn run(self, listen_addr: Multiaddr) -> Result<Libp2pNode<ID>, Libp2pNodeError> {
        // Create a new swarm
        let mut swarm = configure_swarm(
            self.key.clone(),
            self.peers.values().cloned(),
            self.redial_interval,
        )?;

        // Listen on all interfaces
        swarm
            .listen_on(listen_addr)
            .map_err(|e| Libp2pNodeError::TransportError(e.into(), "failed to start listener"))?;

        // Register each of the peers
        self.peers.values().for_each(|p| {
            p.multiaddrs
                .iter()
                .cloned()
                .for_each(|multiaddr| swarm.add_peer_address(p.peer_id, multiaddr));

            swarm
                .behaviour_mut()
                .floodsub
                .add_node_to_partial_view(p.peer_id);

            let dial_opts = DialOpts::peer_id(p.peer_id)
                .addresses(p.multiaddrs.clone())
                .condition(PeerCondition::Always)
                .build();
            if let Err(e) = swarm.dial(dial_opts) {
                tracing::error!(error = ?e, peer_id = %p.peer_id, short_id = ?p.short_id, multiaddrs = ?p.multiaddrs, "Failed to dial peer at given multiaddresses")
            }
        });

        // Create a floodsub topic and subscribe
        let topic = floodsub::Topic::new(LIBP2P_MAIN_TOPIC);
        let _ = swarm.behaviour_mut().floodsub.subscribe(topic);

        // Create channels for sending and receiving
        let (tx_received_message, rx_received_message) = unbounded_channel();
        let (tx_msg_to_send, rx_msg_to_send) = unbounded_channel();

        // Process swarm events in a separate task
        let cancellation_token = CancellationToken::new();
        let events_handler_handle = tokio::spawn(
            EventsHandler::new(
                self.short_id,
                swarm,
                self.peers,
                tx_received_message,
                rx_msg_to_send,
                cancellation_token.clone(),
            )
            .run(),
        );

        Ok(Libp2pNode {
            events_handler_handle,
            cancel: cancellation_token,
            transport: Libp2pTransport::new(rx_received_message, tx_msg_to_send).into(),
        })
    }
}

impl<ID: PartyIdentifier> Libp2pNode<ID> {
    pub fn get_transport(&mut self) -> Option<Libp2pTransport<ID>> {
        self.transport.take()
    }

    pub async fn stop(self) -> Result<(), Libp2pNodeError> {
        self.cancel.cancel();
        self.events_handler_handle.await?
    }
}

/// Configure a libp2p swarm by setting up the keypair, various layers and the behaviour
fn configure_swarm<ID: PartyIdentifier>(
    keypair: Keypair,
    peers: impl IntoIterator<Item = PeerDetail<ID>>,
    redial_interval: Duration,
) -> Result<Swarm<Behaviour<ID>>, Libp2pNodeError> {
    let peer_id = keypair.public().to_peer_id();
    Ok(libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_dns()
        .expect("failed to create swarm with dns")
        .with_behaviour(|_| Behaviour::new(peer_id, peers, redial_interval))
        .unwrap() // infallible
        .with_swarm_config(|cfg| {
            cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)) // stay connected to the peer even if idle
        })
        .build())
}

#[derive(Clone, Debug)]
struct PeerDetail<ID> {
    short_id: ID,
    peer_id: PeerId,
    multiaddrs: Vec<Multiaddr>,
}

impl<ID> PeerDetail<ID> {
    fn new(peer_id: PeerId, short_id: ID, multiaddrs: Vec<Multiaddr>) -> Self {
        Self {
            short_id,
            peer_id,
            multiaddrs,
        }
    }
}

struct PeerDetails<ID> {
    from_peer_id: HashMap<PeerId, PeerDetail<ID>>,
    short_id_to_peer_id: HashMap<ID, PeerId>,
}

impl<ID: PartyIdentifier> FromIterator<PeerDetail<ID>> for PeerDetails<ID> {
    fn from_iter<I>(values: I) -> Self
    where
        I: IntoIterator<Item = PeerDetail<ID>>,
    {
        let from_peer_id = HashMap::from_iter(values.into_iter().map(|d| (d.peer_id, d)));
        let short_id_to_peer_id =
            HashMap::from_iter(from_peer_id.values().map(|d| (d.short_id, d.peer_id)));

        Self {
            from_peer_id,
            short_id_to_peer_id,
        }
    }
}

impl<ID: PartyIdentifier> PeerDetails<ID> {
    fn values(&self) -> impl Iterator<Item = &PeerDetail<ID>> {
        self.from_peer_id.values()
    }

    fn get(&self, peer_id: &PeerId) -> Option<&PeerDetail<ID>> {
        self.from_peer_id.get(peer_id)
    }

    fn get_short_id(&self, peer_id: &PeerId) -> Option<ID> {
        self.get(peer_id).map(|p| p.short_id).or_else(|| {
            tracing::error!(
                sender_peer_id = %peer_id,
                "Failed to convert peer_id to short_id"
            );
            None
        })
    }

    fn get_peer_id(&self, short_id: &ID) -> Option<&PeerId> {
        self.short_id_to_peer_id.get(short_id)
    }
}

/// Libp2p Behaviour with floodsub and a peer whitelist.
#[derive(NetworkBehaviour)]
struct Behaviour<ID> {
    allowed_peers: allow_block_list::Behaviour<AllowedPeers>,
    floodsub: floodsub::Floodsub,
    point_to_point: request_response::Behaviour<DcipherPoint2PointMessageCodec>,
    ping: ping::Behaviour,
    periodic_dial: PeriodicDialBehaviour<ID>,
}

impl<ID: PartyIdentifier> Behaviour<ID> {
    /// Create a new behaviour
    fn new<I>(local_peer_id: PeerId, peers: I, redial_interval: Duration) -> Self
    where
        I: IntoIterator<Item = PeerDetail<ID>>,
    {
        let peers = peers.into_iter().collect::<Vec<_>>();

        // Build a list of allowed peers
        let allowed_peers = {
            let mut allowed_peers = allow_block_list::Behaviour::default();
            peers.iter().for_each(|p| {
                allowed_peers.allow_peer(p.peer_id);
            });
            allowed_peers
        };

        let point_to_point = request_response::Behaviour::new(
            [(
                POINT_TO_POINT_PROTOCOL,
                request_response::ProtocolSupport::Full,
            )],
            request_response::Config::default(),
        );

        Self {
            allowed_peers,
            floodsub: floodsub::Floodsub::new(local_peer_id),
            point_to_point,
            ping: ping::Behaviour::default(),
            periodic_dial: PeriodicDialBehaviour::new(redial_interval, peers),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{MessageType, ReceivedMessage, Transport, TransportSender};
    use futures_util::StreamExt;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    pub(crate) async fn start_nodes<ID: PartyIdentifier>(
        short_ids: &[ID],
        start_port: u16,
    ) -> Vec<Libp2pNode<ID>> {
        let n = short_ids.len().try_into().expect("too many peers");

        let libp2p_sks = short_ids
            .iter()
            .map(|_| Keypair::generate_ed25519())
            .collect::<Vec<_>>();
        let libp2p_peer_ids = libp2p_sks
            .iter()
            .map(|sk| sk.public().to_peer_id())
            .collect::<Vec<_>>();

        let libp2p_addrs = (0..n)
            .map(|i| {
                format!("/ip4/127.0.0.1/tcp/{}", start_port + i)
                    .parse()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        izip!(short_ids.iter(), libp2p_addrs.iter(), libp2p_sks)
            .map(|(i, listen_addr, libp2p_sk)| {
                Libp2pNodeConfig::new(
                    libp2p_sk,
                    *i,
                    libp2p_addrs.clone(),
                    libp2p_peer_ids.clone(),
                    short_ids.to_vec(),
                )
                .run(listen_addr.to_owned())
                .expect("failed to start node")
            })
            .collect()
    }

    #[tokio::test]
    async fn send_receive_direct_messages() {
        // Try to set logging options
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from("debug"))
            .with(tracing_subscriber::fmt::layer())
            .try_init();

        let global_timeout = Duration::from_millis(2000);

        let [mut node_1, mut node_2]: [Libp2pNode<_>; 2] = start_nodes(&[1u16, 2], 32200)
            .await
            .try_into()
            .unwrap_or_else(|_| panic!("failed to create node"));

        let mut transport_1 = node_1.get_transport().unwrap();
        let mut transport_2 = node_2.get_transport().unwrap();

        let mut rx_1 = transport_1.receiver_stream().unwrap();
        let mut rx_2 = transport_2.receiver_stream().unwrap();

        let tx_1 = Transport::sender(&mut transport_1).unwrap();
        let tx_2 = Transport::sender(&mut transport_2).unwrap();

        // Send one message from node 1 to node 2
        let m = b"sent to node 2";
        tx_1.send_single(m.to_vec(), 2)
            .await
            .expect("send to node 2 failed");
        let m2 = tokio::time::timeout(global_timeout, rx_2.next())
            .await
            .expect("failed to obtain signature: timed out")
            .expect("rx_2 stream closed")
            .expect("stream return err");
        assert_eq!(m2.sender, 1);
        assert_eq!(m2.message_type, MessageType::Direct);
        assert_eq!(m2.content, m.to_vec());

        // Send one message from node 2 to node 1
        let m = b"sent to node 1";
        tx_2.send_single(m.to_vec(), 1)
            .await
            .expect("send to node 1 failed");
        let m2 = tokio::time::timeout(global_timeout, rx_1.next())
            .await
            .expect("failed to obtain signature: timed out")
            .expect("rx_1 stream closed")
            .expect("stream return err");
        assert_eq!(m2.sender, 2);
        assert_eq!(m2.message_type, MessageType::Direct);
        assert_eq!(m2.content, m.to_vec());
    }

    #[tokio::test]
    async fn send_receive_broadcast_messages() {
        // Try to set logging options
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from("debug"))
            .with(tracing_subscriber::fmt::layer())
            .try_init();

        let global_timeout = Duration::from_millis(2000);

        let [mut node_1, mut node_2, mut node_3]: [Libp2pNode<_>; 3] =
            start_nodes(&[1, 2, 3], 32300)
                .await
                .try_into()
                .unwrap_or_else(|_| panic!("failed to create node"));

        let mut transport_1 = node_1.get_transport().unwrap();
        let mut transport_2 = node_2.get_transport().unwrap();
        let mut transport_3 = node_3.get_transport().unwrap();

        let mut rx_2 = transport_2.receiver_stream().unwrap();
        let mut rx_3 = transport_3.receiver_stream().unwrap();

        let tx_1 = transport_1.sender().unwrap();

        // Broadcast one message from node 1
        let m = b"broadcast from node 1";
        tx_1.broadcast(m.to_vec())
            .await
            .expect("send to node 2 failed");
        let m2: ReceivedMessage<_, Vec<u8>> = tokio::time::timeout(global_timeout, rx_2.next())
            .await
            .expect("failed to obtain signature: timed out")
            .expect("rx_2 stream closed")
            .expect("stream return err");
        assert_eq!(m2.sender, 1);
        assert_eq!(m2.message_type, MessageType::Broadcast);
        assert_eq!(m2.content, m.to_vec());

        let m3 = tokio::time::timeout(global_timeout, rx_3.next())
            .await
            .expect("failed to obtain signature: timed out")
            .expect("rx_2 stream closed")
            .expect("stream return err");
        assert_eq!(m3.sender, 1);
        assert_eq!(m3.message_type, MessageType::Broadcast);
        assert_eq!(m3.content, m.to_vec());
    }
}
