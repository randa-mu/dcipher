//! Condition resolver for blocklock conditions.

mod block_reached;

use crate::condition_resolver::block_reached::BlockReachedConditionResolverError;
use crate::{BlockNumber, BlocklockCondition, BlocklockConditionDecodeError};
use block_reached::BlockReachedConditionResolver;
use std::hash::Hash;
use std::sync::atomic::AtomicBool;

pub trait ChildResolver {
    type ParentIdentifier;

    /// Mark a child as resolved.
    fn resolve(&self);

    /// Outputs the parent's identifier
    fn parent_identifier(&self) -> Self::ParentIdentifier;

    /// Verify if the parent is resolved, if so, return the parent's identifier
    fn parent_resolved(&self) -> Option<Self::ParentIdentifier>;
}

pub(crate) struct StandaloneResolver<K> {
    id: K,
    resolved: AtomicBool,
}

impl<K> StandaloneResolver<K> {
    /// Create a new standalone resolver with the status set to false
    pub(crate) fn new(id: K) -> Self {
        Self {
            id,
            resolved: false.into(),
        }
    }
}

/// Possible types of updates to blocklock conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlocklockConditionUpdate {
    BlockNumber(BlockNumber),
}

#[derive(thiserror::Error, Debug)]
pub enum BlocklockConditionResolverError {
    #[error("invalid condition")]
    BlocklockConditionDecode(#[from] BlocklockConditionDecodeError),

    #[error(transparent)]
    BlockReachedResolver(#[from] BlockReachedConditionResolverError),
}

/// Condition resolver for blocklock conditions.
pub(crate) struct BlocklockConditionResolver<ID>
where
    StandaloneResolver<ID>: ChildResolver,
{
    block_reached_condition_resolver: BlockReachedConditionResolver<StandaloneResolver<ID>>,
}

impl<ID> BlocklockConditionResolver<ID>
where
    StandaloneResolver<ID>: ChildResolver<ParentIdentifier = ID>,
{
    pub(crate) fn new() -> Self {
        Self {
            block_reached_condition_resolver: BlockReachedConditionResolver::new(),
        }
    }

    /// Add a new condition to the condition resolver.
    pub(crate) fn add_condition(
        &mut self,
        request_id: ID,
        encoded_condition: &[u8],
    ) -> Result<(), BlocklockConditionResolverError>
    where
        ID: Eq + Hash,
    {
        let condition: BlocklockCondition = encoded_condition.try_into()?;
        match condition {
            BlocklockCondition::BlockNumber(block_number) => {
                self.block_reached_condition_resolver
                    .add(block_number, StandaloneResolver::new(request_id))?;
            }
        }

        Ok(())
    }

    /// Remove a condition from the resolver.
    pub(crate) fn remove_condition(&mut self, request_id: &ID)
    where
        ID: Eq + Hash,
    {
        self.block_reached_condition_resolver.remove(request_id);
    }

    /// Update the current conditions and obtain an iterator over resolved request ids.
    pub(crate) fn update_condition<'a>(
        &'a mut self,
        update: &BlocklockConditionUpdate,
    ) -> impl Iterator<Item = ID> + 'a {
        match update {
            BlocklockConditionUpdate::BlockNumber(block_number) => {
                self.block_reached_condition_resolver.resolve(block_number)
            }
        }
    }
}

impl<K> ChildResolver for StandaloneResolver<K>
where
    K: Clone,
{
    type ParentIdentifier = K;

    fn resolve(&self) {
        self.resolved
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn parent_identifier(&self) -> Self::ParentIdentifier {
        self.id.clone()
    }

    fn parent_resolved(&self) -> Option<Self::ParentIdentifier> {
        self.resolved
            .load(std::sync::atomic::Ordering::Relaxed)
            .then(|| self.id.clone())
    }
}

impl<K> From<K> for StandaloneResolver<K> {
    fn from(id: K) -> Self {
        Self {
            id,
            resolved: false.into(),
        }
    }
}

impl From<BlockNumber> for BlocklockConditionUpdate {
    fn from(value: BlockNumber) -> Self {
        BlocklockConditionUpdate::BlockNumber(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    #[test]
    fn condition_resolver() {
        let mut condition_resolver = BlocklockConditionResolver::new();
        condition_resolver
            .add_condition(
                U256::from(1u64),
                &BlocklockCondition::BlockNumber(5u64.into()).to_bytes(),
            )
            .unwrap();
        condition_resolver
            .add_condition(
                U256::from(2u64),
                &BlocklockCondition::BlockNumber(10u64.into()).to_bytes(),
            )
            .unwrap();

        let mut ids = condition_resolver
            .update_condition(&BlocklockConditionUpdate::BlockNumber(1u64.into()));
        assert_eq!(ids.next(), None);
        drop(ids);

        let mut ids = condition_resolver
            .update_condition(&BlocklockConditionUpdate::BlockNumber(5u64.into()));
        assert_eq!(ids.next(), Some(U256::from(1u64)));
        assert_eq!(ids.next(), None);
        drop(ids);

        let mut ids = condition_resolver
            .update_condition(&BlocklockConditionUpdate::BlockNumber(11u64.into()));
        assert_eq!(ids.next(), Some(U256::from(1u64)));
        assert_eq!(ids.next(), Some(U256::from(2u64)));
        assert_eq!(ids.next(), None);
    }
}
