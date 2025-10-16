use crate::BalanceView;
use crate::config::{AppConfig, Currency, NetworkMonitoringConfig};
use crate::maths::to_float;
use crate::provider::create_providers;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider};
use futures::future::join_all;
use generated::onlyswaps::erc20_faucet_token::ERC20FaucetToken::ERC20FaucetTokenInstance;
use std::collections::HashMap;

pub(crate) struct ProbeService {
    probes: Vec<Probe>,
    chains_by_id: HashMap<u64, DynProvider>,
}

impl ProbeService {
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        let probes = create_probes(&config.networks);
        let chains_by_id = create_providers(&config.networks).await?;
        Ok(Self {
            probes,
            chains_by_id,
        })
    }

    pub async fn fetch_balances(&self) -> Vec<BalanceView> {
        let mut futs = Vec::new();
        for probe in self.probes.iter() {
            let provider = self
                .chains_by_id
                .get(&probe.chain_id)
                .expect("cannot probe a chain we don't have a provider for");
            let fut = async move {
                match &probe.token {
                    TokenProbe::Native => {
                        fetch_native_balance(provider, probe.chain_id, probe.wallet).await
                    }
                    TokenProbe::Token(currency) => {
                        fetch_erc20_balance(
                            provider,
                            probe.chain_id,
                            probe.wallet,
                            currency.clone(),
                        )
                        .await
                    }
                }
            };
            futs.push(fut)
        }

        let results = join_all(futs).await;
        results.into_iter().filter_map(Result::ok).collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Probe {
    pub wallet: Address,
    pub token: TokenProbe,
    pub chain_id: u64,
}

#[derive(Debug, Clone)]
pub(crate) enum TokenProbe {
    Native,
    Token(Currency),
}

fn create_probes(networks: &Vec<NetworkMonitoringConfig>) -> Vec<Probe> {
    let mut out = Vec::new();

    for network in networks {
        for wallet_addr in &network.wallets {
            let native_probe = Probe {
                wallet: *wallet_addr,
                token: TokenProbe::Native,
                chain_id: network.chain_id,
            };
            out.push(native_probe);

            for token in &network.tokens {
                let token_probe = Probe {
                    wallet: *wallet_addr,
                    token: TokenProbe::Token(token.clone()),
                    chain_id: network.chain_id,
                };
                out.push(token_probe);
            }
        }
    }

    out
}

async fn fetch_native_balance(
    provider: &DynProvider,
    chain_id: u64,
    wallet: Address,
) -> anyhow::Result<BalanceView> {
    let balance = provider.get_balance(wallet).await?;
    let balance_float = to_float(balance, 18);

    Ok(BalanceView {
        chain_id,
        address: wallet,
        asset: "native".to_string(),
        balance: balance_float,
    })
}

async fn fetch_erc20_balance(
    provider: &DynProvider,
    chain_id: u64,
    wallet: Address,
    currency: Currency,
) -> anyhow::Result<BalanceView> {
    let token = ERC20FaucetTokenInstance::new(currency.address, provider);
    let balance = token.balanceOf(wallet).call().await?;
    let balance_float = to_float(balance, currency.decimals);

    let view = BalanceView {
        chain_id,
        address: wallet,
        asset: currency.symbol,
        balance: balance_float,
    };

    Ok(view)
}
