//! Token configuration

use alloy::primitives::Address;
use std::borrow::Cow;

/// A token with its tag and address
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Token {
    tag: TokenTag,
    address: Address,
}

/// Names of the token supported tokens
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenTag {
    RUSD,
    USDT,
    Other(Cow<'static, str>),
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
