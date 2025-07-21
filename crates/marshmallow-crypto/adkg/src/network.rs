/// Generic authenticated transport trait to send and receive messages.
/// Implementations of the trait must ensure that senders can be authenticated.
use async_trait::async_trait;
use std::{fmt::Debug, time::Duration};
use tracing::info;

/// Recipient of a message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Recipient<I> {
    All,
    Single(I),
}

/// Send a message to a recipient.
#[derive(Clone, Debug)]
pub struct SendMessage<M, I> {
    pub recipient: Recipient<I>,
    pub content: M,
}

/// Message received from a sender.
#[derive(Clone, Debug)]
pub struct ReceivedMessage<M, I> {
    pub sender: I,
    pub recipient: Recipient<I>,
    pub content: M,
}

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

/// This trait allows to send and receive message through an authenticated transport channel.
#[async_trait]
pub trait AuthenticatedTransport: Send + Sync {
    type Message: Send + Sync;
    // The Identity of the message must be determined using a secure authentication mechanism such as signatures, TLS, etc.
    type Identity: Send + Sync;
    type Error: std::error::Error + Send + Sync;

    /// Send a message to a recipient. Whenever the Recipient is All, the network must also send the message to itself.
    async fn send(
        &mut self,
        msg: &SendMessage<Self::Message, Self::Identity>,
    ) -> Result<(), Self::Error>;

    /// Send a message to the recipient with a specific retry strategy.
    async fn send_retry(
        &mut self,
        msg: &SendMessage<Self::Message, Self::Identity>,
        strategy: &RetryStrategy,
    ) -> Result<(), Self::Error> {
        match strategy {
            RetryStrategy::None => self.send(msg).await,

            // Retry 0 times is the same as None
            RetryStrategy::Times { n } => {
                let mut res = Ok(());
                for i in 0..=*n {
                    res = self.send(msg).await;
                    match &res {
                        Ok(_) => break, // successfully sent, break loop
                        Err(e) => info!("Failed to send message after `{}` try.", i + 1),
                    }
                }

                res
            }

            RetryStrategy::WithLinearBackoff { n, backoff } => {
                todo!("retry with linear backoff not implemented")
            }

            RetryStrategy::WithExponentialBackoff { n, first_backoff } => {
                todo!("retry with exponential backoff not implemented")
            }
        }
    }

    /// Receive a message.
    async fn recv(&mut self)
        -> Result<ReceivedMessage<Self::Message, Self::Identity>, Self::Error>;
}

/// Mock network used for tests.
#[cfg(test)]
pub(crate) mod mock_network {
    use crate::helpers::PartyId;
    use crate::network::{AuthenticatedTransport, ReceivedMessage, Recipient, SendMessage};
    use async_trait::async_trait;
    use std::{collections::VecDeque, marker::PhantomData};
    use thiserror::Error;
    use tokio::sync::mpsc::{Receiver, Sender};
    use tracing::error;

    pub struct MockNetwork<T> {
        _t: PhantomData<T>,
    }

    impl<T> MockNetwork<T>
    where
        T: Clone + Sync + Send,
    {
        pub fn get_instances(
            n: usize,
        ) -> VecDeque<impl AuthenticatedTransport<Message = T, Identity = PartyId>> {
            use tokio::sync::mpsc;
            let (sends, mut recvs) = (0..n)
                .map(|_| mpsc::channel(64))
                .collect::<(Vec<_>, VecDeque<_>)>();

            PartyId::iter_all(n)
                .map(|i| MockNetworkInstance {
                    n,
                    node_id: i,
                    rx_channel: recvs.pop_front().unwrap(),
                    tx_channels: sends.clone(),
                })
                .collect()
        }
    }

    struct MockNetworkInstance<T> {
        n: usize,
        node_id: PartyId,
        rx_channel: Receiver<ReceivedMessage<T, PartyId>>, // one channel to receive messages
        tx_channels: Vec<Sender<ReceivedMessage<T, PartyId>>>, // n transmission channels, one per node
    }

    #[derive(Error, Debug)]
    #[error("mock network error")]
    struct MockError;

    #[async_trait]
    impl<T> AuthenticatedTransport for MockNetworkInstance<T>
    where
        T: Clone + Sync + Send,
    {
        type Message = T;
        type Identity = PartyId;
        type Error = MockError;

        async fn send(
            &mut self,
            msg: &SendMessage<Self::Message, Self::Identity>,
        ) -> Result<(), Self::Error> {
            match msg.recipient {
                Recipient::All => {
                    let mut last_error = None;
                    for i in 0..self.n {
                        let m = ReceivedMessage {
                            recipient: msg.recipient.clone(), // nodes are indexed from 1 to n
                            sender: self.node_id,
                            content: msg.content.clone(),
                        };

                        // Log error and store it
                        if let Err(e) = self.tx_channels[i].send(m).await {
                            error!(
                                "Mock network failed to send message to node `{}` during broadcast: {e:?}",
                                i + 1
                            );
                            last_error = Some(e);
                        }
                    }

                    // If there was any error during the broadcast, raise error now.
                    if let Some(e) = last_error {
                        Err(MockError)?
                    }
                }
                Recipient::Single(i) => {
                    let m = ReceivedMessage {
                        recipient: msg.recipient.clone(), // nodes are indexed from 1 to n
                        sender: self.node_id,
                        content: msg.content.clone(),
                    };

                    // Log error and store it
                    if let Err(e) = self.tx_channels[i].send(m).await {
                        error!("Mock network failed to send message to node `{}`: {e:?}", i);
                        Err(MockError)?
                    }
                }
            }

            Ok(())
        }

        async fn recv(
            &mut self,
        ) -> Result<ReceivedMessage<Self::Message, Self::Identity>, Self::Error> {
            // Wait for a new message from any of the nodes in the network.
            let msg = self.rx_channel.recv().await.ok_or(MockError)?;

            Ok(ReceivedMessage {
                sender: msg.sender,
                recipient: msg.recipient,
                content: msg.content,
            })
        }
    }
}
