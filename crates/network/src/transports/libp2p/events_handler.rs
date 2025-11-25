use crate::transports::libp2p::dialer::PeriodicDialEvent;
use crate::transports::libp2p::metrics::Metrics;
use crate::transports::libp2p::{
    Behaviour, BehaviourEvent, LIBP2P_MAIN_TOPIC, Libp2pNodeError, PeerDetails,
};
use crate::transports::{SendBroadcastMessage, SendDirectMessage, TransportAction};
use crate::{PartyIdentifier, ReceivedMessage};
use futures_util::StreamExt;
use libp2p::request_response::{Event as RequestResponseEvent, Message as RequestResponseMessage};
use libp2p::{Swarm, gossipsub, ping, swarm::SwarmEvent};
use std::num::NonZeroU32;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

pub(super) struct EventsHandler<ID: PartyIdentifier> {
    short_id: ID,
    swarm: Swarm<Behaviour<ID>>,
    peers: PeerDetails<ID>,
    tx_received_messages: UnboundedSender<ReceivedMessage<ID>>,
    rx_messages_to_send: UnboundedReceiver<TransportAction<ID>>,
    cancellation_token: CancellationToken,
}

impl<ID: PartyIdentifier> EventsHandler<ID> {
    pub(super) fn new(
        short_id: ID,
        swarm: Swarm<Behaviour<ID>>,
        peers: PeerDetails<ID>,
        tx_received_messages: UnboundedSender<ReceivedMessage<ID>>,
        rx_messages_to_send: UnboundedReceiver<TransportAction<ID>>,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            short_id,
            swarm,
            peers,
            tx_received_messages,
            rx_messages_to_send,
            cancellation_token,
        }
    }

    pub(super) async fn run(mut self) -> Result<(), Libp2pNodeError> {
        let cancel = self.cancellation_token.clone();
        tokio::select! {
            res = self.swarm_event_loop() => {
                tracing::error!(result = ?res, "Swarm event loop stopped unexpectedly.");
                res
            },

            _ = cancel.cancelled() => {
                tracing::info!("Exiting swarm event loop, cause: cancellation token");
                Ok(())
            }
        }
    }
}

impl<ID: PartyIdentifier> EventsHandler<ID> {
    /// Main event loop handling incoming events from swarm, and incoming messages from the `rx_messages_to_send` channel.
    async fn swarm_event_loop(&mut self) -> Result<(), Libp2pNodeError> {
        let mut ready_send_messages = false;
        loop {
            tokio::select! {
                msg = self.rx_messages_to_send.recv(), if ready_send_messages => {
                    match msg {
                        // No more messages
                        None => {
                            tracing::info!("Exiting swarm event loop, cause: message sender dropped, cannot recv");
                            return Err(Libp2pNodeError::SenderDropped)
                        }

                        Some(TransportAction::SendDirectMessage(msg)) => self.send_direct_message_to_swarm(msg),
                        Some(TransportAction::SendBroadcastMessage(msg)) => self.send_broadcast_message_to_swarm(msg),
                    }
                }

                event = self.swarm.select_next_some() => self.handle_swarm_event(event, &mut ready_send_messages),
            }
        }
    }

    fn send_direct_message_to_swarm(&mut self, msg: SendDirectMessage<ID>) {
        let SendDirectMessage {
            to: recipient_short_id,
            msg,
        } = msg;

        if recipient_short_id == self.short_id {
            tracing::debug!("Forwarding direct message to self");
            if self
                .tx_received_messages
                .send(ReceivedMessage::new_direct(self.short_id, msg))
                .is_err()
            {
                tracing::error!("Libp2p node failed to forward to self: channel closed");
            }
        } else {
            let Some(peer_id) = self.peers.get_peer_id(&recipient_short_id) else {
                tracing::error!(
                    ?recipient_short_id,
                    "Cannot send message to peer with unknown peer id"
                );
                return;
            };

            let request_id = self
                .swarm
                .behaviour_mut()
                .point_to_point
                .send_request(peer_id, msg);
            tracing::debug!(point_to_point_request_id = ?request_id, %peer_id, ?recipient_short_id, "Sent point to point message to peer");
        }
    }

    fn send_broadcast_message_to_swarm(&mut self, msg: SendBroadcastMessage) {
        tracing::info!("Swarm broadcasting message to all connected peers");
        if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(
            gossipsub::Sha256Topic::new(LIBP2P_MAIN_TOPIC),
            msg.msg.clone(),
        ) {
            tracing::error!(error = ?e, "Failed to publish message to gossipsub topic");
        }

        if msg.broadcast_self {
            tracing::debug!("Sending broadcast to self");
            if self
                .tx_received_messages
                .send(ReceivedMessage::new_broadcast(self.short_id, msg.msg))
                .is_err()
            {
                tracing::error!("Libp2p node failed echo broadcast to self: channel closed");
            }
        }
    }

    fn handle_swarm_event(
        &mut self,
        event: SwarmEvent<BehaviourEvent<ID>>,
        ready_send_messages: &mut bool,
    ) {
        match event {
            SwarmEvent::Behaviour(behaviour_event) => {
                self.handle_behaviour_event(behaviour_event, ready_send_messages);
            }

            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                let short_id = peer_id.and_then(|peer_id| self.peers.get_short_id(&peer_id));

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
                let short_id = self.peers.get_short_id(&peer_id).or_else(|| {
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
                let short_id = self.peers.get_short_id(&peer_id).or_else(|| {
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
                    address
                        .with_p2p(self.swarm.local_peer_id().to_owned())
                        .unwrap()
                );
            }

            _ => {}
        }
    }

    fn handle_behaviour_event(
        &mut self,
        event: BehaviourEvent<ID>,
        ready_send_messages: &mut bool,
    ) {
        match event {
            BehaviourEvent::Gossipsub(event) => {
                self.handle_gossipsub_event(event, ready_send_messages);
            }

            BehaviourEvent::PointToPoint(event) => {
                self.handle_point_to_point_event(event);
            }

            BehaviourEvent::PeriodicDial(event) => {
                self.handle_periodic_dial_event(event);
            }

            BehaviourEvent::Ping(event) => {
                self.handle_ping_event(event);
            }

            BehaviourEvent::AllowedPeers(_) => (),
        }
    }

    fn handle_gossipsub_event(&mut self, event: gossipsub::Event, ready_send_messages: &mut bool) {
        use gossipsub::Event;

        match event {
            Event::Message {
                message: gossipsub::Message { source, data, .. },
                ..
            } => {
                let Some(sender_peer_id) = source else {
                    tracing::warn!("Libp2p node received message from an anonymous peer");
                    return;
                };

                let Some(short_id) = self.peers.get_short_id(&sender_peer_id) else {
                    tracing::error!(sender_peer_id = %sender_peer_id, "Libp2p node received message from an unknown peer");
                    return;
                };

                tracing::debug!(sender_peer_id = %sender_peer_id, sender_short_id = ?short_id, "Libp2p node received message from peer");
                if self
                    .tx_received_messages
                    .send(ReceivedMessage::new_broadcast(short_id, data.to_vec()))
                    .is_err()
                {
                    tracing::error!(sender_peer_id = %sender_peer_id, sender_short_id = ?short_id, "Libp2p node failed to forward message through channel: channel closed");
                }
            }

            Event::Subscribed { peer_id, topic } => {
                let short_id = self.peers.get_short_id(&peer_id);

                tracing::info!(%peer_id, ?short_id, ?topic, "Peer subscribed to topic");
                // Once we've received at least one topic subscription from a remote peer, we should
                // be able to send messages.
                *ready_send_messages = true;
            }

            Event::Unsubscribed { peer_id, topic } => {
                let short_id = self.peers.get_short_id(&peer_id);

                tracing::info!(%peer_id, ?short_id, ?topic, "Peer unsubscribed to topic");
            }

            _ => (),
        }
    }

    fn handle_point_to_point_event(&mut self, event: RequestResponseEvent<Vec<u8>, ()>) {
        match event {
            RequestResponseEvent::Message {
                peer: sender_peer_id,
                message:
                    RequestResponseMessage::Request {
                        request: msg,
                        request_id,
                        channel,
                    },
                ..
            } => {
                let Some(sender_short_id) = self.peers.get_short_id(&sender_peer_id) else {
                    tracing::error!(
                        %sender_peer_id,
                        point_to_point_request_id = ?request_id,
                        "Received point to point message from an unknown peer"
                    );
                    return;
                };

                tracing::debug!(%sender_peer_id, ?sender_short_id, point_to_point_request_id = ?request_id, "Received point to point message from peer");
                // Send an ack to the sender
                if self
                    .swarm
                    .behaviour_mut()
                    .point_to_point
                    .send_response(channel, ())
                    .is_ok()
                {
                    tracing::debug!(%sender_peer_id, ?sender_short_id, point_to_point_request_id = ?request_id, "Sent point to point response to peer");
                } else {
                    tracing::error!(%sender_peer_id, ?sender_short_id, point_to_point_request_id = ?request_id, "Failed to send point to point response");
                }

                // Forward the message through channel
                if self
                    .tx_received_messages
                    .send(ReceivedMessage::new_direct(sender_short_id, msg))
                    .is_err()
                {
                    tracing::error!(%sender_peer_id, ?sender_short_id, "Libp2p node failed to forward message through channel: channel closed");
                }
            }

            RequestResponseEvent::OutboundFailure {
                peer: sender_peer_id,
                request_id,
                error,
                ..
            } => {
                let sender_short_id = self.peers.get_short_id(&sender_peer_id);
                tracing::error!(%sender_peer_id, ?sender_short_id, point_to_point_request_id = ?request_id, ?error, "Point to point outbound failure");
            }

            RequestResponseEvent::InboundFailure {
                peer: sender_peer_id,
                request_id,
                error,
                ..
            } => {
                let sender_short_id = self.peers.get_short_id(&sender_peer_id);
                tracing::error!(%sender_peer_id, ?sender_short_id, point_to_point_request_id = ?request_id, ?error, "Point to point inbound failure");
            }

            _ => {}
        }
    }

    fn handle_periodic_dial_event(&mut self, event: PeriodicDialEvent) {
        match event {
            PeriodicDialEvent::MultiDial(multi_dial_opts) => {
                tracing::debug!(n_dials = multi_dial_opts.len(), "Obtained peers to dial");

                multi_dial_opts.into_iter().for_each(|dial_opts| {
                    if let Err(e) = self.swarm.dial(dial_opts) {
                        tracing::error!(error = ?e, "Failed to dial peer");
                    }
                });
            }
        }
    }

    fn handle_ping_event(&mut self, event: ping::Event) {
        match event {
            // Successful ping
            ping::Event {
                peer: peer_id,
                result: Ok(rtt),
                ..
            } => {
                let short_id = self.peers.get_short_id(&peer_id);

                let host = if let Some(short_id) = short_id {
                    format!("{short_id:02}@{peer_id}")
                } else {
                    format!("??@{peer_id}")
                };
                Metrics::report_host_rtt(rtt.as_secs_f64(), host);
                tracing::debug!(%peer_id, ?short_id, rtt_secs = rtt.as_secs_f64(), "Successful ping");
            }

            // Failed ping
            ping::Event {
                peer: peer_id,
                result: Err(e),
                ..
            } => {
                let short_id = self.peers.get_short_id(&peer_id);

                tracing::debug!(%peer_id, ?short_id, error = ?e, "Failed to ping");
            }
        }
    }
}
