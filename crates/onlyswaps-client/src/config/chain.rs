//! Per-chain configuration for onlyswaps

use crate::config::token::{SupportedTokenTag, Token};
use alloy::primitives::{Address, address};
use std::collections::HashMap;
use std::sync::LazyLock;

/// The configuration on a specific chain
pub struct ChainConfig {
    pub(crate) chain_id: u64,
    pub(crate) router_address: Address,
    pub(crate) supported_tokens: HashMap<SupportedTokenTag, Address>,
}

impl ChainConfig {
    pub fn new(chain_id: u64, router_address: Address, supported_tokens: HashMap<SupportedTokenTag, Address>) -> Self {
        Self {
            chain_id, router_address, supported_tokens,
        }
    }
}

pub static BASE: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 8453,
    router_address: address!("0x4cB630aAEA9e152db83A846f4509d83053F21078"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            SupportedTokenTag::RUSD,
            address!("0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"),
        )
            .into(),
        Token::new(
            SupportedTokenTag::RUSD,
            address!("0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"),
        )
            .into(),
    ]),
});

pub static BASE_SEPOLIA: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 84532,
    router_address: address!("0xC69DD549B037215BA1Ea9866FFa59603862bf986"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            SupportedTokenTag::RUSD,
            address!("0x908e1D85604E0e9e703d52D18f3f3f604Fe7Bb1b"),
        )
            .into(),
    ]),
});

pub static AVAX_C: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 43114,
    router_address: address!("0x4cB630aAEA9e152db83A846f4509d83053F21078"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            SupportedTokenTag::RUSD,
            address!("0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"),
        )
            .into(),
        Token::new(
            SupportedTokenTag::USDT,
            address!("0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7"),
        )
            .into(),
    ]),
});

pub static AVAX_FUJI: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 43113,
    router_address: address!("0xC69DD549B037215BA1Ea9866FFa59603862bf986"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            SupportedTokenTag::RUSD,
            address!("0x908e1D85604E0e9e703d52D18f3f3f604Fe7Bb1b"),
        )
            .into(),
    ]),
});
