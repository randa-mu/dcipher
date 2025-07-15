//! Libp2p node that can be used to broadcast and receive arbitrary messages using floodsub and a
//! peer whitelist.

mod dialer;
pub mod metrics;
mod point_to_point;
mod transport;

use crate::transports::SendMessage;
use crate::transports::libp2p::BehaviourEvent::Ping;
use crate::transports::libp2p::dialer::{PeriodicDialBehaviour, PeriodicDialEvent};
use crate::transports::libp2p::metrics::Metrics;
use crate::transports::libp2p::point_to_point::{
    DcipherPoint2PointMessageCodec, POINT_TO_POINT_PROTOCOL,
};
use crate::transports::libp2p::transport::Libp2pTransport;
use crate::{ReceivedMessage, Recipient};
use futures_util::StreamExt;
use itertools::izip;
use libp2p::allow_block_list::AllowedPeers;
use libp2p::floodsub::{FloodsubEvent, FloodsubMessage};
use libp2p::identity::Keypair;
use libp2p::request_response::{Event as RequestResponseEvent, Message as RequestResponseMessage};
use libp2p::swarm::NetworkBehaviour;
use libp2p::swarm::dial_opts::{DialOpts, PeerCondition};
use libp2p::{
    Multiaddr, PeerId, Swarm, allow_block_list, floodsub, noise, ping, request_response,
    swarm::SwarmEvent, tcp, yamux,
};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio_util::sync::CancellationToken;

const LIBP2P_MAIN_TOPIC: &str = "main";
const DEFAULT_REDIAL_INTERVAL: Duration = Duration::from_secs(2 * 60); // 2mins

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
}

#[derive(Clone, Debug)]
pub struct PeerDetail {
    short_id: u16,
    peer_id: PeerId,
    multiaddrs: Vec<Multiaddr>,
}

impl PeerDetail {
    pub fn new(peer_id: PeerId, short_id: u16, multiaddrs: Vec<Multiaddr>) -> Self {
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
        self.get(&peer_id).map(|p| p.short_id).or_else(|| {
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

pub struct Libp2pNode {
    key: Keypair,
    peers: PeerDetails,
    redial_interval: Duration,
}

impl Libp2pNode {
    /// Create a new libp2p node.
    pub fn new(
        key: Keypair,
        peer_addrs: Vec<Multiaddr>,
        peer_ids: Vec<PeerId>,
        peer_short_ids: Vec<u16>,
    ) -> Self {
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
    pub fn run(
        self,
        listen_addr: Multiaddr,
        cancellation_token: CancellationToken,
    ) -> Result<Libp2pTransport, Libp2pNodeError> {
        // Create a new swarm
        let mut swarm = Self::configure_swarm(
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
        tokio::spawn(Self::swarm_event_loop(
            swarm,
            self.peers,
            tx_received_message,
            rx_msg_to_send,
            cancellation_token,
        ));

        Ok(Libp2pTransport::new(rx_received_message, tx_msg_to_send))
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

    /// Main event loop handling incoming events from swarm, and incoming messages from the `rx_messages_to_send` channel.
    async fn swarm_event_loop(
        mut swarm: Swarm<Behaviour>,
        peers: PeerDetails,
        tx_received_messages: UnboundedSender<ReceivedMessage<u16>>,
        mut rx_messages_to_send: UnboundedReceiver<SendMessage<u16>>,
        cancellation_token: CancellationToken,
    ) {
        let inner_fn = async move {
            let mut ready_send_messages = false;
            loop {
                tokio::select! {
                    m = async {
                        if ready_send_messages {
                            // Ready, listen for incoming messages
                            rx_messages_to_send.recv().await
                        } else {
                            // Not ready, sleep for infinity until next swarm event triggers the
                            // other branch of the tokio::select!
                            loop { tokio::time::sleep(Duration::from_secs(600)).await; }
                        }
                    } => {
                        match m {
                            // No more messages
                            None => {
                                tracing::info!("Exiting swarm event loop, cause: message sender dropped, cannot recv");
                                return Err::<(), _>(Libp2pNodeError::SenderDropped)
                            }

                            // Broadcast message to send
                            Some(SendMessage {
                                msg,
                                to: Recipient::All
                            }) => {
                                tracing::info!("Swarm broadcasting message to all connected peers");
                                swarm
                                    .behaviour_mut()
                                    .floodsub
                                    .publish(floodsub::Topic::new(LIBP2P_MAIN_TOPIC), msg);
                            }

                            // Point-to-Point message to send
                            Some(SendMessage {
                                msg,
                                to: Recipient::Single(recipient_short_id)
                            }) => {
                                let Some(peer_id) = peers.get_peer_id(&recipient_short_id) else {
                                    tracing::error!(recipient_short_id, "Cannot send message to peer with unknown peer id");
                                    continue;
                                };

                                let request_id = swarm.behaviour_mut().point_to_point.send_request(peer_id, msg);
                                tracing::info!(point_to_point_request_id = ?request_id, %peer_id, recipient_short_id, "Sent point to point message to peer");
                            }
                        }
                    }

                    event = swarm.select_next_some() => Self::handle_swarm_event(event, &mut swarm, &peers, &tx_received_messages, &mut ready_send_messages),
                }
            }
        };

        tokio::select! {
            res = inner_fn => {
                tracing::error!(result = ?res, "Swarm event loop stopped unexpectedly.");
            },

            _ = cancellation_token.cancelled() => {
                tracing::info!("Exiting swarm event loop, cause: cancellation token");
            }
        }
    }

    fn handle_swarm_event(
        event: SwarmEvent<BehaviourEvent>,
        swarm: &mut Swarm<Behaviour>,
        peers: &PeerDetails,
        tx_received_messages: &UnboundedSender<ReceivedMessage<u16>>,
        ready_send_messages: &mut bool,
    ) {
        match event {
            SwarmEvent::Behaviour(BehaviourEvent::Floodsub(FloodsubEvent::Message(
                FloodsubMessage {
                    source: sender_peer_id,
                    data,
                    ..
                },
            ))) => {
                let Some(short_id) = peers.get_short_id(&sender_peer_id) else {
                    tracing::error!(
                        sender_peer_id = %sender_peer_id,
                        "Libp2p node received message from an unknown peer"
                    );
                    return;
                };

                tracing::debug!(sender_peer_id = %sender_peer_id, sender_short_id = short_id, "Libp2p node received message from peer");
                if tx_received_messages
                    .send(ReceivedMessage::new_broadcast(short_id, data.to_vec()))
                    .is_err()
                {
                    tracing::error!(sender_peer_id = %sender_peer_id, sender_short_id = short_id, "Libp2p node failed to forward message through channel: channel closed");
                }
            }

            SwarmEvent::Behaviour(BehaviourEvent::PointToPoint(
                RequestResponseEvent::Message {
                    peer: sender_peer_id,
                    message:
                        RequestResponseMessage::Request {
                            request: msg,
                            request_id,
                            channel,
                        },
                    ..
                },
            )) => {
                let Some(sender_short_id) = peers.get_short_id(&sender_peer_id) else {
                    tracing::error!(
                        %sender_peer_id,
                        "Received point to point message from an unknown peer"
                    );
                    return;
                };

                tracing::debug!(%sender_peer_id, sender_short_id, point_to_point_request_id = ?request_id, "Received point to point message from peer");
                // Send an ack to the sender
                if swarm
                    .behaviour_mut()
                    .point_to_point
                    .send_response(channel, ())
                    .is_ok()
                {
                    tracing::error!(%sender_peer_id, sender_short_id, point_to_point_request_id = ?request_id, "Sent point to point response to peer");
                } else {
                    tracing::error!(%sender_peer_id, sender_short_id, point_to_point_request_id = ?request_id, "Failed to send point to point response");
                }

                // Forward the message through channel
                if tx_received_messages
                    .send(ReceivedMessage::new_direct(sender_short_id, msg))
                    .is_err()
                {
                    tracing::error!(%sender_peer_id, sender_short_id, "Libp2p node failed to forward message through channel: channel closed");
                }
            }

            SwarmEvent::Behaviour(BehaviourEvent::Floodsub(FloodsubEvent::Subscribed {
                peer_id,
                topic,
            })) => {
                let short_id = peers.get_short_id(&peer_id);

                tracing::info!(%peer_id, ?short_id, ?topic, "Peer subscribed to topic");
                // Once we've received at least one topic subscription from a remote peer, we should
                // be able to send messages.
                *ready_send_messages = true;
            }

            SwarmEvent::Behaviour(BehaviourEvent::Floodsub(FloodsubEvent::Unsubscribed {
                peer_id,
                topic,
            })) => {
                let short_id = peers.get_short_id(&peer_id);

                tracing::info!(%peer_id, ?short_id, ?topic, "Peer unsubscribed to topic");
            }

            SwarmEvent::Behaviour(BehaviourEvent::PeriodicDial(PeriodicDialEvent::MultiDial(
                multi_dial_opts,
            ))) => {
                tracing::debug!(n_dials = multi_dial_opts.len(), "Obtained peers to dial");

                multi_dial_opts.into_iter().for_each(|dial_opts| {
                    if let Err(e) = swarm.dial(dial_opts) {
                        tracing::error!(error = ?e, "Failed to dial peer");
                    }
                });
            }

            // Successful ping
            SwarmEvent::Behaviour(Ping(ping::Event {
                peer: peer_id,
                result: Ok(rtt),
                ..
            })) => {
                let short_id = peers.get_short_id(&peer_id);

                let host = if let Some(short_id) = short_id {
                    format!("{short_id:02}@{peer_id}")
                } else {
                    format!("??@{peer_id}")
                };
                Metrics::report_host_rtt(rtt.as_secs_f64(), host);
                tracing::debug!(%peer_id, ?short_id, rtt_secs = rtt.as_secs_f64(), "Successful ping");
            }

            // Failed ping
            SwarmEvent::Behaviour(Ping(ping::Event {
                peer: peer_id,
                result: Err(e),
                ..
            })) => {
                let short_id = peers.get_short_id(&peer_id);

                tracing::debug!(%peer_id, ?short_id, error = ?e, "Failed to ping");
            }

            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                let short_id = peer_id.and_then(|peer_id| peers.get_short_id(&peer_id));

                if let Some(peer_id) = peer_id {
                    tracing::warn!(%peer_id, ?short_id, ?error, "Outgoing connection to peer failed");
                } else {
                    tracing::warn!(?error, "Outgoing connection to unknown peer failed");
                }
            }

            SwarmEvent::ConnectionEstablished {
                peer_id,
                endpoint,
                num_established,
                ..
            } => {
                let short_id = peers.get_short_id(&peer_id).or_else(|| {
                    tracing::error!(
                        sender_peer_id = %peer_id,
                        sender_remote_addr = %endpoint.get_remote_address(),
                        "Libp2p node established connection with an unknown peer"
                    );
                    None
                });

                if num_established == const { NonZeroU32::new(1).unwrap() } {
                    // First connection established, report new peer connected
                    Metrics::report_peer_connected();
                }

                tracing::info!(
                    sender_peer_id = %peer_id,
                    sender_short_id = ?short_id,
                    sender_remote_addr = %endpoint.get_remote_address(),
                    num_established,
                    "Libp2p node established connection with peer"
                );
            }

            SwarmEvent::ConnectionClosed {
                peer_id,
                endpoint,
                num_established,
                cause,
                ..
            } => {
                let short_id = peers.get_short_id(&peer_id).or_else(|| {
                    tracing::error!(
                        sender_peer_id = %peer_id,
                        sender_remote_addr = %endpoint.get_remote_address(),
                        "Libp2p node closed connection with an unknown peer"
                    );
                    None
                });

                if num_established == 0 {
                    // No more connections, report disconnect
                    Metrics::report_peer_disconnected();
                }

                tracing::info!(
                    sender_peer_id = %peer_id,
                    sender_short_id = ?short_id,
                    sender_remote_addr = %endpoint.get_remote_address(),
                    remaining_connections = num_established,
                    ?cause,
                    "Libp2p node closed connection to peer"
                );
            }

            SwarmEvent::NewListenAddr { address, .. } => {
                tracing::info!(
                    "Local node is listening on {}",
                    address.with_p2p(swarm.local_peer_id().to_owned()).unwrap()
                );
            }

            _ => {}
        }
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
