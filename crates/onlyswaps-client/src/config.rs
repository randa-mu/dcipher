//! Various static configuration for onlyswaps

use crate::config::chain::ChainConfig;
use crate::config::token::TokenTag;
use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider};
use std::collections::HashMap;
use superalloy::provider::{MultiChainProvider, MultiProvider};

pub mod chain;
pub mod token;

/// A configuration used by the onlyswaps client
#[derive(Clone)]
pub struct OnlySwapsClientConfig {
    /// a list of chain-specific configuration
    chains: HashMap<u64, ChainConfig>,

    /// a multichain provider that can be used to interact with chains
    provider: MultiProvider<u64>,
}

impl OnlySwapsClientConfig {
    /// Create an empty onlyswaps client configuration
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
}
