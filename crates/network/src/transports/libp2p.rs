//! Libp2p node that can be used to broadcast and receive arbitrary messages using floodsub and a
//! peer whitelist.

mod dialer;
mod events_handler;
pub mod metrics;
mod point_to_point;
mod transport;

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
pub struct Libp2pNodeConfig {
    key: Keypair,
    peers: PeerDetails,
    redial_interval: Duration,
}

/// A libp2p node actively running in a background task.
pub struct Libp2pNode {
    events_handler_handle: JoinHandle<Result<(), Libp2pNodeError>>,
    transport: Option<Libp2pTransport<u16>>,
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

impl Libp2pNodeConfig {
    /// Create a new libp2p node.
    pub fn new(
        key: Keypair,
        peer_addrs: Vec<Multiaddr>,
        peer_ids: Vec<PeerId>,
        peer_short_ids: Vec<u16>,
    ) -> Self {
        if peer_addrs.len() != peer_ids.len() || peer_addrs.len() != peer_short_ids.len() {
            panic!("run requires all inputs array to be of equal length");
        }

        let peers = FromIterator::from_iter(izip!(peer_ids, peer_addrs, peer_short_ids).map(
            |(peer_id, multiaddr, short_id)| PeerDetail::new(peer_id, short_id, vec![multiaddr]),
        ));

        Self {
            key,
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
    pub fn run(self, listen_addr: Multiaddr) -> Result<Libp2pNode, Libp2pNodeError> {
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
                tracing::error!(error = ?e, peer_id = %p.peer_id, short_id = p.short_id, multiaddrs = ?p.multiaddrs, "Failed to dial peer at given multiaddresses")
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

impl Libp2pNode {
    pub fn get_transport(&mut self) -> Option<Libp2pTransport<u16>> {
        self.transport.take()
    }

    pub async fn stop(self) -> Result<(), Libp2pNodeError> {
        self.cancel.cancel();
        self.events_handler_handle.await?
    }
}

/// Configure a libp2p swarm by setting up the keypair, various layers and the behaviour
fn configure_swarm(
    keypair: Keypair,
    peers: impl IntoIterator<Item = PeerDetail>,
    redial_interval: Duration,
) -> Result<Swarm<Behaviour>, Libp2pNodeError> {
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
struct PeerDetail {
    short_id: u16,
    peer_id: PeerId,
    multiaddrs: Vec<Multiaddr>,
}

impl PeerDetail {
    fn new(peer_id: PeerId, short_id: u16, multiaddrs: Vec<Multiaddr>) -> Self {
        Self {
            short_id,
            peer_id,
            multiaddrs,
        }
    }
}

struct PeerDetails {
    from_peer_id: HashMap<PeerId, PeerDetail>,
    short_id_to_peer_id: HashMap<u16, PeerId>,
}

impl FromIterator<PeerDetail> for PeerDetails {
    fn from_iter<I>(values: I) -> Self
    where
        I: IntoIterator<Item = PeerDetail>,
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

impl PeerDetails {
    fn values(&self) -> impl Iterator<Item = &PeerDetail> {
        self.from_peer_id.values()
    }

    fn get(&self, peer_id: &PeerId) -> Option<&PeerDetail> {
        self.from_peer_id.get(peer_id)
    }

    fn get_short_id(&self, peer_id: &PeerId) -> Option<u16> {
        self.get(peer_id).map(|p| p.short_id).or_else(|| {
            tracing::error!(
                sender_peer_id = %peer_id,
                "Failed to convert peer_id to short_id"
            );
            None
        })
    }

    fn get_peer_id(&self, short_id: &u16) -> Option<&PeerId> {
        self.short_id_to_peer_id.get(short_id)
    }
}

/// Libp2p Behaviour with floodsub and a peer whitelist.
#[derive(NetworkBehaviour)]
struct Behaviour {
    allowed_peers: allow_block_list::Behaviour<AllowedPeers>,
    floodsub: floodsub::Floodsub,
    point_to_point: request_response::Behaviour<DcipherPoint2PointMessageCodec>,
    ping: ping::Behaviour,
    periodic_dial: PeriodicDialBehaviour,
}

impl Behaviour {
    /// Create a new behaviour
    fn new(
        local_peer_id: PeerId,
        peers: impl IntoIterator<Item = PeerDetail>,
        redial_interval: Duration,
    ) -> Self {
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
