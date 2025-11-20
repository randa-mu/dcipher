mod api_definitions;
mod builder;

pub use builder::*;

use crate::price_feed::coingecko::api_definitions::{
    AssetPlatform, AssetPlatforms, CoinGeckoError, CoinGeckoResult,
};
use alloy::primitives::Address;
use itertools::Itertools;
use reqwest::Url;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde::de::DeserializeOwned;
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
}

impl CoinGeckoClient {
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

fn asset_platforms_url(base_url: Url) -> Result<Url, url::ParseError> {
    base_url.join("asset_platforms")
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
