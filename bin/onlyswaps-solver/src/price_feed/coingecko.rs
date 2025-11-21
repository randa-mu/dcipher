mod api_definitions;
mod builder;

pub use builder::*;

use crate::price_feed::TokenPriceFeed;
use crate::price_feed::coingecko::api_definitions::{
    AssetPlatform, AssetPlatforms, CoinData, CoinGeckoError, CoinGeckoResult, CoinList,
    CoinListEntry, CoinPriceMap,
};
use alloy::primitives::ChainId;
use itertools::Itertools;
use reqwest::Url;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::collections::HashMap;

type GCAddress = String;
type CGChainId = String;
type CGCoinId = String;

pub struct CoinGeckoClient {
    client: reqwest::Client,
    base_url: Url,

    /// A mapping from u64 chain ids to CoinGecko chain ids
    chain_id_to_id: HashMap<u64, AssetPlatform>,

    /// A mapping from token addresses to CoinGecko coin ids.
    /// We need that mapping to obtain the decimals...
    token_addr_to_details: HashMap<GCAddress, CGCoinId>,

    /// A mapping from (u64 chain id, token address) to token details.
    // We use a mutex here to fill the structure & cache on-demand
    // TODO: Not sure if it's the client responsibility, tbd
    token_details: tokio::sync::Mutex<HashMap<(CGCoinId, GCAddress), TokenDetails>>,
}

#[derive(Clone, Debug)]
struct TokenDetails {
    decimal_places: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum CoinGeckoClientError {
    #[error("failed to parse api key into header")]
    BuildClient(#[source] reqwest::Error),

    #[error("failed to parse url")]
    ParseUrl(#[from] url::ParseError),

    #[error("http error")]
    HttpError(#[source] reqwest::Error),

    #[error(transparent)]
    CoinGeckoError(#[from] CoinGeckoError),

    #[error("unsupported chain id")]
    UnsupportedChainId,

    #[error("failed to get token price from CoinGecko: invalid response")]
    CGInvalidResponse,

    #[error("token with address `{0}` not available on chain `{1}`")]
    TokenNotOnChain(GCAddress, u64),

    #[error("token with null / zero decimals")]
    NullDecimals,
}

impl CoinGeckoClient {
    pub fn builder() -> CoinGeckoClientBuilder {
        Default::default()
    }

    pub(crate) fn new(
        base_url: Url,
        headers: HeaderMap,
    ) -> Result<CoinGeckoClient, CoinGeckoClientError> {
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(CoinGeckoClientError::BuildClient)?;

        Ok(Self {
            client,
            base_url,
            chain_id_to_id: Default::default(),
            token_addr_to_details: Default::default(),
            token_details: Default::default(),
        })
    }

    /// Initializes an u64 chain_id to CoinGecko ids mapping by fetching the asset platform list.
    pub async fn init_chain_id_mapping(&mut self) -> Result<(), CoinGeckoClientError> {
        // Store a map of all platforms, i.e., chains... (~420)
        for asset_platform in self.get_asset_platforms_list().await? {
            self.chain_id_to_id
                .insert(asset_platform.chain_identifier, asset_platform);
        }

        // Store a map of all token addresses (~20k entries)
        for CoinListEntry {
            id: coin_id,
            platforms,
        } in self.get_coins_list().await?
        {
            for (_chain, address) in platforms {
                self.token_addr_to_details.insert(address, coin_id.clone());
            }
        }

        Ok(())
    }

    async fn get_coingecko<T>(&self, url: Url) -> Result<T, CoinGeckoClientError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(CoinGeckoClientError::HttpError)?;

        let out_or_err: CoinGeckoResult<_> = response
            .json()
            .await
            .map_err(CoinGeckoClientError::HttpError)?;
        Ok(out_or_err.into_result()?)
    }

    async fn get_asset_platforms_list(&self) -> Result<AssetPlatforms, CoinGeckoClientError> {
        let url = asset_platforms_url(self.base_url.clone())?;
        self.get_coingecko(url).await
    }

    async fn get_coins_list(&self) -> Result<CoinList, CoinGeckoClientError> {
        let url = coins_list_url(self.base_url.clone())?;
        self.get_coingecko(url).await
    }

    async fn get_coin_data(&self, token_address: &str) -> Result<CoinData, CoinGeckoClientError> {
        let token_id = self
            .token_addr_to_details
            .get(token_address)
            .ok_or(CoinGeckoClientError::UnsupportedChainId)?;

        let url = coins_data_url(self.base_url.clone(), token_id)?;
        self.get_coingecko(url).await
    }
}

impl TokenPriceFeed for CoinGeckoClient {
    type Error = CoinGeckoClientError;

    async fn native_value(&self, chain_id: ChainId) -> Result<f64, Self::Error> {
        let native_coin_id = self
            .chain_id_to_id
            .get(&chain_id)
            .ok_or(Self::Error::UnsupportedChainId)?
            .native_coin_id
            .clone();

        let url = coin_price_url(
            self.base_url.clone(),
            std::iter::once(native_coin_id.clone()),
        )?;
        let price: CoinPriceMap = self.get_coingecko(url).await?;

        Ok(price
            .get(&native_coin_id)
            .ok_or(Self::Error::CGInvalidResponse)?
            .usd)
    }

    async fn token_decimals(
        &self,
        chain_id: ChainId,
        token_address: String,
    ) -> Result<u8, Self::Error> {
        let chain_name = self
            .chain_id_to_id
            .get(&chain_id)
            .ok_or(Self::Error::UnsupportedChainId)?
            .id
            .clone();

        let mut token_details = self.token_details.lock().await;
        let decimals = if let Some(details) =
            token_details.get(&(chain_name.clone(), token_address.clone()))
        {
            details.decimal_places
        } else {
            let coin_data = self.get_coin_data(&token_address).await?;
            let decimals = coin_data
                .detail_platforms
                .get(&chain_name)
                .ok_or(Self::Error::TokenNotOnChain(
                    token_address.clone(),
                    chain_id,
                ))?
                .decimal_place;

            for (chain_name, details) in coin_data.detail_platforms {
                token_details.insert(
                    (chain_name, token_address.clone()),
                    TokenDetails {
                        decimal_places: details.decimal_place,
                    },
                );
            }

            decimals.try_into().map_err(|_| Self::Error::NullDecimals)?
        };

        Ok(decimals)
    }

    async fn token_price_batched(
        &self,
        chain_id: ChainId,
        token_addresses: impl IntoIterator<Item = String, IntoIter: Send> + Send,
    ) -> Result<Vec<f64>, Self::Error> {
        let chain_name = self
            .chain_id_to_id
            .get(&chain_id)
            .ok_or(Self::Error::UnsupportedChainId)?
            .id
            .clone();

        // Need to collect to Clone
        let token_addresses: Vec<_> = token_addresses.into_iter().collect();

        let url = token_price_url(self.base_url.clone(), &chain_name, token_addresses.clone())?;
        let price_map: CoinPriceMap = self.get_coingecko(url).await?;

        let prices: Option<_> = token_addresses
            .iter()
            .map(|addr| price_map.get(&addr.to_string()).map(|p| p.usd))
            .collect();
        prices.ok_or(Self::Error::CGInvalidResponse)
    }
}

fn asset_platforms_url(base_url: Url) -> Result<Url, url::ParseError> {
    base_url.join("asset_platforms")
}

fn coin_price_url(
    base_url: Url,
    coin_ids: impl IntoIterator<Item = String>,
) -> Result<Url, url::ParseError> {
    let coin_ids = coin_ids.into_iter().join(",");
    let mut url = base_url.join(&format!("simple/price"))?;
    url.query_pairs_mut()
        .append_pair("vs_currencies", "usd")
        .append_pair("ids", &coin_ids);

    Ok(url)
}

fn token_price_url(
    base_url: Url,
    chain_name: &str,
    contract_addresses: impl IntoIterator<Item = String>,
) -> Result<Url, url::ParseError> {
    let contract_addresses = contract_addresses.into_iter().join(",");
    let mut url = base_url.join(&format!("simple/token_price/{chain_name}"))?;
    url.query_pairs_mut()
        .append_pair("vs_currencies", "usd")
        .append_pair("contract_addresses", &contract_addresses);

    Ok(url)
}

fn coins_list_url(base_url: Url) -> Result<Url, url::ParseError> {
    base_url.join("coins/list")
}

fn coins_data_url(base_url: Url, coin_id: &str) -> Result<Url, url::ParseError> {
    let mut url = base_url.join(&format!("coins/{coin_id}"))?;
    url.query_pairs_mut()
        .append_pair("localization", "false")
        .append_pair("tickers", "false")
        .append_pair("market_data", "false")
        .append_pair("community_data", "false")
        .append_pair("developer_data", "false");

    Ok(url)
}
