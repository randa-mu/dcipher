use crate::parsing::TransferReceipt;
use crate::pending::{RequestId, Verification, extract_pending_verifications};
use crate::signing::{ChainService, VerifiedSwap};
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use anyhow::anyhow;
use async_trait::async_trait;
use config::network::NetworkConfig;
use futures::future::{try_join, try_join_all};
use generated::onlyswaps::router::IRouter::SwapRequestParameters;
use generated::onlyswaps::router::Router::RouterInstance;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChainState<ID> {
    pub chain_id: u64,
    pub fulfilled: Vec<ID>,
    pub verified: Vec<ID>,
}

pub(crate) struct NetworkBus<P> {
    pub networks: HashMap<u64, Network<P>>,
}

pub(crate) struct Network<P> {
    chain_id: u64,
    should_write: bool,
    request_timeout: Duration,
    router: RouterInstance<P>,
}

impl NetworkBus<DynProvider> {
    pub async fn create(network_configs: &[NetworkConfig]) -> anyhow::Result<Self> {
        let mut networks = HashMap::new();

        for config in network_configs.iter() {
            let network = Network::new(config).await?;
            networks.insert(config.chain_id, network);
        }

        Ok(Self { networks })
    }

    pub async fn fetch_pending_verifications(
        &self,
    ) -> anyhow::Result<Vec<Verification<RequestId>>> {
        let futs = self.networks.values().map(|t| t.fetch_chain_state());
        let states = try_join_all(futs).await?;
        Ok(extract_pending_verifications(states))
    }
}

#[async_trait]
impl<P: Provider> ChainService for NetworkBus<P> {
    async fn fetch_transfer_receipt(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<TransferReceipt> {
        let transport = self
            .networks
            .get(&chain_id)
            .ok_or(anyhow!("No chain transport for {}", chain_id))?;

        transport.fetch_transfer_receipt(request_id).await
    }

    async fn fetch_transfer_params(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<SwapRequestParameters> {
        let transport = self
            .networks
            .get(&chain_id)
            .ok_or(anyhow!("No chain transport for {}", chain_id))?;

        transport.fetch_transfer_params(request_id).await
    }

    async fn submit_verification(
        &self,
        chain_id: u64,
        verified_swap: &VerifiedSwap,
    ) -> anyhow::Result<()> {
        let transport = self
            .networks
            .get(&chain_id)
            .ok_or(anyhow!("No chain transport for {}", chain_id))?;

        transport.submit_verified_swap(verified_swap).await
    }
}

impl Network<DynProvider> {
    pub async fn new(config: &NetworkConfig) -> anyhow::Result<Self> {
        let url = config.rpc_url.clone();
        let signer = PrivateKeySigner::from_slice(config.private_key.as_slice())?;
        let own_addr = signer.address();
        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(EthereumWallet::new(signer))
            .connect_ws(WsConnect::new(url))
            .await?
            .erased();

        tracing::info!(
            chain_id = config.chain_id,
            addr = own_addr.to_string(),
            "configured chain"
        );
        Ok(Self {
            chain_id: config.chain_id,
            should_write: config.should_write,
            request_timeout: config.request_timeout,
            router: RouterInstance::new(Address(config.router_address), provider.clone()),
        })
    }
}
impl<P: Provider> Network<P> {
    pub async fn fetch_chain_state(&self) -> anyhow::Result<ChainState<RequestId>> {
        let f = self.fetch_fulfilled_transfer_ids();
        let v = self.fetch_verified_transfer_ids();
        let (fulfilled, verified) = try_join(f, v).await?;
        Ok(ChainState {
            chain_id: self.chain_id,
            fulfilled,
            verified,
        })
    }
    pub async fn fetch_transfer_params(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<SwapRequestParameters> {
        Ok(self
            .router
            .getSwapRequestParameters(request_id)
            .call()
            .await?)
    }

    pub async fn fetch_transfer_receipt(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<TransferReceipt> {
        let receipt = self.router.getSwapRequestReceipt(request_id).call().await?;
        Ok(TransferReceipt {
            chain_id: U256::from(self.chain_id),
            request_id: receipt.requestId,
            recipient: receipt.recipient,
            src_chain_id: receipt.srcChainId,
            token: receipt.token,
            fulfilled: receipt.fulfilled,
            solver: receipt.solver,
            amount_out: receipt.amountOut,
            fulfilled_at: receipt.fulfilledAt,
        })
    }
    pub async fn fetch_fulfilled_transfer_ids(&self) -> anyhow::Result<Vec<FixedBytes<32>>> {
        Ok(self.router.getFulfilledTransfers().call().await?)
    }

    pub async fn fetch_verified_transfer_ids(&self) -> anyhow::Result<Vec<FixedBytes<32>>> {
        Ok(self.router.getFulfilledSolverRefunds().call().await?)
    }

    pub async fn submit_verified_swap(&self, verified_swap: &VerifiedSwap) -> anyhow::Result<()> {
        // nodes can be configured not to write the signature to save gas
        if !self.should_write {
            return Ok(());
        }

        match tokio::time::timeout(self.request_timeout, self.rebalance(verified_swap)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => anyhow::bail!("error submitting swap: {:?}", e),
            Err(_) => anyhow::bail!("request timed out"),
        }
    }

    async fn rebalance(&self, verified_swap: &VerifiedSwap) -> anyhow::Result<()> {
        let tx = self
            .router
            .rebalanceSolver(
                verified_swap.solver,
                verified_swap.request_id,
                Bytes::from(verified_swap.signature.clone()),
            )
            .send()
            .await?;

        let tx_hash = tx
            .with_required_confirmations(1)
            .with_timeout(Some(self.request_timeout))
            .watch()
            .await?;
        tracing::info!(tx_hash = tx_hash.to_string(), "verified swap");

        Ok(())
    }
}
