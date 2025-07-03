//! Libp2p node that can be used to broadcast and receive arbitrary messages using floodsub and a
//! peer whitelist.

use crate::signer::threshold_signer::libp2p::BehaviourEvent::Ping;
use crate::signer::threshold_signer::metrics::Metrics;
use futures_util::StreamExt;
use libp2p::allow_block_list::AllowedPeers;
use libp2p::floodsub::{FloodsubEvent, FloodsubMessage};
use libp2p::identity::Keypair;
use libp2p::swarm::NetworkBehaviour;
use libp2p::swarm::dial_opts::{DialOpts, PeerCondition};
use libp2p::{
    Multiaddr, PeerId, Swarm, allow_block_list, floodsub, noise, ping, swarm::SwarmEvent, tcp,
    yamux,
};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

const LIBP2P_MAIN_TOPIC: &str = "main";

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

pub struct LibP2PNode {
    key: Keypair,
    peer_addrs: Vec<Multiaddr>,
    peer_ids: Vec<PeerId>,
    peer_short_ids: Vec<u16>,
}

impl LibP2PNode {
    /// Create a new libp2p node.
    pub fn new(
        key: Keypair,
        peer_addrs: Vec<Multiaddr>,
        peer_ids: Vec<PeerId>,
        peer_short_ids: Vec<u16>,
    ) -> Self {
        Self {
            key,
            peer_addrs,
            peer_ids,
            peer_short_ids,
        }
    }

    /// Runs a new libp2p node that listens on `listen_addr`, forwards messages from the swarm to `tx_received_messages`,
    /// and broadcasts messages from `rx_messages_to_send` to the swarm.
    pub fn run(
        self,
        listen_addr: Multiaddr,
        tx_received_messages: UnboundedSender<(u16, Vec<u8>)>,
        rx_messages_to_send: UnboundedReceiver<Vec<u8>>,
        cancellation_token: CancellationToken,
    ) -> Result<(), Libp2pNodeError> {
        // Create the peer_id -> u16 mapping
        let peer_id_to_short_id: HashMap<PeerId, u16> =
            HashMap::from_iter(self.peer_ids.iter().cloned().zip(self.peer_short_ids));

        // Create a new swarm
        let mut swarm = Self::configure_swarm(self.key.clone(), self.peer_ids.clone())?;

        // Listen on all interfaces
        swarm
            .listen_on(listen_addr)
            .map_err(|e| Libp2pNodeError::TransportError(e.into(), "failed to start listener"))?;

        // Register each of the peers
        self.peer_ids
            .into_iter()
            .zip(self.peer_addrs)
            .for_each(|(peer_id, peer_addr)| {
                swarm.add_peer_address(peer_id, peer_addr.clone());

                swarm
                    .behaviour_mut()
                    .floodsub
                    .add_node_to_partial_view(peer_id);

                let dial_opts = DialOpts::peer_id(peer_id)
                    .addresses(vec![peer_addr.clone()])
                    .condition(PeerCondition::Always)
                    .build();
                if let Err(e) = swarm.dial(dial_opts) {
                    tracing::error!("Failed to dial peer at address {peer_addr}: {e:?}")
                }
            });

        // Create a floodsub topic and subscribe
        let topic = floodsub::Topic::new(LIBP2P_MAIN_TOPIC);
        let _ = swarm.behaviour_mut().floodsub.subscribe(topic);

        // Process swarm events in a separate task
        tokio::spawn(Self::swarm_event_loop(
            swarm,
            peer_id_to_short_id,
            tx_received_messages,
            rx_messages_to_send,
            cancellation_token,
        ));

        Ok(())
    }

    /// Configure a libp2p swarm by setting up the keypair, various layers and the behaviour
    fn configure_swarm(
        keypair: Keypair,
        peers: impl IntoIterator<Item = PeerId>,
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
            .with_behaviour(|_| Behaviour::new(peer_id, peers))
            .unwrap() // infallible
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)) // stay connected to the peer even if idle
            })
            .build())
    }

    /// Main event loop handling incoming events from swarm, and incoming messages from the `rx_messages_to_send` channel.
    async fn swarm_event_loop(
        mut swarm: Swarm<Behaviour>,
        peer_id_to_short_id: HashMap<PeerId, u16>,
        tx_received_messages: UnboundedSender<(u16, Vec<u8>)>,
        mut rx_messages_to_send: UnboundedReceiver<Vec<u8>>,
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
                            None => {
                                tracing::info!("Exiting swarm event loop, cause: message sender dropped, cannot recv");
                                return Err::<(), _>(Libp2pNodeError::SenderDropped)
                            }
                            Some(m) => {
                                tracing::info!("Swarm broadcasting message to all connected peers");
                                swarm
                                    .behaviour_mut()
                                    .floodsub
                                    .publish(floodsub::Topic::new(LIBP2P_MAIN_TOPIC), m);
                            }
                        }
                    }

                    event = swarm.select_next_some() => Self::handle_swarm_event(event, &mut swarm, &peer_id_to_short_id, &tx_received_messages, &mut ready_send_messages),
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
        peer_id_to_short_id: &HashMap<PeerId, u16>,
        tx_received_messages: &UnboundedSender<(u16, Vec<u8>)>,
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
                let Some(short_id) = peer_id_to_short_id.get(&sender_peer_id) else {
                    tracing::error!(
                        incoming_peer_id = %sender_peer_id,
                        "Libp2p node received message from an unknown peer"
                    );
                    return;
                };

                tracing::debug!(incoming_peer_id = %sender_peer_id, incoming_short_id = short_id, "Libp2p node received message from peer");
                if tx_received_messages
                    .send((*short_id, data.to_vec()))
                    .is_err()
                {
                    tracing::error!(incoming_peer_id = %sender_peer_id, incoming_short_id = short_id, "Libp2p node failed to forward message through channel: channel closed");
                }
            }

            SwarmEvent::Behaviour(BehaviourEvent::Floodsub(FloodsubEvent::Subscribed {
                peer_id,
                topic,
            })) => {
                let short_id = peer_id_to_short_id.get(&peer_id).or_else(|| {
                    tracing::error!(
                        incoming_peer_id = %peer_id,
                        "Failed to convert peer_id to short_id"
                    );
                    None
                });

                tracing::info!(%peer_id, ?short_id, ?topic, "Peer subscribed to topic");
                // Once we've received at least one topic subscription from a remote peer, we should
                // be able to send messages.
                *ready_send_messages = true;
            }

            SwarmEvent::Behaviour(BehaviourEvent::Floodsub(FloodsubEvent::Unsubscribed {
                peer_id,
                topic,
            })) => {
                let short_id = peer_id_to_short_id.get(&peer_id).or_else(|| {
                    tracing::error!(
                        incoming_peer_id = %peer_id,
                        "Failed to convert peer_id to short_id"
                    );
                    None
                });

                tracing::info!(%peer_id, ?short_id, ?topic, "Peer unsubscribed to topic");
            }

            // Successful ping
            SwarmEvent::Behaviour(Ping(ping::Event {
                peer: peer_id,
                result: Ok(rtt),
                ..
            })) => {
                let short_id = peer_id_to_short_id.get(&peer_id).or_else(|| {
                    tracing::error!(
                        incoming_peer_id = %peer_id,
                        "Failed to convert peer_id to short_id"
                    );
                    None
                });

                let host = if let Some(short_id) = short_id {
                    format!("{short_id:02}@{peer_id}")
                } else {
                    format!("??@{peer_id}")
                };
                Metrics::report_host_rtt(rtt.as_secs_f64(), host);
                tracing::debug!(%peer_id, ?short_id, rtt_ms = rtt.as_millis(), "Successful ping");
            }

            // Failed ping
            SwarmEvent::Behaviour(Ping(ping::Event {
                peer: peer_id,
                result: Err(e),
                ..
            })) => {
                let short_id = peer_id_to_short_id.get(&peer_id).or_else(|| {
                    tracing::error!(
                        incoming_peer_id = %peer_id,
                        "Failed to convert peer_id to short_id"
                    );
                    None
                });

                tracing::debug!(%peer_id, ?short_id, error = ?e, "Failed to ping");
            }

            SwarmEvent::ConnectionEstablished {
                peer_id,
                endpoint,
                num_established,
                ..
            } => {
                let short_id = peer_id_to_short_id.get(&peer_id).or_else(|| {
                    tracing::error!(
                        incoming_peer_id = %peer_id,
                        incoming_remote_addr = %endpoint.get_remote_address(),
                        "Libp2p node established connection with an unknown peer"
                    );
                    None
                });

                if num_established == const { NonZeroU32::new(1).unwrap() } {
                    // First connection established, report new peer connected
                    Metrics::report_peer_connected();
                }

                tracing::info!(
                    incoming_peer_id = %peer_id,
                    incoming_short_id = ?short_id,
                    incoming_remote_addr = %endpoint.get_remote_address(),
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
                let short_id = peer_id_to_short_id.get(&peer_id).or_else(|| {
                    tracing::error!(
                        incoming_peer_id = %peer_id,
                        incoming_remote_addr = %endpoint.get_remote_address(),
                        "Libp2p node closed connection with an unknown peer"
                    );
                    None
                });

                if num_established == 0 {
                    // No more connections, report disconnect
                    Metrics::report_peer_disconnected();
                }

                tracing::info!(
                    incoming_peer_id = %peer_id,
                    incoming_short_id = ?short_id,
                    incoming_remote_addr = %endpoint.get_remote_address(),
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
    floodsub: floodsub::Floodsub,
    allowed_peers: allow_block_list::Behaviour<AllowedPeers>,
    ping: ping::Behaviour,
}

impl Behaviour {
    /// Create a new behaviour
    fn new(local_peer_id: PeerId, peers: impl IntoIterator<Item = PeerId>) -> Self {
        // Build a list of allowed peers
        let allowed_peers = {
            let mut allowed_peers = allow_block_list::Behaviour::default();
            peers.into_iter().for_each(|p| {
                allowed_peers.allow_peer(p);
            });
            allowed_peers
        };

        Self {
            allowed_peers,
            floodsub: floodsub::Floodsub::new(local_peer_id),
            ping: ping::Behaviour::default(),
        }
    }
}
