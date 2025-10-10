use crate::chain_state_pending::{RequestId, Verification, extract_pending_verifications};
use crate::config::TimeoutConfig;
use crate::signing::VerifiedSwap;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use anyhow::anyhow;
use config::network::NetworkConfig;
use futures::future::{try_join, try_join_all};
use generated::onlyswaps::router::IRouter::SwapRequestParameters;
use generated::onlyswaps::router::Router::{RouterInstance, getSwapRequestReceiptReturn};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SwapStatus<ID> {
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
    router: RouterInstance<P>,
    timeout_config: TimeoutConfig,
}

impl NetworkBus<DynProvider> {
    pub async fn new(
        eth_private_key: impl Into<Arc<PrivateKeySigner>>,
        network_configs: &[NetworkConfig],
        timeout_config: &TimeoutConfig,
    ) -> anyhow::Result<Self> {
        let private_key = eth_private_key.into();
        let mut networks = HashMap::new();

        for config in network_configs.iter() {
            let network = Network::new(private_key.clone(), config, timeout_config.clone()).await?;
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
    pub(crate) async fn fetch_swap_receipt(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<getSwapRequestReceiptReturn> {
        let transport = self
            .networks
            .get(&chain_id)
            .ok_or(anyhow!("No chain transport for {}", chain_id))?;

        transport.fetch_transfer_receipt(request_id).await
    }

    pub(crate) async fn fetch_swap_params(
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

    pub(crate) async fn submit_verification(
        &self,
        verified_swap: &VerifiedSwap,
    ) -> anyhow::Result<()> {
        let chain_id: u64 = verified_swap.src_chain_id.try_into()?;
        let transport = self.networks.get(&chain_id).ok_or(anyhow!(
            "No chain transport for {}",
            verified_swap.src_chain_id
        ))?;

        transport.submit_verified_swap(verified_swap).await
    }
}

impl Network<DynProvider> {
    pub async fn new(
        signer: impl Into<Arc<PrivateKeySigner>>,
        config: &NetworkConfig,
        timeout_config: TimeoutConfig,
    ) -> anyhow::Result<Self> {
        let signer = signer.into();
        let url = config.rpc_url.clone();
        let own_addr = signer.address();
        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(EthereumWallet::new(signer.clone()))
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
            router: RouterInstance::new(Address(config.router_address), provider.clone()),
            timeout_config,
        })
    }
}
impl<P: Provider> Network<P> {
    pub async fn fetch_chain_state(&self) -> anyhow::Result<SwapStatus<RequestId>> {
        let f = self.fetch_fulfilled_transfer_ids();
        let v = self.fetch_verified_transfer_ids();
        let (fulfilled, verified) = try_join(f, v).await?;
        Ok(SwapStatus {
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
            .block(self.timeout_config.block_safety.into())
            .call()
            .await?)
    }

    pub async fn fetch_transfer_receipt(
        &self,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<getSwapRequestReceiptReturn> {
        let receipt = self
            .router
            .getSwapRequestReceipt(request_id)
            .block(self.timeout_config.block_safety.into())
            .call()
            .await?;
        Ok(receipt)
    }
    pub async fn fetch_fulfilled_transfer_ids(&self) -> anyhow::Result<Vec<FixedBytes<32>>> {
        Ok(self
            .router
            .getFulfilledTransfers()
            .block(self.timeout_config.block_safety.into())
            .call()
            .await?)
    }

    pub async fn fetch_verified_transfer_ids(&self) -> anyhow::Result<Vec<FixedBytes<32>>> {
        Ok(self
            .router
            .getFulfilledSolverRefunds()
            .block(self.timeout_config.block_safety.into())
            .call()
            .await?)
    }

    pub async fn submit_verified_swap(&self, verified_swap: &VerifiedSwap) -> anyhow::Result<()> {
        // nodes can be configured not to write the signature to save gas
        if !self.should_write {
            return Ok(());
        }

        match tokio::time::timeout(
            self.timeout_config.request_timeout,
            self.rebalance(verified_swap),
        )
        .await
        {
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
            .block(self.timeout_config.block_safety.into())
            .send()
            .await?;

        tracing::info!(
            request_id = verified_swap.request_id,
            tx_hash = tx.tx_hash().to_string(),
            "swap verification submitting"
        );
        let _ = tx
            .with_required_confirmations(1)
            .with_timeout(Some(self.timeout_config.request_timeout))
            .watch()
            .await?;

        tracing::info!(
            request_id = verified_swap.request_id,
            tx_hash = tx.tx_hash().to_string(),
            "swap verification finalised"
        );

        Ok(())
    }
}
