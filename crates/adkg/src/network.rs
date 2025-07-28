// //! Generic authenticated transport trait to send and receive messages.
// //! Implementations of the trait must ensure that senders can be authenticated.
//
// pub mod dispatcher;
//
// use async_trait::async_trait;
// use serde::{Deserialize, Serialize};
// use std::sync::Arc;
use crate::helpers::PartyId;
use dcipher_network::{Recipient, TransportSender};
use std::{fmt::Debug, time::Duration};
use tracing::{debug, error, trace};
// use tracing::info;
//
// /// Recipient of a message.
// #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
// pub enum Recipient<I> {
//     All,
//     Single(I),
// }
//
// /// Send a message to a recipient.
// #[derive(Clone, Debug)]
// pub struct SendMessage<M, I> {
//     pub recipient: Recipient<I>,
//     pub content: M,
// }
//
// /// Message received from a sender.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ReceivedMessage<M, I> {
//     pub sender: I,
//     pub recipient: Recipient<I>,
//     pub content: M,
// }
//
// /// Generic bytes type used by the components of the ADKG to send and receive serialized messages.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SerializedMessage(#[serde(with = "serde_bytes")] Vec<u8>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetryStrategy {
    // Do not retry to send the message in case of failure.
    None,
    // Retry to send the message n times. Using Times with n = 0 is equivalent to None.
    Times { n: usize },
    // Retry to send the message n times with linear backoff.
    WithLinearBackoff { n: usize, backoff: Duration },
    // Retry to send the message n times with exponential backoff.
    WithExponentialBackoff { n: usize, first_backoff: Duration },
}

// #[async_trait]
// pub trait AuthenticatedTransportSerialized: AuthenticatedTransport<SerializedMessage> {
//     /// Send a serializable type through the authenticated channel.
//     /// The default implementation uses the `bson` crate for the serialization.
//     async fn send_serialize<T>(
//         &self,
//         msg: &SendMessage<T, Self::Identity>,
//         strategy: &RetryStrategy,
//     ) -> Result<(), Self::Error>
//     where
//         T: Serialize + Sync,
//         Self::Identity: Clone,
//     {
//         let ser = SerializedMessage::serialize_bson(&msg.content)?;
//         self.send_retry(
//             &SendMessage {
//                 content: ser,
//                 recipient: msg.recipient.clone(),
//             },
//             strategy,
//         )
//         .await
//     }
//
//     /// Send a serializable type through the authenticated channel.
//     /// The default implementation uses the `bson` crate for the serialization.
//     async fn recv_deserialize<T>(&self) -> Result<ReceivedMessage<T, Self::Identity>, Self::Error>
//     where
//         T: for<'de> Deserialize<'de>,
//     {
//         let received = self.recv().await?;
//         let deser = received.content.deserialize_bson()?;
//         Ok(ReceivedMessage {
//             content: deser,
//             recipient: received.recipient,
//             sender: received.sender,
//         })
//     }
// }
//
// /// Blanket implementation of AuthenticatedTransportSerialized for all types implementing AuthenticatedTransport for SerializedMessage
// #[async_trait]
// impl<T: AuthenticatedTransport<SerializedMessage>> AuthenticatedTransportSerialized for T {}
//
// // Transport trait to send and receive SerializedMessage(s) through a mutually authenticated channel.
// #[async_trait]
// pub trait AuthenticatedTransport<M>: Send + Sync + 'static
// where
//     M: Send + Sync,
// {
//     // The Identity of the message must be determined using a secure authentication mechanism such as signatures, TLS, etc.
//     type Identity: Send + Sync;
//     type Error: std::error::Error + From<bson::ser::Error> + From<bson::de::Error> + Send + Sync;
//
//     /// Send a message to a recipient. Whenever the Recipient is All, the transport must also send the message to itself.
//     async fn send(&self, msg: &SendMessage<M, Self::Identity>) -> Result<(), Self::Error>;
//
//     /// Receive messages.
//     async fn recv(&self) -> Result<ReceivedMessage<M, Self::Identity>, Self::Error>;
//
//     /// Send a message to the recipient with a specific retry strategy.
//     async fn send_retry(
//         &self,
//         msg: &SendMessage<M, Self::Identity>,
//         strategy: &RetryStrategy,
//     ) -> Result<(), Self::Error> {
//         match strategy {
//             RetryStrategy::None => self.send(msg).await,
//
//             // Retry 0 times is the same as None
//             RetryStrategy::Times { n } => {
//                 let mut res = Ok(());
//                 for i in 0..=*n {
//                     res = self.send(msg).await;
//                     match &res {
//                         Ok(_) => break, // successfully sent, break loop
//                         Err(_) => info!("Failed to send message after `{}` try.", i + 1),
//                     }
//                 }
//
//                 res
//             }
//
//             RetryStrategy::WithLinearBackoff { .. } => {
//                 todo!("retry with linear backoff not implemented")
//             }
//
//             RetryStrategy::WithExponentialBackoff { .. } => {
//                 todo!("retry with exponential backoff not implemented")
//             }
//         }
//     }
// }
//
// /// This trait allows to send messages through an authenticated transport channel and subscribe to topics.
// #[async_trait]
// pub trait AuthenticatedTopicTransport: Send + Sync {
//     // The Identity of the message must be determined using a secure authentication mechanism such as signatures, TLS, etc.
//     type Identity: Send + Sync;
//     type Error: std::error::Error + Send + Sync;
//
//     fn get_transport(
//         self: Arc<Self>,
//         topic: String,
//     ) -> impl AuthenticatedTransportSerialized<Identity = Self::Identity, Error = Self::Error>;
// }
//
// impl SerializedMessage {
//     pub fn serialize_bson<T>(t: &T) -> Result<Self, bson::ser::Error>
//     where
//         T: Serialize,
//     {
//         Ok(Self(bson::to_vec(t)?))
//     }
//
//     pub fn deserialize_bson<T>(&self) -> Result<T, bson::de::Error>
//     where
//         T: for<'de> Deserialize<'de>,
//     {
//         bson::from_slice(&self.0)
//     }
// }
//
// /// Mock transport used for tests.
// #[cfg(test)]
// pub(crate) mod mock_network {
//     use crate::helpers::PartyId;
//     use crate::network::{
//         dispatcher::TopicTransportDispatcher, AuthenticatedTopicTransport, AuthenticatedTransport,
//         ReceivedMessage, Recipient, SendMessage,
//     };
//     use async_trait::async_trait;
//     use std::collections::VecDeque;
//     use thiserror::Error;
//     use tokio::sync::broadcast;
//
//     #[derive(Error, Debug)]
//     #[error("memory transport error")]
//     pub enum MemoryTransportError {
//         TokioSender,
//         BsonSer(#[from] bson::ser::Error),
//         BsonDe(#[from] bson::de::Error),
//     }
//
//     pub struct MemoryNetwork {
//         n: usize,
//         dispatchers: Vec<TopicTransportDispatcher>,
//     }
//
//     struct BusMemoryTransport<M> {
//         node_id: PartyId,
//         tx_channel: broadcast::Sender<M>,
//         rx_channel: tokio::sync::Mutex<broadcast::Receiver<M>>, // need mutex for interior mutability + Sync
//     }
//
//     impl MemoryNetwork {
//         pub fn new(n: usize) -> Self {
//             Self {
//                 n,
//                 dispatchers: vec![],
//             }
//         }
//
//         /// Starts n individual dispatchers used to dispatch messages from BusMemoryTransports
//         pub fn start(&mut self) -> VecDeque<impl AuthenticatedTopicTransport<Identity = PartyId>> {
//             let (tx, _) = broadcast::channel(4096);
//
//             let (dispatchers, topic_transports) = PartyId::iter_all(self.n)
//                 .map(|i| {
//                     let tx_channel = tx.clone();
//                     let rx_channel = tokio::sync::Mutex::new(tx_channel.subscribe());
//                     let memory_transport = BusMemoryTransport {
//                         node_id: i,
//                         rx_channel,
//                         tx_channel,
//                     };
//
//                     let mut transport_dispatcher = TopicTransportDispatcher::new();
//                     let topic_transport = transport_dispatcher.start(memory_transport);
//                     (transport_dispatcher, topic_transport)
//                 })
//                 .collect::<(Vec<_>, VecDeque<_>)>();
//
//             self.dispatchers = dispatchers;
//             topic_transports
//         }
//
//         /// Stop the transport.
//         pub async fn stop(self) {
//             for dispatcher in self.dispatchers.into_iter() {
//                 dispatcher.stop().await
//             }
//         }
//     }
//
//     #[async_trait]
//     impl<M> AuthenticatedTransport<M> for BusMemoryTransport<ReceivedMessage<M, PartyId>>
//     where
//         M: Clone + Send + Sync + 'static,
//     {
//         type Identity = PartyId;
//         type Error = MemoryTransportError;
//
//         async fn send(&self, msg: &SendMessage<M, Self::Identity>) -> Result<(), Self::Error> {
//             let msg = ReceivedMessage {
//                 sender: self.node_id,
//                 recipient: msg.recipient,
//                 content: msg.content.clone(),
//             };
//
//             self.tx_channel
//                 .send(msg)
//                 .and(Ok(()))
//                 .map_err(|_| MemoryTransportError::TokioSender)
//         }
//
//         async fn recv(&self) -> Result<ReceivedMessage<M, Self::Identity>, Self::Error> {
//             loop {
//                 match self.rx_channel.lock().await.recv().await {
//                     Ok(m) => match m.recipient {
//                         Recipient::All => return Ok(m),
//                         Recipient::Single(i) => {
//                             if i == self.node_id {
//                                 return Ok(m);
//                             }
//                         }
//                     },
//                     Err(_) => Err(MemoryTransportError::TokioSender)?,
//                 }
//             }
//         }
//     }
// }

#[derive(thiserror::Error, Debug)]
pub(crate) enum SendSerializeError {
    #[error("transport error")]
    Transport(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to serialize bson")]
    BsonSer(#[from] bson::ser::Error),
}

pub(crate) async fn broadcast_with_self<T, M>(
    m: &M,
    retry_strategy: &RetryStrategy,
    transport: &T,
) -> Result<(), SendSerializeError>
where
    T: TransportSender<Identity = PartyId>,
    M: serde::Serialize,
{
    send_serialize_helper(m, Recipient::AllIncludingSelf, retry_strategy, transport).await
}

/// Try to send a message to other nodes.
pub(crate) async fn send_serialize_helper<T, M>(
    m: &M,
    to: Recipient<PartyId>,
    _retry_strategy: &RetryStrategy,
    transport: &T,
) -> Result<(), SendSerializeError>
where
    T: TransportSender<Identity = PartyId>,
    M: serde::Serialize,
{
    debug!("Attempting to send message to {to:?}");
    let m_vec = bson::to_vec(m)?;
    match transport.send(m_vec, to).await {
        Ok(_) => {
            trace!("Message to {to:?} sent");
            Ok(())
        }
        Err(e) => {
            error!("Failed to send message to node(s): {e:?}");
            Err(SendSerializeError::Transport(e.into()))?
        }
    }
}
