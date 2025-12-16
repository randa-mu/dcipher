//! Some custom alloy fillers

use alloy::eips::eip1559::Eip1559Estimation;
use alloy::network::Network;
use alloy::providers::fillers::{FillerControlFlow, GasFillable, GasFiller, TxFiller};
use alloy::providers::{Provider, SendableTx};
use alloy::transports::{TransportErrorKind, TransportResult};

/// A filler that fetches the gas cost using a [`GasFiller`], and adds a constant buffer to it.
#[derive(Clone, Copy, Debug)]
pub struct GasBufferFiller {
    gas_limit_buffer_percentage: u16,
    gas_price_buffer_percentage: u16,
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
            gas_limit_buffer_percentage: 100,
            gas_price_buffer_percentage: 100,
            gas_filler: GasFiller,
        }
    }
}

impl GasBufferFiller {
    pub fn new(buffer_percentage: u16) -> Self {
        Self {
            gas_limit_buffer_percentage: buffer_percentage,
            gas_filler: GasFiller,
            ..Default::default()
        }
    }

    pub fn with_gas_price_buffer(mut self, gas_price_buffer_percentage: u16) -> Self {
        self.gas_price_buffer_percentage = gas_price_buffer_percentage;
        self
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
        // value * percentage / 100
        let buffer_u128 = |value: u128, percentage: u16| {
            value
                .checked_mul(percentage as u128)
                .ok_or_else(|| TransportErrorKind::custom(GasBufferFillerError::IntegerOverflow))
                .map(|v| v / 100)
        };

        let buffer_u64 = |value: u64, percentage: u16| {
            buffer_u128(value as u128, percentage)?
                .try_into()
                .map_err(|_| TransportErrorKind::custom(GasBufferFillerError::IntegerOverflow))
        };

        let mut estimate = TxFiller::<N>::prepare(&self.gas_filler, provider, tx).await?;
        match &mut estimate {
            GasFillable::Legacy {
                gas_limit,
                gas_price,
            } => {
                *gas_limit = buffer_u64(*gas_limit, self.gas_limit_buffer_percentage)?;
                *gas_price = buffer_u128(*gas_price, self.gas_price_buffer_percentage)?;
            }
            GasFillable::Eip1559 {
                gas_limit,
                estimate:
                    Eip1559Estimation {
                        max_fee_per_gas,
                        max_priority_fee_per_gas,
                    },
            } => {
                *gas_limit = buffer_u64(*gas_limit, self.gas_limit_buffer_percentage)?;
                *max_fee_per_gas = buffer_u128(*max_fee_per_gas, self.gas_price_buffer_percentage)?;
                *max_priority_fee_per_gas =
                    buffer_u128(*max_priority_fee_per_gas, self.gas_price_buffer_percentage)?;
            }
        };

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
    async fn default_buffer_normal_gas() {
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
        assert_eq!(tx.max_fee_per_gas(), 2000000001);
        assert_eq!(tx.max_priority_fee_per_gas().expect("eip1559 tx"), 1);
    }

    #[tokio::test]
    async fn double_buffer_double_gas() {
        let provider = ProviderBuilder::<_, _, Ethereum>::default()
            .filler(GasBufferFiller::new(200).with_gas_price_buffer(200))
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
        assert_eq!(tx.gas_limit(), 21000 * 2);
        assert_eq!(tx.max_fee_per_gas(), 2000000001 * 2);
        assert_eq!(tx.max_priority_fee_per_gas().expect("eip1559 tx"), 2);
    }
}
