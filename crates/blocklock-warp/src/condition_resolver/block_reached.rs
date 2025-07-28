//! Condition resolver for block numbers.

use crate::BlockNumber;
use crate::condition_resolver::ChildResolver;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(thiserror::Error, Debug)]
pub enum BlockReachedConditionResolverError {
    #[error("cannot add two conditions with duplicated parent identifiers")]
    DuplicatedParentIdentifier,
}

pub struct BlockReachedConditionResolver<T>
where
    T: ChildResolver,
{
    ids_block_lookup: HashMap<T::ParentIdentifier, BlockNumber>,
    items: BTreeMap<BlockNumber, HashMap<T::ParentIdentifier, T>>,
}

impl<T> BlockReachedConditionResolver<T>
where
    T: ChildResolver,
{
    pub fn new() -> Self {
        Self {
            ids_block_lookup: HashMap::new(),
            items: BTreeMap::new(),
        }
    }

    /// Add a condition with a specific block number.
    /// Returns an error if a condition with the same parent identifier already exists.
    /// Complexity: O(log(n_conditions) + log(n_blocks))
    pub fn add(
        &mut self,
        block: BlockNumber,
        item: T,
    ) -> Result<(), BlockReachedConditionResolverError>
    where
        T::ParentIdentifier: Eq + Hash,
    {
        let parent_identifier = item.parent_identifier();
        if self
            .ids_block_lookup
            .insert(parent_identifier, block)
            .is_some()
        {
            Err(BlockReachedConditionResolverError::DuplicatedParentIdentifier)?
        }

        let items = self.items.entry(block).or_default();
        items.insert(item.parent_identifier(), item);
        Ok(())
    }

    /// Remove a condition using the parent's identifier
    /// Complexity: O(log(n_conditions) + log(n_blocks) + log(d_r)) where d_r is the number
    /// of conditions for block number r
    pub fn remove(&mut self, parent_identifier: &T::ParentIdentifier) -> Option<T>
    where
        T::ParentIdentifier: Eq + Hash,
    {
        let block_number = self.ids_block_lookup.remove(parent_identifier)?;
        self.items
            .get_mut(&block_number)
            .and_then(|items| items.remove(parent_identifier))
    }

    /// Returns the list of parent identifiers that have resolved.
    /// Complexity: O(log(n_blocks) + n_conditions_in_range * O_child_condition)
    pub fn resolve<'a>(
        &'a mut self,
        current_block: &BlockNumber,
    ) -> impl Iterator<Item = T::ParentIdentifier> + 'a {
        // This would likely deserve a parallel iterator.
        self.items
            .range(..=current_block)
            .flat_map(|(_, child_conditions)| child_conditions.iter())
            .filter_map(|(_, child_condition)| {
                // First, resolve the child
                child_condition.resolve();

                // Then, check if the parent is resolved
                child_condition.parent_resolved()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition_resolver::StandaloneResolver;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn should_resolve_when_block_reached() {
        let mut resolver = BlockReachedConditionResolver::<StandaloneResolver<u64>>::new();
        resolver.add(5u64.into(), 1u64.into()).unwrap();
        resolver.add(7u64.into(), 2u64.into()).unwrap();
        resolver.add(10u64.into(), 3u64.into()).unwrap();

        let mut items = resolver.resolve(&10u64.into());
        assert_eq!(items.next(), Some(1));
        assert_eq!(items.next(), Some(2));
        assert_eq!(items.next(), Some(3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn should_not_delete_conditions_when_block_reached() {
        let mut resolver = BlockReachedConditionResolver::<StandaloneResolver<u64>>::new();
        resolver.add(5u64.into(), 1u64.into()).unwrap();
        resolver.add(7u64.into(), 2u64.into()).unwrap();
        resolver.add(10u64.into(), 3u64.into()).unwrap();

        let mut items = resolver.resolve(&10u64.into());
        assert_eq!(items.next(), Some(1));
        assert_eq!(items.next(), Some(2));
        assert_eq!(items.next(), Some(3));
        assert_eq!(items.next(), None);
        drop(items); // move items explicitly since it holds a mutable reference to resolver

        let mut items = resolver.resolve(&20u64.into());
        assert_eq!(items.next(), Some(1));
        assert_eq!(items.next(), Some(2));
        assert_eq!(items.next(), Some(3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn should_prevent_duplicated_parent_ids() {
        let mut resolver = BlockReachedConditionResolver::<StandaloneResolver<u64>>::new();
        let res = resolver.add(5u64.into(), 1.into());
        assert!(res.is_ok());

        let res = resolver.add(5u64.into(), 1.into());
        assert!(res.is_err());

        let res = resolver.add(10u64.into(), 1.into());
        assert!(res.is_err());
    }

    #[test]
    fn should_remove_condition() {
        let sub_condition_1 = 1u64.into();
        let sub_condition_2 = 2u64.into();
        let sub_condition_3 = 3u64.into();

        let mut resolver = BlockReachedConditionResolver::<StandaloneResolver<u64>>::new();
        resolver.add(5u64.into(), sub_condition_1).unwrap();
        resolver.add(7u64.into(), sub_condition_2).unwrap();
        resolver.add(10u64.into(), sub_condition_3).unwrap();

        // Remove the second condition
        let removed = resolver.remove(&2u64);
        assert!(removed.is_some());
        let StandaloneResolver { id, mut resolved } = removed.unwrap();
        assert_eq!(id, 2u64);
        assert!(!*resolved.get_mut());

        let mut items = resolver.resolve(&10u64.into());
        assert_eq!(items.next(), Some(1));
        assert_eq!(items.next(), Some(3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn should_resolve_child_only_with_parent() {
        type SharedBool = Arc<AtomicBool>;

        struct BlockNumberChildCondition {
            parent: Arc<ParentCondition>,
            resolved: SharedBool,
        }

        struct ParentCondition {
            id: u64,
            child_conditions: Vec<SharedBool>,
        }

        impl ChildResolver for BlockNumberChildCondition {
            type ParentIdentifier = u64;

            fn resolve(&self) {
                self.resolved
                    .store(true, std::sync::atomic::Ordering::Relaxed);
            }

            fn parent_identifier(&self) -> Self::ParentIdentifier {
                self.parent.id
            }

            fn parent_resolved(&self) -> Option<Self::ParentIdentifier> {
                if self
                    .parent
                    .child_conditions
                    .iter()
                    .all(|c| c.load(std::sync::atomic::Ordering::Relaxed))
                {
                    Some(self.parent.id)
                } else {
                    None
                }
            }
        }

        // Create a parents that contains two boolean values
        let parent = Arc::new(ParentCondition {
            id: 1,
            child_conditions: vec![AtomicBool::new(false).into(), AtomicBool::new(false).into()],
        });
        let child_1 = BlockNumberChildCondition {
            parent: parent.clone(),
            resolved: parent.child_conditions[0].clone(),
        };

        let mut resolver = BlockReachedConditionResolver::new();
        // we only include one child condition, and resolve the other manually
        resolver.add(5u64.into(), child_1).unwrap();

        // This should resolve the first child condition
        assert_eq!(resolver.resolve(&6u64.into()).next(), None);
        assert!(parent.child_conditions[0].load(std::sync::atomic::Ordering::Relaxed));

        // Let's now resolve the other condition
        parent.child_conditions[1].store(true, std::sync::atomic::Ordering::Relaxed);

        // Resolve should now return the parent's condition id
        assert_eq!(resolver.resolve(&7u64.into()).next(), Some(parent.id));
    }
}
