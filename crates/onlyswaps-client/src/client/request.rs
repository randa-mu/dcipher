//! Definitions used to specify & build swap requests.

use crate::client::routing::SwapRouting;
use crate::config::token::TokenTag;
use alloy::primitives::{Address, U256};

/// Details required to execute a swap with OnlySwaps
pub struct OnlySwapsRequest {
    pub recipient: Address,
    pub amount_out: U256,
    pub fee: U256,
    pub route: SwapRouting,
}

/// A builder for constructing an [`OnlySwapsRequest`].
///
/// All required fields
/// must be set before calling [`build`](Self::build), which returns `None` if
/// any required field is missing.
///
/// # Examples
///
/// ```ignore
/// let request = OnlySwapsRequestBuilder::new()
///     .recipient(recipient_address)
///     .amount_out(U256::from(1000))
///     .fee(U256::from(10))
///     .source_chain(1)
///     .destination_chain(137)
///     .source_token(TokenTag::USDC)
///     .build();
/// ```
#[derive(Default)]
pub struct OnlySwapsRequestBuilder {
    recipient: Option<Address>,
    amount_out: Option<U256>,
    fee: Option<U256>,
    src_chain: Option<u64>,
    dst_chain: Option<u64>,
    src_token: Option<TokenTag>,
    dst_token: Option<TokenTag>,
}

impl OnlySwapsRequestBuilder {
    /// Creates a new builder with all fields unset.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the recipient address for the swap.
    pub fn recipient(mut self, recipient: Address) -> Self {
        self.recipient = Some(recipient);
        self
    }

    /// Sets the expected output amount for the swap.
    pub fn amount_out(mut self, amount_out: U256) -> Self {
        self.amount_out = Some(amount_out);
        self
    }

    /// Sets the fee amount for the swap.
    pub fn fee(mut self, fee: U256) -> Self {
        self.fee = Some(fee);
        self
    }

    /// Sets the source chain ID.
    pub fn source_chain(mut self, src_chain: u64) -> Self {
        self.src_chain = Some(src_chain);
        self
    }

    /// Sets the destination chain ID.
    pub fn destination_chain(mut self, dst_chain: u64) -> Self {
        self.dst_chain = Some(dst_chain);
        self
    }

    /// Sets the source token tag.
    pub fn source_token(mut self, src_token: TokenTag) -> Self {
        self.src_token = Some(src_token);
        self
    }

    /// Sets the destination token tag.
    pub fn destination_token(mut self, dst_token: TokenTag) -> Self {
        self.dst_token = Some(dst_token);
        self
    }

    /// Sets routing parameters from an existing [`SwapRouting`].
    ///
    /// This is a convenience method that sets source/destination chains and tokens
    /// from a routing object in a single call.
    pub fn route(mut self, route: SwapRouting) -> Self {
        self.src_chain = Some(route.src_chain);
        self.dst_chain = Some(route.dst_chain);
        self.src_token = Some(route.src_token);
        self.dst_token = Some(route.dst_token);
        self
    }

    /// Builds an [`OnlySwapsRequest`] from the configured parameters.
    ///
    /// Returns `None` if any required field is missing (recipient, amount_out, fee,
    /// source_chain, destination_chain, or source_token).
    ///
    /// If no destination token is specified, the source token is used as the destination.
    pub fn build(self) -> Option<OnlySwapsRequest> {
        // if dst token not specified, use source token
        let src_token = self.src_token?;
        let dst_token = self.dst_token.unwrap_or(src_token);

        Some(OnlySwapsRequest {
            amount_out: self.amount_out?,
            fee: self.fee?,
            recipient: self.recipient?,
            route: SwapRouting {
                src_token,
                dst_token,
                src_chain: self.src_chain?,
                dst_chain: self.dst_chain?,
            },
        })
    }
}
