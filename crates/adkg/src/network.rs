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
