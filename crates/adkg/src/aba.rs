//! Module for Asynchronous Byzantine Agreements

/// Tyler Crain's Asynchronous Byzantine Agreement described in https://arxiv.org/pdf/2002.08765.
pub mod crain20;
pub(crate) mod multi_aba;

use crate::helpers::SessionId;
use dcipher_network::topic::TopicBasedTransport;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

/// Trait used to store the configuration required by a ACSS protocol and create new instances.
pub trait AbaConfig<'a, ID>: Send + Sync + 'static {
    type Input: Send + 'static;
    type Error: std::error::Error + Send + Sync + 'static;

    // We need an explicit lifetime for the output to have a different lifetime than self
    fn new_instance<T>(
        self: &Arc<Self>,
        sid: SessionId,
        transport: T,
    ) -> Result<impl Aba<Input = Self::Input, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = ID>,
    {
        self.new_instance_with_prefix(sid, "".to_owned(), transport)
    }

    // We need an explicit lifetime for the output to have a different lifetime than self
    fn new_instance_with_prefix<T>(
        self: &Arc<Self>,
        sid: SessionId,
        topic_prefix: String,
        transport: T,
    ) -> Result<impl Aba<Input = Self::Input, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = ID>;
}

pub trait Aba: Send {
    type Input: Send + 'static;
    type Error: std::error::Error + Send + Sync + 'static;

    fn propose<RNG>(
        self,
        inputs: oneshot::Receiver<Self::Input>,
        output: oneshot::Sender<Estimate>,
        cancellation_token: CancellationToken,
        rng: &mut RNG,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        RNG: RngCore + CryptoRng + Send + Sync;
}

/// A binary estimate can either be 0/1, or \bot.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Estimate {
    Bot,
    Zero,
    One,
}
