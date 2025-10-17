//! Various static configuration for onlyswaps

use std::collections::HashMap;
use alloy::network::Network;
use alloy::providers::Provider;
use superalloy::provider::MultiProvider;
use crate::config::chain::ChainConfig;

pub mod chain;
pub mod token;

/// A configuration used by the onlyswaps client
pub struct OnlyswapsClientConfig {
    /// a list of chain-specific configuration
    chains: HashMap<u64, ChainConfig>,

    /// a multichain provider that can be used to interact with chains
    provider: MultiProvider<u64>,
}

impl OnlyswapsClientConfig {
    /// Create an empty onlyswaps client configuration
    pub fn new() -> Self {
        Self {
            chains: HashMap::default(),
            provider: MultiProvider::default(),
        }
    }

    /// Add a chain to the configuration alongside its rpc provider
    pub fn add_chain<N: Network>(&mut self, chain_config: ChainConfig, provider: impl Provider<N> + 'static) {
        let chain_id = chain_config.chain_id;

        self.chains.insert(chain_id, chain_config);
        self.provider.extend([(chain_id, provider.erased())])
    }


    /// Add an ethereum chain to the configuration alongside a default RPC provider
    pub fn add_chain_with_default_provider(&mut self, chain_config: ChainConfig) -> Result<(), OnlyswapsClientConfigError> {
        let chain_id = chain_config.chain_id;

        self.chains.insert(chain_id, chain_config);
        todo!("have a default RPC provider configuration")
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OnlyswapsClientConfigError {}
