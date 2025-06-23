//! A provider that can interact with multiple chains over multiple networks.

use alloy::network::{Ethereum, Network};
use alloy::providers::DynProvider;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

/// Trait allowing to obtain providers for any network and any chain.
pub trait MultiChainProvider<ChainId> {
    /// Returns `false` if the [`Network`] / `chain_id` combination is not supported.
    fn is_supported<N: Network>(&self, chain_id: &ChainId) -> bool;

    /// Try to get a provider for a specific `chain_id`, and a specific [`Network`].
    /// Returns [`None`] if the [`Network`] / `chain_id` is not supported.
    fn get_provider<N: Network>(&self, chain_id: &ChainId) -> Option<&DynProvider<N>>;

    /// Try to get an ethereum provider for a specific `chain_id`.
    /// Returns [`None`] if the `chain_id` is not supported.
    fn get_ethereum_provider(&self, chain_id: &ChainId) -> Option<&DynProvider<Ethereum>> {
        self.get_provider(chain_id)
    }
}

/// Provider for a single chain/network.
pub struct SingleProvider<N> {
    provider: DynProvider<N>,
    _n: PhantomData<N>,
}

impl<N: 'static> SingleProvider<N> {
    pub fn new(provider: DynProvider<N>) -> Self {
        Self {
            provider,
            _n: PhantomData,
        }
    }
}

impl<N: 'static> MultiChainProvider<()> for SingleProvider<N> {
    fn is_supported<PN: Network>(&self, _chain_id: &()) -> bool {
        // Verify that the provider is a DynProvider<PN>
        <dyn std::any::Any>::is::<DynProvider<PN>>(&self.provider)
    }

    fn get_provider<PN: Network>(&self, _chain_id: &()) -> Option<&DynProvider<PN>> {
        // Downcast ref ensures that N == PN, otherwise return None
        // This is required due to the highly generic nature of the MultiChainProvider trait.
        // Implementations should return a valid provider for _any_ network, but [`SingleProvider`]
        // only works for a set network, and set chain_id as `|()|` = 1.
        <dyn std::any::Any>::downcast_ref(&self.provider)
    }
}

/// A structure used to store multiple providers for multiple networks.
///
/// Due to the dyn-incompatibility of the [`Network`] trait, it's tricky to store multiple network
/// in the same data structure. We have to store the providers in a `Arc<dyn std::any::Any>` and
/// try to downcast the box into the requested concrete type when extracting the provider.
#[derive(Clone, Default)]
pub struct MultiProvider<ChainId> {
    providers: HashMap<ChainId, Arc<dyn std::any::Any>>,
}

impl<ChainId> MultiProvider<ChainId>
where
    ChainId: Eq + Hash,
{
    /// Create an empty [`MultiProvider`].
    pub fn empty() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Add providers for a specific [`N: Network`](Network).
    pub fn extend<N: Network>(
        &mut self,
        providers: impl IntoIterator<Item = (ChainId, DynProvider<N>)>,
    ) {
        self.providers.extend(
            providers.into_iter().map(|(chain_id, provider)| {
                (chain_id, Arc::new(provider) as Arc<dyn std::any::Any>)
            }),
        );
    }
}

impl<ChainId> MultiChainProvider<ChainId> for MultiProvider<ChainId>
where
    ChainId: Eq + Hash,
{
    fn is_supported<N: Network>(&self, chain_id: &ChainId) -> bool {
        self.providers
            .get(chain_id)
            .is_some_and(|provider| provider.is::<DynProvider<N>>())
    }

    fn get_provider<N: Network>(&self, chain_id: &ChainId) -> Option<&DynProvider<N>> {
        self.providers
            .get(chain_id)
            .and_then(|provider| provider.downcast_ref::<DynProvider<N>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::network::{AnyNetwork, Ethereum};
    use alloy::providers::mock::Asserter;
    use alloy::providers::{Provider, ProviderBuilder};

    #[test]
    fn single_provider() {
        let asserter = Asserter::new();
        let mock_provider: DynProvider<Ethereum> = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter)
            .erased();
        let ethereum_provider = SingleProvider::new(mock_provider);

        // Must work for ethereum
        assert!(MultiChainProvider::is_supported::<Ethereum>(
            &ethereum_provider,
            &()
        ));
        assert!(MultiChainProvider::get_provider::<Ethereum>(&ethereum_provider, &()).is_some());

        // Must not work for other networks
        assert!(!MultiChainProvider::is_supported::<AnyNetwork>(
            &ethereum_provider,
            &()
        ));
        assert!(MultiChainProvider::get_provider::<AnyNetwork>(&ethereum_provider, &()).is_none());
    }

    #[test]
    fn multi_provider_same_network() {
        let asserter = Asserter::new();
        let mock_ethereum_provider: DynProvider<Ethereum> =
            ProviderBuilder::new_with_network::<Ethereum>()
                .disable_recommended_fillers()
                .connect_mocked_client(asserter.clone())
                .erased();
        let mock_anynetwork_provider: DynProvider<AnyNetwork> =
            ProviderBuilder::new_with_network::<AnyNetwork>()
                .connect_mocked_client(asserter)
                .erased();

        let mut multi_provider = MultiProvider::default();
        multi_provider.extend([
            (1u64, mock_ethereum_provider.clone()),
            (2u64, mock_ethereum_provider),
        ]);
        multi_provider.extend([(3u64, mock_anynetwork_provider)]);

        // Must work for ethereum, chain id 1
        assert!(MultiChainProvider::is_supported::<Ethereum>(
            &multi_provider,
            &1u64
        ));
        assert!(MultiChainProvider::get_provider::<Ethereum>(&multi_provider, &1u64).is_some());

        // Must work for ethereum, chain id 2
        assert!(MultiChainProvider::is_supported::<Ethereum>(
            &multi_provider,
            &2u64
        ));
        assert!(MultiChainProvider::get_provider::<Ethereum>(&multi_provider, &2u64).is_some());

        // Must work for AnyNetwork, chain id 3
        assert!(MultiChainProvider::is_supported::<AnyNetwork>(
            &multi_provider,
            &3u64
        ));
        assert!(MultiChainProvider::get_provider::<AnyNetwork>(&multi_provider, &3u64).is_some());

        // Cannot get an Ethereum provider with chain id 3
        assert!(!MultiChainProvider::is_supported::<Ethereum>(
            &multi_provider,
            &3u64
        ));
        assert!(MultiChainProvider::get_provider::<Ethereum>(&multi_provider, &3u64).is_none());
    }
}
