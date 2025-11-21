//! Various CoinGecko api definitions.

#![allow(dead_code)]

use serde::Deserialize;
use serde_with::DefaultOnNull;
use serde_with::serde_as;
use std::collections::HashMap;

/// An entry of the Asset platforms List endpoint <https://docs.coingecko.com/reference/asset-platforms-list>.
/// We only include `id`, and `chain_identifier` here.
#[derive(Debug, Clone, Deserialize)]
pub struct AssetPlatform {
    pub id: String,
    pub chain_identifier: u64,
    pub native_coin_id: String,
}

/// The output of the Asset platforms List endpoint <https://docs.coingecko.com/reference/asset-platforms-list>.
pub type AssetPlatforms = Vec<AssetPlatform>;

/// Usd price of a token.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenPrice {
    pub usd: f64,
}

/// Output of the Coin Price by Token Addresses endpoint <https://docs.coingecko.com/reference/simple-token-price>.
pub type TokenPriceMap = HashMap<String, TokenPrice>;

/// Output of the Coin Price by Id endpoint <https://docs.coingecko.com/reference/simple-token-price>.
pub type CoinPriceMap = HashMap<String, TokenPrice>;

/// An entry of the list given by the Coins List endpoint <https://docs.coingecko.com/v3.0.1/reference/coins-list>
#[derive(Debug, Clone, Deserialize)]
pub struct CoinListEntry {
    /// the unique identifier of the coin
    pub id: String,

    /// a map of chain names to contract address
    pub platforms: HashMap<String, String>,
}

/// The entries returned by the Coins List endpoint <https://docs.coingecko.com/v3.0.1/reference/coins-list>
pub type CoinList = Vec<CoinListEntry>;

/// The output of the Coin Data by ID endpoint <https://docs.coingecko.com/v3.0.1/reference/coins-id>
#[derive(Debug, Clone, Deserialize)]
pub struct CoinData {
    pub id: String,

    /// mapping from chain id to platform detail
    pub detail_platforms: HashMap<String, PlatformDetails>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformDetails {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub decimal_place: u8,
    pub contract_address: String,
}

/// A coingecko error
#[derive(thiserror::Error, Debug, Deserialize)]
#[error("coingecko error: error_code = `{error_code}`, error_message = `{}`", status.error_message)]
pub struct CoinGeckoError {
    error_code: i128,
    status: CoinGeckoStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinGeckoStatus {
    error_message: String,
}

/// A type that either deserializes into a valid output, or an error. Basically an untagged Result,
/// where we attempt to deserialize into each variant.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CoinGeckoResult<Out> {
    Ok(Out),
    Error(CoinGeckoError),
}

impl<Out> CoinGeckoResult<Out> {
    pub fn into_result(self) -> Result<Out, CoinGeckoError> {
        match self {
            CoinGeckoResult::Ok(out) => Ok(out),
            CoinGeckoResult::Error(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_asset_platforms() {
        let response = r#"[
          {
            "id": "polygon-pos",
            "chain_identifier": 137,
            "name": "Polygon POS",
            "shortname": "MATIC",
            "native_coin_id": "matic-network",
            "image": {
              "thumb": "https://coin-images.coingecko.com/asset_platforms/images/15/thumb/polygon_pos.png?1706606645",
              "small": "https://coin-images.coingecko.com/asset_platforms/images/15/small/polygon_pos.png?1706606645",
              "large": "https://coin-images.coingecko.com/asset_platforms/images/15/large/polygon_pos.png?1706606645"
            }
          }
        ]"#;

        let asset_platforms: AssetPlatforms =
            serde_json::from_str(response).expect("to decode asset platforms response");
        assert_eq!(asset_platforms.len(), 1, "to have exactly one element");
        assert_eq!(asset_platforms[0].id, "polygon-pos");
        assert_eq!(asset_platforms[0].chain_identifier, 137);
        assert_eq!(asset_platforms[0].native_coin_id, "matic-network");
    }

    #[test]
    fn decode_coin_price_by_token_addresses() {
        let expected_usd_price = 67187.3358936566;
        let response = r#"{
          "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599": {
            "usd": 67187.3358936566,
            "usd_market_cap": 1317802988326.25,
            "usd_24h_vol": 31260929299.5248,
            "usd_24h_change": 3.63727894677354,
            "last_updated_at": 1711356300
          }
        }"#;

        let price_map: TokenPriceMap = serde_json::from_str(response)
            .expect("to decode coin price by token addresses response");
        let price = price_map
            .get("0x2260fac5e5542a773aa44fbcfedf7c193bc2c599")
            .expect("to be a valid entry");
        assert!(
            (price.usd - expected_usd_price).abs() <= 1e-06,
            "usd price to be approx equal"
        );
    }

    #[test]
    fn decode_coin_price_by_ids() {
        let expected_usd_price = 67187.3358936566;
        let response = r#"{
          "bitcoin": {
            "usd": 67187.3358936566,
            "usd_market_cap": 1317802988326.25,
            "usd_24h_vol": 31260929299.5248,
            "usd_24h_change": 3.63727894677354,
            "last_updated_at": 1711356300
          }
        }"#;

        let price_map: CoinPriceMap = serde_json::from_str(response)
            .expect("to decode coin price by token addresses response");
        let price = price_map.get("bitcoin").expect("to be a valid entry");
        assert!(
            (price.usd - expected_usd_price).abs() <= 1e-06,
            "usd price to be approx equal"
        );
    }
}
