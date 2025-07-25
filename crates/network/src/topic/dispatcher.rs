use crate::topic::{Topic, TopicBasedTransport};
use crate::{ReceivedMessage, Recipient, Transport, TransportSender};
use futures_util::stream::BoxStream;
use futures_util::StreamExt;
use prost::Message;
use std::borrow::Cow;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::CancellationToken;

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

type TopicMessageTuple<I, M = Vec<u8>> = (Vec<u8>, ReceivedMessage<I, M>);

type BoxReceivedMessageStream<T, M, E> =
    BoxStream<'static, Result<ReceivedMessage<<T as TransportSender>::Identity, M>, E>>;

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

        let (dispatcher_sender, dispatcher_receiver) = tokio::sync::broadcast::channel(1024);

        self.recv_task_handle = Some(tokio::task::spawn(Self::recv_task::<_Transport>(
            receiver,
            dispatcher_sender,
            self.cancel.clone(),
        )));

        TopicBasedTransportImpl::new(transport_sender, dispatcher_receiver)
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
        dispatcher_sender: tokio::sync::broadcast::Sender<TopicMessageTuple<_Transport::Identity>>,
        cancel: CancellationToken,
    ) where
        _Transport: Transport,
    {
        tokio::select! {
            res = Self::recv_loop::<_Transport>(recv_stream, dispatcher_sender) => {
                tracing::error!(?res, "Dispatcher loop exited unexpectedly");
            }

            _ = cancel.cancelled() => {
                tracing::info!("Stopping recv_task due to cancellation token");
            }
        }
    }

    async fn recv_loop<_Transport>(
        mut recv_stream: _Transport::ReceiveMessageStream,
        dispatcher_sender: tokio::sync::broadcast::Sender<TopicMessageTuple<_Transport::Identity>>,
    ) -> Result<(), TopicDispatcherError>
    where
        _Transport: Transport,
    {
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

            if dispatcher_sender.send((topic, m)).is_err() {
                tracing::info!("Lost message due to lack of receivers");
            }
        }
    }
}

pub struct TopicBasedTransportImpl<_TransportSender, M = Vec<u8>>
where
    _TransportSender: TransportSender,
{
    transport_sender: _TransportSender,
    dispatcher_receiver:
        tokio::sync::broadcast::Receiver<TopicMessageTuple<_TransportSender::Identity, M>>,
}

impl<_TransportSender, M> TopicBasedTransportImpl<_TransportSender, M>
where
    _TransportSender: TransportSender,
    ReceivedMessage<_TransportSender::Identity, M>: Clone,
{
    fn new(
        transport_sender: _TransportSender,
        dispatcher_receiver: tokio::sync::broadcast::Receiver<
            TopicMessageTuple<_TransportSender::Identity, M>,
        >,
    ) -> Self {
        Self {
            transport_sender,
            dispatcher_receiver,
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
            dispatcher_receiver: self.dispatcher_receiver.resubscribe(),
        }
    }
}

impl<_TransportSender> TopicBasedTransport for TopicBasedTransportImpl<_TransportSender, Vec<u8>>
where
    _TransportSender: TransportSender + Clone + Send + Sync + 'static,
    TransportImpl<_TransportSender, BroadcastStreamRecvError, Vec<u8>>:
        Transport<Identity = _TransportSender::Identity>,
{
    type Transport = TransportImpl<_TransportSender, BroadcastStreamRecvError, Vec<u8>>;
    type Identity = _TransportSender::Identity;

    fn get_transport_for<T>(&self, topic: T) -> Option<Self::Transport>
    where
        T: Topic,
    {
        let transport_sender = self.transport_sender.clone();
        let topic: Cow<[u8]> = Cow::Owned(topic.as_ref().to_owned());
        let topic_cloned = topic.clone();

        let receiver_stream = BroadcastStream::new(self.dispatcher_receiver.resubscribe())
            .filter_map(move |item| {
                let topic = topic.clone();
                async move {
                    let (msg_topic, msg) = match item {
                        Ok(msg) => msg,
                        Err(e) => return Some(Err(e)),
                    };
                    msg_topic.eq(topic.as_ref()).then(|| Ok(msg))
                }
            })
            .boxed();

        Some(TransportImpl {
            receiver: Some(receiver_stream),
            transport_sender: TransportSenderImpl {
                transport_sender,
                topic: topic_cloned,
            },
        })
    }
}

pub struct TransportImpl<_TransportSender, E, M = Vec<u8>>
where
    _TransportSender: TransportSender,
{
    receiver: Option<BoxReceivedMessageStream<_TransportSender, M, E>>,
    transport_sender: TransportSenderImpl<_TransportSender>,
}

impl<_TransportSender, E> Transport for TransportImpl<_TransportSender, E, Vec<u8>>
where
    _TransportSender: TransportSender + Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    type Error = E;
    type Identity = _TransportSender::Identity;
    type ReceiveMessageStream =
        BoxStream<'static, Result<ReceivedMessage<Self::Identity, Vec<u8>>, E>>;
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

    async fn send(&self, msg: Vec<u8>, to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        let m = MessageWithTopic {
            content: msg,
            topic: self.topic.clone().into_owned(),
        };
        self.transport_sender.send(m.encode_to_vec(), to).await
    }
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
        use std::num::NonZeroU16;

        let [mut node_1, mut node_2]: [Libp2pNode; 2] =
            libp2p::tests::start_nodes(const { NonZeroU16::new(2).unwrap() }, 43260)
                .await
                .try_into()
                .unwrap_or_else(|_| panic!("failed to start nodes"));

        let transport_1 = node_1.get_transport().expect("should get transport");
        let transport_2 = node_2.get_transport().expect("should get transport");

        can_send_and_receive_on_many_topics(transport_1, transport_2).await;
    }
}
