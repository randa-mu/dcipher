//! Routing configuration used by the only swaps client

use crate::config::chain::ChainConfig;
use crate::config::token::TokenTag;
use alloy::primitives::Address;

/// Parameters required to route a swap
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapRouting {
    pub src_chain: u64,
    pub dst_chain: u64,

    pub src_token: Address,
    pub dst_token: Address,
}

impl SwapRouting {
    /// Create a new route from src_chain to dst_chain, swapping to the same token _address_.
    ///
    /// # Warning
    /// You must make sure that the address of the token is the same on both chains.
    pub fn new_same_token(src_chain: u64, dst_chain: u64, token: Address) -> Self {
        Self {
            src_chain,
            dst_chain,
            src_token: token,
            dst_token: token,
        }
    }

    /// Create a new route from src_chain to dst_chain, swapping to the same token
    ///
    /// # Panics
    /// If the token is not supported by both chains
    pub fn new_same_token_from_configs(
        src_chain: &ChainConfig,
        dst_chain: &ChainConfig,
        token: &TokenTag,
    ) -> Self {
        Self {
            src_chain: src_chain.chain_id,
            dst_chain: dst_chain.chain_id,
            src_token: *src_chain
                .supported_tokens
                .get(token)
                .expect("token not supported by source chain"),
            dst_token: *dst_chain
                .supported_tokens
                .get(token)
                .expect("token not supported by destination chain"),
        }
    }
}
