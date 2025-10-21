//! Per-chain configuration for onlyswaps

use crate::config::token::{Token, TokenTag};
use alloy::primitives::{Address, address};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;

const TIMEOUT_DEFAULT: Duration = Duration::from_secs(30);
const REQUIRED_CONFIRMATIONS_DEFAULT: u64 = 1;

/// The configuration on a specific chain
#[derive(Clone, Debug)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub router_address: Address,
    pub supported_tokens: HashMap<TokenTag, Address>,
    pub timeout: Duration,
    pub required_confirmations: u64,
}

impl ChainConfig {
    /// Create a new chain configuration
    pub fn new(
        chain_id: u64,
        router_address: Address,
        supported_tokens: HashMap<TokenTag, Address>,
        timeout: Duration,
        required_confirmations: u64,
    ) -> Self {
        Self {
            chain_id,
            router_address,
            supported_tokens,
            timeout,
            required_confirmations,
        }
    }

    /// Obtain a chain config from a given chain id
    pub fn from_chain_id(chain_id: u64) -> Option<Self> {
        match chain_id {
            8453 => Some(BASE.clone()),
            84532 => Some(BASE_SEPOLIA.clone()),
            43114 => Some(AVAX_C.clone()),
            43113 => Some(AVAX_FUJI.clone()),
            _ => None,
        }
    }

    pub fn router_address(&self) -> Address {
        self.router_address
    }
}

pub static BASE: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 8453,
    router_address: address!("0x4cB630aAEA9e152db83A846f4509d83053F21078"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            TokenTag::RUSD,
            address!("0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"),
        )
        .into(),
        Token::new(
            TokenTag::RUSD,
            address!("0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"),
        )
        .into(),
    ]),
    timeout: TIMEOUT_DEFAULT,
    required_confirmations: REQUIRED_CONFIRMATIONS_DEFAULT,
});

pub static BASE_SEPOLIA: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 84532,
    router_address: address!("0xC69DD549B037215BA1Ea9866FFa59603862bf986"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            TokenTag::RUSD,
            address!("0x908e1D85604E0e9e703d52D18f3f3f604Fe7Bb1b"),
        )
        .into(),
    ]),
    timeout: TIMEOUT_DEFAULT,
    required_confirmations: REQUIRED_CONFIRMATIONS_DEFAULT,
});

pub static AVAX_C: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 43114,
    router_address: address!("0x4cB630aAEA9e152db83A846f4509d83053F21078"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            TokenTag::RUSD,
            address!("0x1b0F6cF6f3185872a581BD2B5a738EB52CCd4d76"),
        )
        .into(),
        Token::new(
            TokenTag::USDT,
            address!("0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7"),
        )
        .into(),
    ]),
    timeout: TIMEOUT_DEFAULT,
    required_confirmations: REQUIRED_CONFIRMATIONS_DEFAULT,
});

pub static AVAX_FUJI: LazyLock<ChainConfig> = LazyLock::new(|| ChainConfig {
    chain_id: 43113,
    router_address: address!("0xC69DD549B037215BA1Ea9866FFa59603862bf986"),
    supported_tokens: HashMap::from_iter(vec![
        Token::new(
            TokenTag::RUSD,
            address!("0x908e1D85604E0e9e703d52D18f3f3f604Fe7Bb1b"),
        )
        .into(),
    ]),
    timeout: TIMEOUT_DEFAULT,
    required_confirmations: REQUIRED_CONFIRMATIONS_DEFAULT,
});
