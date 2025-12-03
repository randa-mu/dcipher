use crate::price_feed::coingecko::{CoinGeckoClient, CoinGeckoClientError};
use axum::http::HeaderMap;
use reqwest::Url;
use reqwest::header::{HeaderName, HeaderValue};
use std::str::FromStr;

const PRO_API_URL: &str = "https://pro-api.coingecko.com/api/v3/";
const PRO_API_KEY_HEADER: &str = "x-cg-pro-api-key";
const DEMO_API_URL: &str = "https://api.coingecko.com/api/v3/";
const DEMO_API_KEY_HEADER: &str = "x-cg-demo-api-key";

/// A builder for a [`CoinGeckoClient`].
pub struct CoinGeckoClientBuilder {
    api_key_header: String,
    endpoint: String,
    api_key: Option<String>,
    api_key_required: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum CoinGeckoClientBuilderError {
    #[error("failed to parse api key into header")]
    ParseApiKey,

    #[error("failed to parse coingecko url")]
    ParseUrl(#[from] url::ParseError),

    #[error("api key missing")]
    MissingApiKey,

    #[error("failed to initialize client")]
    ClientError(#[from] CoinGeckoClientError),
}

impl Default for CoinGeckoClientBuilder {
    fn default() -> Self {
        Self {
            endpoint: PRO_API_URL.to_owned(),
            api_key_header: PRO_API_KEY_HEADER.to_owned(),
            api_key: None,
            api_key_required: true,
        }
    }
}

impl CoinGeckoClientBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn use_demo_api(mut self) -> Self {
        self.endpoint = DEMO_API_URL.to_owned();
        self.api_key_header = DEMO_API_KEY_HEADER.to_owned();
        self.api_key_required = false;
        self
    }

    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn build(self) -> Result<CoinGeckoClient, CoinGeckoClientBuilderError> {
        if self.api_key_required && self.api_key.is_none() {
            // api_key unset while required, return Err
            Err(CoinGeckoClientBuilderError::ParseApiKey)?
        }

        let endpoint: Url = self.endpoint.parse()?;
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        if let Some(api_key) = self.api_key {
            let mut api_key: HeaderValue = api_key
                .parse()
                .map_err(|_| CoinGeckoClientBuilderError::ParseApiKey)?;
            api_key.set_sensitive(true);

            headers.insert(
                HeaderName::from_str(&self.api_key_header)
                    .map_err(|_| CoinGeckoClientBuilderError::ParseApiKey)?,
                api_key,
            );
        }

        let client = CoinGeckoClient::new(endpoint, headers)?;
        Ok(client)
    }
}
