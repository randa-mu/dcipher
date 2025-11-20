mod api_definitions;
mod builder;

pub use builder::*;

use crate::price_feed::TokenPriceFeed;
use crate::price_feed::coingecko::api_definitions::{
    AssetPlatform, AssetPlatforms, CoinGeckoError, CoinGeckoResult, CoinPriceMap,
};
use alloy::primitives::{Address, ChainId};
use itertools::Itertools;
use reqwest::Url;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CoinGeckoClient {
    client: reqwest::Client,
    base_url: Url,
    chain_id_to_id: HashMap<u64, AssetPlatform>,
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
            chain_id_to_id: HashMap::default(),
        })
    }

    /// Initializes an u64 chain_id to CoinGecko ids mapping by fetching the asset platform list.
    pub async fn init_chain_id_mapping(&mut self) -> Result<(), CoinGeckoClientError> {
        for asset_platform in self.get_asset_platforms_list().await? {
            self.chain_id_to_id
                .insert(asset_platform.chain_identifier, asset_platform);
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
}

impl TokenPriceFeed for CoinGeckoClient {
    type Error = CoinGeckoClientError;

    async fn native_price(&self, chain_id: ChainId) -> Result<f64, Self::Error> {
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

    async fn token_price_batched(
        &self,
        chain_id: ChainId,
        token_addresses: impl IntoIterator<Item = Address, IntoIter: Send> + Send,
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
    contract_addresses: impl IntoIterator<Item = Address>,
) -> Result<Url, url::ParseError> {
    let contract_addresses = contract_addresses
        .into_iter()
        .map(|a| a.to_string())
        .join(",");
    let mut url = base_url.join(&format!("simple/token_price/{chain_name}"))?;
    url.query_pairs_mut()
        .append_pair("vs_currencies", "usd")
        .append_pair("contract_addresses", &contract_addresses);

    Ok(url)
}
