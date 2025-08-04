//! An implementation of [`Transport`] that forwards messages sent to other nodes through
//! a [`MessageWriter`].

mod transport;

// re-export transport
pub use transport::*;

use crate::{PartyIdentifier, Recipient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

/// A trait used to store messages sent to a specific recipient.
pub trait MessageWriter<ID, M>: Send + Sync
where
    ID: PartyIdentifier,
    M: Send + Sync,
{
    type Error: std::error::Error + Send + Sync + 'static;

    /// Write a message sent to a specific recipient.
    fn write(
        &self,
        msg: &M,
        to: Recipient<ID>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync;
}

/// A [`MessageWriter`] that stores messages in memory.
#[derive(Clone)]
pub struct InMemoryWriter<ID: PartyIdentifier, M> {
    messages: Arc<tokio::sync::Mutex<InMemoryEntries<ID, M>>>,
}

/// The entries of an [`InMemoryWriter`].
pub type InMemoryEntries<ID, M> = HashMap<InMemoryEntryType<ID>, Vec<InMemoryEntry<ID, M>>>;

/// The type of entries used by an [`InMemoryWriter`].
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum InMemoryEntryType<ID> {
    Broadcast,
    Direct(ID),
}

/// An entry stored inside an [`InMemoryWriter`].
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InMemoryEntry<ID: PartyIdentifier, M> {
    pub timestamp: i64,
    pub recipient: Recipient<ID>,
    pub msg: M,
}

impl<ID: PartyIdentifier, M> Default for InMemoryWriter<ID, M> {
    fn default() -> Self {
        Self {
            messages: Default::default(),
        }
    }
}

impl<ID: PartyIdentifier, M> InMemoryWriter<ID, M> {
    /// Attempts to get a mutable reference to the entries using [`Arc::get_mut`].
    /// This method should generally be used when it is known that there is a single [`InMemoryWriter`]
    /// instance remaining.
    pub fn try_get_mut(&mut self) -> Option<&mut InMemoryEntries<ID, M>> {
        let messages = Arc::get_mut(&mut self.messages);
        messages.map(|messages| messages.get_mut())
    }

    /// Locks the internal mutex to obtain a mutable reference to the [`InMemoryEntries`].
    pub async fn get_mut(&self) -> tokio::sync::MutexGuard<'_, InMemoryEntries<ID, M>> {
        self.messages.lock().await
    }

    /// Take the current [`InMemoryEntries`] from the writer.
    pub async fn take(&self) -> InMemoryEntries<ID, M> {
        std::mem::take(&mut *self.messages.lock().await)
    }
}

impl<ID, M> MessageWriter<ID, M> for InMemoryWriter<ID, M>
where
    ID: PartyIdentifier,
    M: ToOwned<Owned = M> + Send + Sync,
{
    type Error = Infallible;

    async fn write(&self, msg: &M, to: Recipient<ID>) -> Result<(), Self::Error> {
        let timestamp = chrono::Utc::now().timestamp();
        let msg = msg.to_owned();
        let key = match to {
            Recipient::AllIncludingSelf | Recipient::All => InMemoryEntryType::Broadcast,
            Recipient::Single(i) => InMemoryEntryType::Direct(i),
        };

        // push the message to the vec for this type of entry
        self.messages
            .lock()
            .await
            .entry(key)
            .or_default()
            .push(InMemoryEntry {
                timestamp,
                recipient: to,
                msg,
            });
        Ok(())
    }
}

impl<ID: PartyIdentifier, MFrom> InMemoryEntry<ID, MFrom>
where
    ID: PartyIdentifier,
{
    pub fn into_new_m<M>(self) -> InMemoryEntry<ID, M>
    where
        MFrom: Into<M>,
    {
        InMemoryEntry::<ID, M> {
            msg: self.msg.into(),
            timestamp: self.timestamp,
            recipient: self.recipient,
        }
    }
}
