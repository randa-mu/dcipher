//! Some custom alloy fillers

use alloy::network::Network;
use alloy::providers::fillers::{FillerControlFlow, GasFillable, GasFiller, TxFiller};
use alloy::providers::{Provider, SendableTx};
use alloy::transports::{TransportErrorKind, TransportResult};

/// A filler that fetches the gas cost using a [`GasFiller`], and adds a constant buffer to it.
#[derive(Clone, Copy, Debug)]
pub struct GasBufferFiller {
    buffer_percentage: u16,
    gas_filler: GasFiller,
}

#[derive(thiserror::Error, Debug)]
pub enum GasBufferFillerError {
    #[error("integer overflow")]
    IntegerOverflow,
}

impl Default for GasBufferFiller {
    fn default() -> Self {
        Self {
            buffer_percentage: 100,
            gas_filler: GasFiller,
        }
    }
}

impl GasBufferFiller {
    pub fn new(buffer_percentage: u16) -> Self {
        Self {
            buffer_percentage,
            gas_filler: GasFiller,
        }
    }
}

impl<N: Network> TxFiller<N> for GasBufferFiller {
    type Fillable = GasFillable;

    fn status(&self, tx: &<N as Network>::TransactionRequest) -> FillerControlFlow {
        TxFiller::<N>::status(&self.gas_filler, tx)
    }

    fn fill_sync(&self, _tx: &mut SendableTx<N>) {}

    async fn prepare<P>(
        &self,
        provider: &P,
        tx: &<N as Network>::TransactionRequest,
    ) -> TransportResult<Self::Fillable>
    where
        P: Provider<N>,
    {
        let mut estimate = TxFiller::<N>::prepare(&self.gas_filler, provider, tx).await?;
        match &mut estimate {
            GasFillable::Legacy { gas_limit, .. } | GasFillable::Eip1559 { gas_limit, .. } => {
                // gas_limit = gas_limit * percentage / 100
                *gas_limit = gas_limit
                    .checked_mul(self.buffer_percentage as u64)
                    .ok_or_else(|| {
                        TransportErrorKind::custom(GasBufferFillerError::IntegerOverflow)
                    })?
                    / 100;
            }
        }

        Ok(estimate)
    }

    async fn fill(
        &self,
        fillable: Self::Fillable,
        tx: SendableTx<N>,
    ) -> TransportResult<SendableTx<N>> {
        TxFiller::<N>::fill(&self.gas_filler, fillable, tx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::consensus::Transaction;
    use alloy::network::Ethereum;
    use alloy::primitives::{U256, address};
    use alloy::providers::ProviderBuilder;
    use alloy::providers::fillers::{BlobGasFiller, ChainIdFiller};
    use alloy::rpc::types::TransactionRequest;

    #[tokio::test]
    async fn default_buffer_normal_gas_limit() {
        let provider = ProviderBuilder::<_, _, Ethereum>::default()
            .filler(GasBufferFiller::default())
            .filler(BlobGasFiller)
            .filler(ChainIdFiller::default())
            .with_simple_nonce_management()
            .connect_anvil_with_wallet();

        let tx_req = TransactionRequest {
            value: Some(U256::from(100)),
            to: Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into()),
            ..Default::default()
        };

        let sendable_tx = provider.fill(tx_req).await.unwrap();
        let tx = sendable_tx.as_envelope().expect("should be signed");
        assert_eq!(tx.gas_limit(), 21000);
    }

    #[tokio::test]
    async fn double_buffer_double_gas_limit() {
        let provider = ProviderBuilder::<_, _, Ethereum>::default()
            .filler(GasBufferFiller::new(200))
            .filler(BlobGasFiller)
            .filler(ChainIdFiller::default())
            .with_simple_nonce_management()
            .connect_anvil_with_wallet();

        let tx_req = TransactionRequest {
            value: Some(U256::from(100)),
            to: Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into()),
            ..Default::default()
        };

        let sendable_tx = provider.fill(tx_req).await.unwrap();
        let tx = sendable_tx.as_envelope().expect("should be signed");
        assert_eq!(tx.gas_limit(), 42000);
    }
}
