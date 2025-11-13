use crate::topic::{Topic, TopicBasedTransport};
use crate::transports::{StatusAction, StatusOutput};
use crate::{PartyIdentifier, ReceivedMessage, Recipient, Transport, TransportSender};
use futures_util::StreamExt;
use prost::Message;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_util::sync::CancellationToken;

const TOPIC_CHANNEL_CACHE_SIZE: usize = 256;

type Receiver<I, M> = tokio::sync::broadcast::Receiver<ReceivedMessage<I, M>>;
type Sender<I, M> = tokio::sync::broadcast::Sender<ReceivedMessage<I, M>>;
type ChannelsMapEntry<I, M> = (Option<Sender<I, M>>, Option<Receiver<I, M>>);
type ChannelsMap<I, M> = HashMap<Cow<'static, [u8]>, ChannelsMapEntry<I, M>>;
type Channels<I, M> = Arc<std::sync::Mutex<ChannelsMap<I, M>>>;

/// A dispatcher that can be used to multiplex many topics in a single [`Transport`].
pub struct TopicDispatcher {
    recv_task_handle: Option<tokio::task::JoinHandle<()>>,
    cancel: CancellationToken,
}

#[derive(thiserror::Error, Debug)]
pub enum TopicDispatcherError {
    #[error("no more items in stream")]
    EmptyStream,
}

impl Default for TopicDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl TopicDispatcher {
    pub fn new() -> Self {
        Self {
            recv_task_handle: None,
            cancel: CancellationToken::new(),
        }
    }

    /// Start a topic dispatcher to obtain an implementation of [`TopicBasedTransport`].
    ///
    /// # Panics
    /// Panics if the dispatcher was already started, or the provided [`Transport`] does not return
    /// a receiver and sender.
    pub fn start<_Transport>(
        &mut self,
        mut transport: _Transport,
    ) -> TopicBasedTransportImpl<_Transport::Sender>
    where
        _Transport: Transport + 'static,
    {
        if self.recv_task_handle.is_some() {
            panic!("Cannot start a topic dispatcher while is already running");
        };

        let receiver = transport
            .receiver_stream()
            .expect("failed to obtain receiver");
        let transport_sender = transport.sender().expect("failed to obtain sender");

        let channels = Channels::default();
        self.recv_task_handle = Some(tokio::task::spawn(Self::recv_task::<_Transport>(
            receiver,
            channels.clone(),
            self.cancel.clone(),
        )));

        TopicBasedTransportImpl::new(transport_sender, channels)
    }

    pub async fn stop(mut self) {
        let Some(recv_task_handle) = self.recv_task_handle.take() else {
            return;
        };
        self.cancel.cancel();
        let _ = recv_task_handle.await;
    }

    async fn recv_task<_Transport>(
        recv_stream: _Transport::ReceiveMessageStream,
        channels: Channels<_Transport::Identity, Vec<u8>>,
        cancel: CancellationToken,
    ) where
        _Transport: Transport,
    {
        tokio::select! {
            res = Self::recv_loop::<_Transport>(recv_stream, channels) => {
                tracing::error!(?res, "Dispatcher loop exited unexpectedly");
            }

            _ = cancel.cancelled() => {
                tracing::info!("Stopping recv_task due to cancellation token");
            }
        }
    }

    async fn recv_loop<_Transport>(
        mut recv_stream: _Transport::ReceiveMessageStream,
        channels: Channels<_Transport::Identity, Vec<u8>>,
    ) -> Result<(), TopicDispatcherError>
    where
        _Transport: Transport,
    {
        // Local broadcast senders used before locking shared struct
        let mut local_senders = HashMap::new();
        loop {
            let m: ReceivedMessage<_> = match recv_stream.next().await {
                Some(Ok(m)) => m,
                Some(Err(e)) => {
                    tracing::error!(error = ?e, "Error receiving message");
                    continue;
                }
                None => {
                    // No more items in the stream, exit
                    return Err(TopicDispatcherError::EmptyStream)?;
                }
            };

            let MessageWithTopic { topic, content } = match MessageWithTopic::decode(&*m.content) {
                Ok(m) => m,
                Err(e) => {
                    tracing::error!(error = ?e, sender = %m.sender, msg_type = ?m.message_type, "Failed to decode message");
                    continue;
                }
            };
            let m = ReceivedMessage {
                sender: m.sender,
                message_type: m.message_type,
                content,
            };

            let topic: Cow<[u8]> = Cow::Owned(topic);

            let sender = {
                // First, try to get from local sender
                let sender = local_senders.get(&topic);
                if let Some(sender) = sender {
                    sender
                } else {
                    // Lock shared struct, and insert (tx, rx)
                    let mut channels = channels.lock().expect("a task panicked holding the mutex");
                    let (tx, _) = channels_get_or_insert(topic.clone(), &mut channels);
                    let tx = tx
                        .take()
                        .expect("must be Some as local_senders does not contain the sender");
                    local_senders.entry(topic).or_insert(tx)
                }
            };

            // Only attempt to send if a receiver exists. No edge-case between sending / recreating a
            // receiver as we only allow a single receiver currently.
            if sender.receiver_count() > 0 && sender.send(m).is_err() {
                // Cleanup / re-creating a receiver is probably sound, but log and monitor for now
                tracing::warn!("Dispatcher lost message due to lack of receivers");
            }
        }
    }
}

pub struct TopicBasedTransportImpl<_TransportSender, M = Vec<u8>>
where
    _TransportSender: TransportSender,
{
    transport_sender: _TransportSender,
    channels: Channels<_TransportSender::Identity, M>,
}

impl<_TransportSender, M> TopicBasedTransportImpl<_TransportSender, M>
where
    _TransportSender: TransportSender,
    ReceivedMessage<_TransportSender::Identity, M>: Clone,
{
    fn new(
        transport_sender: _TransportSender,
        channels: Channels<_TransportSender::Identity, M>,
    ) -> Self {
        Self {
            transport_sender,
            channels,
        }
    }
}

impl<_TransportSender, M> Clone for TopicBasedTransportImpl<_TransportSender, M>
where
    _TransportSender: TransportSender + Clone,
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            transport_sender: self.transport_sender.clone(),
            channels: self.channels.clone(),
        }
    }
}

impl<_TransportSender> TopicBasedTransport for TopicBasedTransportImpl<_TransportSender, Vec<u8>>
where
    _TransportSender: TransportSender + Clone + Send + Sync + 'static,
    TransportImpl<_TransportSender, Vec<u8>>: Transport<Identity = _TransportSender::Identity>,
{
    type Transport = TransportImpl<_TransportSender, Vec<u8>>;
    type Identity = _TransportSender::Identity;

    fn get_transport_for<T>(&self, topic: T) -> Option<Self::Transport>
    where
        T: Topic,
    {
        let transport_sender = self.transport_sender.clone();
        let topic: Cow<[u8]> = Cow::Owned(topic.as_ref().to_owned());

        // try to take an existing receiver, or create a new channel
        let mut channels = self
            .channels
            .lock()
            .expect("a task holding the mutex panicked");
        let (_, receiver) = channels_get_or_insert(topic.clone(), &mut channels);

        // if a receiver for that topic has already been taken, return None
        let receiver = receiver.take()?;
        // Create a new receiver stream
        let receiver_stream = BroadcastStream::new(receiver);

        Some(TransportImpl {
            receiver: Some(receiver_stream),
            transport_sender: TransportSenderImpl {
                transport_sender,
                topic,
            },
        })
    }
}

pub struct TransportImpl<_TransportSender, M = Vec<u8>>
where
    _TransportSender: TransportSender,
{
    receiver: Option<BroadcastStream<ReceivedMessage<_TransportSender::Identity, M>>>,
    transport_sender: TransportSenderImpl<_TransportSender>,
}

impl<_TransportSender> Transport for TransportImpl<_TransportSender, Vec<u8>>
where
    _TransportSender: TransportSender + Clone + Send + Sync + 'static,
{
    type Error = BroadcastStreamRecvError;
    type Identity = _TransportSender::Identity;
    type ReceiveMessageStream =
        BroadcastStream<ReceivedMessage<_TransportSender::Identity, Vec<u8>>>;
    type Sender = TransportSenderImpl<_TransportSender>;

    fn sender(&mut self) -> Option<Self::Sender> {
        Some(self.transport_sender.clone())
    }

    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream> {
        self.receiver.take()
    }
}

#[derive(Clone)]
pub struct TransportSenderImpl<_TransportSender> {
    topic: Cow<'static, [u8]>,
    transport_sender: _TransportSender,
}

impl<_TransportSender> TransportSender for TransportSenderImpl<_TransportSender>
where
    _TransportSender: TransportSender + Sync,
{
    type Identity = _TransportSender::Identity;
    type Error = _TransportSender::Error;

    async fn status(
        &self,
        action: StatusAction,
    ) -> Result<StatusOutput<Self::Identity>, Self::Error> {
        self.transport_sender.status(action).await
    }

    async fn send(&self, msg: Vec<u8>, to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        let m = MessageWithTopic {
            content: msg,
            topic: self.topic.clone().into_owned(),
        };
        self.transport_sender.send(m.encode_to_vec(), to).await
    }
}

/// Helper used to get an existing entry, or insert a value into a locked [`ChannelsMap`]
fn channels_get_or_insert<'a, I, M>(
    topic: Cow<'static, [u8]>,
    channels: &'a mut std::sync::MutexGuard<'_, ChannelsMap<I, M>>,
) -> &'a mut ChannelsMapEntry<I, M>
where
    I: PartyIdentifier,
    M: Clone,
{
    channels.entry(topic).or_insert_with(|| {
        let (tx, rx) = tokio::sync::broadcast::channel(TOPIC_CHANNEL_CACHE_SIZE);
        (Some(tx), Some(rx))
    })
}

#[derive(Clone, prost::Message)]
struct MessageWithTopic {
    #[prost(bytes, tag = "1")]
    topic: Vec<u8>,

    #[prost(bytes, tag = "2")]
    content: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    async fn can_send_and_receive_on_many_topics<T>(transport_1: T, mut transport_2: T)
    where
        T: Transport<Identity = u16> + 'static,
        TopicBasedTransportImpl<T::Sender>: TopicBasedTransport<Identity = T::Identity>,
    {
        let transport_2_sender = transport_2.sender().expect("sender should exist");
        let mut transport_2_receiver = transport_2
            .receiver_stream()
            .expect("receiver stream should exist");

        let mut dispatcher = TopicDispatcher::default();
        let topic_transport_1 = dispatcher.start(transport_1);
        let mut topic_transport_1_for_a = topic_transport_1
            .get_transport_for(b"a")
            .expect("should create transport");
        let mut topic_transport_1_for_b = topic_transport_1
            .get_transport_for(b"b")
            .expect("should create transport");

        topic_transport_1_for_a
            .sender()
            .expect("sender should exist")
            .broadcast(b"sent on topic a".to_vec())
            .await
            .expect("should broadcast on topic");
        let m = transport_2_receiver
            .next()
            .await
            .expect("receiver should have received message")
            .expect("message should be ok");
        let m_with_topic = MessageWithTopic::decode(&*m.content).expect("should decode message");
        assert_eq!(m_with_topic.topic, b"a");
        assert_eq!(m.sender, 1);
        assert_eq!(m_with_topic.content, b"sent on topic a");

        topic_transport_1_for_b
            .sender()
            .expect("sender should exist")
            .broadcast(b"sent on topic b".to_vec())
            .await
            .expect("should broadcast on topic");
        let m = transport_2_receiver
            .next()
            .await
            .expect("receiver should have received message")
            .expect("message should be ok");
        let m_with_topic = MessageWithTopic::decode(&*m.content).expect("should decode message");
        assert_eq!(m_with_topic.topic, b"b");
        assert_eq!(m.sender, 1);
        assert_eq!(m_with_topic.content, b"sent on topic b");

        // Send message on topic a
        transport_2_sender
            .broadcast(
                MessageWithTopic {
                    topic: b"a".to_vec(),
                    content: b"sent on topic a from 2".to_vec(),
                }
                .encode_to_vec(),
            )
            .await
            .expect("should broadcast on topic");
        let m: ReceivedMessage<_, _> = topic_transport_1_for_a
            .receiver_stream()
            .expect("receiver should exist")
            .next()
            .await
            .expect("should not be none")
            .expect("message should be ok");
        assert_eq!(m.sender, 2);
        assert_eq!(m.content, b"sent on topic a from 2");

        dispatcher.stop();
    }

    #[cfg(feature = "in_memory")]
    #[tokio::test]
    async fn can_send_and_receive_on_many_topics_in_memory() {
        use crate::transports::in_memory::MemoryNetwork;

        let mut transports = MemoryNetwork::get_transports(1..=2);
        let transport_1 = transports.pop_front().unwrap();
        let transport_2 = transports.pop_front().unwrap();

        can_send_and_receive_on_many_topics(transport_1, transport_2).await;
    }

    #[cfg(feature = "libp2p")]
    #[tokio::test]
    async fn can_send_and_receive_on_many_topics_libp2p() {
        use crate::transports::libp2p::{self, Libp2pNode};

        let [mut node_1, mut node_2]: [Libp2pNode<_>; 2] =
            libp2p::tests::start_nodes(&[1u16, 2], 43260)
                .await
                .try_into()
                .unwrap_or_else(|_| panic!("failed to start nodes"));

        let transport_1 = node_1.get_transport().expect("should get transport");
        let transport_2 = node_2.get_transport().expect("should get transport");

        can_send_and_receive_on_many_topics(transport_1, transport_2).await;
    }
}
