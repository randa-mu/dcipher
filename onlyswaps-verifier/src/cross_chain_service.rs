use std::collections::HashMap;
use alloy::primitives::FixedBytes;
use crate::eth::{ChainTransport, Network};
use crate::pending::{RequestId, Verification, extract_pending_verifications};
use alloy::providers::Provider;
use async_trait::async_trait;
use anyhow::anyhow;
use futures::future::try_join_all;
use crate::eth::IRouter::TransferParams;
use crate::parsing::TransferReceipt;
use crate::signing::ChainService;

pub(crate) struct CrossChainService<'a, P> {
    transports: HashMap<u64, ChainTransport<'a, P>>,
}

impl<'a, P: Provider> CrossChainService<'a, P> {
    pub fn new(networks: &'a HashMap<u64, Network<P>>) -> Self {
        let transports = networks
            .iter()
            .map(|(chain_id, network)| (*chain_id, ChainTransport::new(*chain_id, &network.router)))
            .collect::<HashMap<u64, ChainTransport<_>>>();

        Self { transports }
    }

    pub async fn fetch_pending_verifications(&self) -> anyhow::Result<Vec<Verification<RequestId>>> {
        let futs = self.transports.values().map(|t| t.fetch_chain_state());

        let states = try_join_all(futs).await?;
        Ok(extract_pending_verifications(states))
    }
}

#[async_trait]
impl<'a, P: Provider> ChainService for CrossChainService<'a, P> {
    async fn fetch_transfer_receipt(&self, chain_id: u64, request_id: FixedBytes<32>) -> anyhow::Result<TransferReceipt> {
        let transport = self.transports.get(&chain_id)
            .ok_or(anyhow!("No chain transport for {}", chain_id))?;
        
        transport.fetch_transfer_receipt(request_id).await
    }

    async fn fetch_transfer_params(&self, chain_id: u64, request_id: FixedBytes<32>) -> anyhow::Result<TransferParams> {
        let transport = self.transports.get(&chain_id)
            .ok_or(anyhow!("No chain transport for {}", chain_id))?;
        
        transport.fetch_transfer_params(request_id).await
    }
}
