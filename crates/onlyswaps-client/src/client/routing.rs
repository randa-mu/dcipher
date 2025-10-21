//! Routing configuration used by the onlyswaps client

use crate::config::token::TokenTag;

/// Parameters required to route a swap
#[derive(Copy, Clone, Debug)]
pub struct SwapRouting {
    pub src_chain: u64,
    pub dst_chain: u64,

    pub src_token: TokenTag,
    pub dst_token: TokenTag,
}

impl SwapRouting {
    /// Create a new route from src_chain to dst_chain, swapping to the same token
    pub fn new_same_token(src_chain: u64, dst_chain: u64, token: TokenTag) -> Self {
        Self {
            src_chain,
            dst_chain,
            src_token: token,
            dst_token: token,
        }
    }
}
