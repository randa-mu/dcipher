//! Various static configuration for only swaps

use crate::config::chain::ChainConfig;
use crate::config::token::TokenTag;
use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider};
use generated::onlyswaps::i_router::IRouter::IRouterInstance;
use std::collections::HashMap;
use superalloy::provider::{MultiChainProvider, MultiProvider};

pub mod chain;
pub mod token;

/// A configuration used by the only swaps client
#[derive(Clone)]
pub struct OnlySwapsClientConfig {
    /// a list of chain-specific configuration
    chains: HashMap<u64, ChainConfig>,

    /// a multichain provider that can be used to interact with chains
    provider: MultiProvider<u64>,
}

impl OnlySwapsClientConfig {
    /// Create an empty only swaps client configuration
    pub fn empty() -> Self {
        Self {
            chains: HashMap::default(),
            provider: MultiProvider::default(),
        }
    }

    /// Add a chain to the configuration alongside its rpc provider
    pub fn add_ethereum_chain(
        &mut self,
        chain_config: ChainConfig,
        provider: impl Provider<Ethereum> + 'static,
    ) {
        let chain_id = chain_config.chain_id;

        self.chains.insert(chain_id, chain_config);
        self.provider.extend([(chain_id, provider.erased())])
    }

    /// Get an ethereum provider for the specified chain_id
    pub fn get_ethereum_provider(&self, chain_id: u64) -> Option<&DynProvider<Ethereum>> {
        self.provider.get_ethereum_provider(&chain_id)
    }

    /// Get the chain configuration for the specified chain id
    pub fn get_chain_config(&self, chain_id: u64) -> Option<&ChainConfig> {
        self.chains.get(&chain_id)
    }

    /// Get the address of a supported token
    pub fn get_token_address(&self, chain_id: u64, token_tag: &TokenTag) -> Option<Address> {
        let chain_config = self.get_chain_config(chain_id)?;
        Some(*chain_config.supported_tokens.get(token_tag)?)
    }

    /// Get the address of the router on the specified chain
    pub fn get_router_address(&self, chain_id: u64) -> Option<Address> {
        Some(self.get_chain_config(chain_id)?.router_address)
    }

    /// Get a Router contract instance
    pub fn get_router_instance(&self, chain_id: u64) -> Option<IRouterInstance<&DynProvider>> {
        let router_address = self.get_router_address(chain_id)?;
        let provider = self.get_ethereum_provider(chain_id)?;

        Some(IRouterInstance::new(router_address, provider))
    }
}
