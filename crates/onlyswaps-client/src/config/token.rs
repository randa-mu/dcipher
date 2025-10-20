//! Token configuration

use alloy::primitives::Address;

/// A token with its tag and address
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Token {
    tag: TokenTag,
    address: Address,
}

/// Names of the token supported tokens
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenTag {
    RUSD,
    USDT,
    Other(&'static str),
}

impl Token {
    pub fn new(tag: TokenTag, address: Address) -> Self {
        Self { tag, address }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

impl From<Token> for (TokenTag, Address) {
    fn from(value: Token) -> Self {
        (value.tag, value.address)
    }
}
