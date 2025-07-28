// use crate::helpers::PartyId;
// use crate::network::{
//     AuthenticatedTopicTransport, AuthenticatedTransport, AuthenticatedTransportSerialized,
//     ReceivedMessage, SendMessage, SerializedMessage,
// };
// use async_trait::async_trait;
// use serde::{Deserialize, Serialize};
// use std::collections::hash_map::Entry;
// use std::collections::{HashMap, VecDeque};
// use std::sync::{Arc, Mutex};
// use thiserror::Error;
// use tokio::sync::Notify;
// use tokio::task::JoinHandle;
// use tokio_util::sync::CancellationToken;
// use tracing::error;
// 
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SerializedMessageWithTopic {
//     pub topic: String,
//     pub msg: SerializedMessage,
// }
// 
// pub struct TopicTransportDispatcher {
//     recv_task: Option<JoinHandle<()>>,
//     cancel: CancellationToken,
// }
// 
// #[derive(Error, Debug)]
// pub enum TransportDispatcherError {
//     #[error(transparent)]
//     UnderlyingTransport(#[from] Box<dyn std::error::Error + Send + Sync>),
//     #[error("failed to serialize msg into bson")]
//     BsonSer(#[from] bson::ser::Error),
//     #[error("failed to deserialize msg from bson")]
//     BsonDe(#[from] bson::de::Error),
// }
// 
// type ReceivedSerializedMessage = ReceivedMessage<SerializedMessage, PartyId>;
// type SharedChannelMap = Arc<VecDequeChannel<ReceivedSerializedMessage>>;
// type TopicChannelsMap = Mutex<HashMap<String, SharedChannelMap>>;
// 
// pub struct TopicTransportDispatcherInstance<T>
// where
//     T: AuthenticatedTransport<SerializedMessageWithTopic, Identity = PartyId>,
// {
//     rx_topic_channels: Arc<TopicChannelsMap>,
//     tx_channel: Arc<T>,
// }
// 
// struct TransportDispatcherInstance<T> {
//     topic: String,
//     rx_topic_channel: SharedChannelMap,
//     tx_channel: Arc<T>,
// }
// 
// impl TopicTransportDispatcher {
//     pub fn new() -> Self {
//         Self {
//             cancel: CancellationToken::new(),
//             recv_task: None,
//         }
//     }
// 
//     /// Executes a dispatcher thread in the background
//     pub fn start<T>(&mut self, transport: T) -> TopicTransportDispatcherInstance<T>
//     where
//         T: AuthenticatedTransport<SerializedMessageWithTopic, Identity = PartyId>,
//     {
//         let transport = Arc::new(transport);
//         let topic_channels = Arc::new(Mutex::new(HashMap::new()));
//         let topic_transport = TopicTransportDispatcherInstance {
//             rx_topic_channels: topic_channels.clone(),
//             tx_channel: transport.clone(),
//         };
// 
//         self.recv_task = tokio::task::spawn(Self::recv_task(
//             transport,
//             topic_channels,
//             self.cancel.clone(),
//         ))
//         .into();
// 
//         topic_transport
//     }
// 
//     /// Stop the dispatcher thread.
//     pub async fn stop(self) {
//         self.cancel.cancel();
//         self.recv_task
//             .expect("failed to join dispatcher recv task")
//             .await
//             .expect("dispatcher recv task returned error");
//     }
// 
//     /// Receive message and dispatch it on the correct topic channel.
//     async fn recv_task<T>(
//         transport: Arc<T>,
//         tx_topic_channels: Arc<TopicChannelsMap>,
//         cancel: CancellationToken,
//     ) where
//         T: AuthenticatedTransport<SerializedMessageWithTopic, Identity = PartyId>,
//     {
//         tokio::select! {
//             _ = cancel.cancelled_owned() => (),
// 
//             _ = async {
//                 loop {
//                     let msg = match transport.recv().await {
//                         Ok(msg) => msg,
//                         Err(e) => {
//                             error!("Dispatcher recv task failed to recv from transport: {e:?}");
//                             continue;
//                         }
//                     };
// 
//                     let tx_topic = get_or_insert_topic_channel(msg.content.topic.clone(), &tx_topic_channels);
//                     let msg = ReceivedMessage {
//                         sender: msg.sender,
//                         recipient: msg.recipient,
//                         content: msg.content.msg
//                     };
// 
//                     tx_topic.send(msg);
//                 }
//             } => (),
//         }
//     }
// }
// 
// impl Default for TopicTransportDispatcher {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// 
// impl<T> AuthenticatedTopicTransport for TopicTransportDispatcherInstance<T>
// where
//     T: AuthenticatedTransport<SerializedMessageWithTopic, Identity = PartyId>,
// {
//     type Identity = PartyId;
//     type Error = TransportDispatcherError;
// 
//     fn get_transport(
//         self: Arc<Self>,
//         topic: String,
//     ) -> impl AuthenticatedTransportSerialized<Identity = Self::Identity, Error = Self::Error> {
//         let rx_topic_channel = get_or_insert_topic_channel(topic.clone(), &self.rx_topic_channels);
// 
//         TransportDispatcherInstance {
//             topic: topic.clone(),
//             rx_topic_channel,
//             tx_channel: self.tx_channel.clone(),
//         }
//     }
// }
// 
// #[async_trait]
// impl<T> AuthenticatedTransport<SerializedMessage> for TransportDispatcherInstance<T>
// where
//     T: AuthenticatedTransport<SerializedMessageWithTopic, Identity = PartyId>,
// {
//     type Identity = PartyId;
//     type Error = TransportDispatcherError;
// 
//     async fn send(
//         &self,
//         msg: &SendMessage<SerializedMessage, Self::Identity>,
//     ) -> Result<(), Self::Error> {
//         let msg = SendMessage {
//             recipient: msg.recipient,
//             content: SerializedMessageWithTopic {
//                 topic: self.topic.clone(),
//                 msg: msg.content.clone(),
//             },
//         };
// 
//         self.tx_channel
//             .send(&msg)
//             .await
//             .map_err(|e| TransportDispatcherError::UnderlyingTransport(e.into()))
//     }
// 
//     async fn recv(
//         &self,
//     ) -> Result<ReceivedMessage<SerializedMessage, Self::Identity>, Self::Error> {
//         Ok(self.rx_topic_channel.recv().await)
//     }
// }
// 
// /// Implementation of a simple unbounded mpsc channel with interior mutability (unlike tokio's..., std's is blocking)
// struct VecDequeChannel<T> {
//     notify: Notify,
//     values: Mutex<VecDeque<T>>,
// }
// 
// impl<T> VecDequeChannel<T> {
//     pub fn new() -> Self {
//         Self {
//             notify: Notify::new(),
//             values: Mutex::new(VecDeque::new()),
//         }
//     }
// 
//     pub fn send(&self, v: T) {
//         self.values.lock().unwrap().push_back(v);
//         self.notify.notify_one();
//     }
// 
//     pub async fn recv(&self) -> T {
//         loop {
//             if let Some(value) = self.values.lock().unwrap().pop_front() {
//                 return value;
//             }
// 
//             self.notify.notified().await;
//         }
//     }
// }
// 
// fn get_or_insert_topic_channel(
//     topic: String,
//     topic_channels: &TopicChannelsMap,
// ) -> SharedChannelMap {
//     let mut topic_channels = topic_channels.lock().unwrap();
// 
//     match topic_channels.entry(topic.clone()) {
//         Entry::Occupied(o) => o.get().clone(),
//         Entry::Vacant(v) => v.insert(Arc::new(VecDequeChannel::new())).clone(),
//     }
// }
