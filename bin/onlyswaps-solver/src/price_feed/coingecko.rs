mod api_definitions;
mod builder;

pub use builder::*;

use alloy::primitives::Address;
use itertools::Itertools;
use reqwest::Url;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CoinGeckoClient {
    client: reqwest::Client,
    endpoint: Url,
    chain_id_to_id: HashMap<u64, String>,
}

#[derive(thiserror::Error, Debug)]
pub enum CoinGeckoClientError {
    #[error("failed to parse api key into header")]
    BuildClient(#[from] reqwest::Error),
}

impl CoinGeckoClient {
    pub(crate) async fn new(
        endpoint: Url,
        headers: HeaderMap,
    ) -> Result<CoinGeckoClient, CoinGeckoClientError> {
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            endpoint,
            chain_id_to_id: HashMap::default(),
        })
    }

    async fn get_asset_platforms_list(&self) -> Result<Vec<String>, CoinGeckoClientError> {
        todo!()
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
