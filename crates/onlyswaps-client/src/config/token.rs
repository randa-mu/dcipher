//! Token configuration

use alloy::primitives::{Address, address};

/// A token with its tag and address
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Token {
    tag: SupportedTokenTag,
    address: Address,
}

/// Names of the token supported tokens
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum SupportedTokenTag {
    RUSD,
    USDT,
}

impl Token {
    pub fn new(tag: SupportedTokenTag, address: Address) -> Self {
        Self { tag, address }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

impl From<Token> for (SupportedTokenTag, Address) {
    fn from(value: Token) -> Self {
        (value.tag, value.address)
    }
}
