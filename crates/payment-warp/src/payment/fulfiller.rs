//! Generic fulfillment logic for dcipher payment contracts.

use crate::PaymentContract;
use crate::estimator::{
    OtherPaymentEstimatorError, PaymentEstimatorCostError, PaymentEstimatorError,
    RequestFulfillmentEstimator,
};
use alloy::contract::SolCallBuilder;
use alloy::network::{Network, ReceiptResponse};
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::{Provider, WalletProvider};
use alloy::sol_types::SolCall;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum GenericFulfillerError {
    #[error(transparent)]
    PendingTransaction(#[from] alloy::providers::PendingTransactionError),

    #[error("failed to send transaction")]
    FailedToSendTransaction(#[source] alloy::contract::Error),

    #[error(transparent)]
    OtherPaymentEstimator(#[from] OtherPaymentEstimatorError),

    #[error(transparent)]
    CostError(#[from] PaymentEstimatorCostError),
}

#[derive(Clone)]
pub struct GenericFulfiller<P, N, PC> {
    provider: P,
    contract_address: Address,
    required_confirmations: u64,
    timeout: Duration,
    payment_estimator: RequestFulfillmentEstimator<P, N, PC>,
    simulate_tx: bool,
}

impl<P, N, PC> GenericFulfiller<P, N, PC> {
    pub fn new(
        provider: P,
        contract_address: Address,
        payment_estimator: RequestFulfillmentEstimator<P, N, PC>,
        required_confirmations: u64,
        timeout: Duration,
    ) -> Self {
        Self {
            provider,
            contract_address,
            required_confirmations,
            timeout,
            payment_estimator,
            simulate_tx: false,
        }
    }

    /// Allows to simulate call while never submitting transactions.
    pub fn set_simulate_tx(&mut self) {
        self.simulate_tx = true;
    }
}

impl<P, N, PC> GenericFulfiller<P, N, PC>
where
    P: Provider<N> + WalletProvider<N>,
    N: Network,
    PC: PaymentContract<P, N>,
{
    pub async fn fulfil_calls<'lt_self, 'lt_sr, I, SC>(
        &'lt_self self,
        requests: I,
    ) -> Vec<Result<(), GenericFulfillerError>>
    where
        I: IntoIterator<Item = (U256, SC)> + Send + 'lt_self,
        I::IntoIter: Send,
        SC: SolCall,
    {
        // Evaluate the iterator and send register transactions sequentially
        let transactions = {
            let mut transactions = vec![];
            for (request_id, fulfillment_call) in requests {
                let pending_tx_res = self.fulfil_request(request_id, &fulfillment_call).await;
                transactions.push((request_id, pending_tx_res));
            }

            transactions
        };

        // We now have a vector of sent transactions (and/or errors)
        let mut transaction_results = vec![];
        for (request_id, tx) in transactions {
            // Did we fail to send the transaction?
            let pending_tx = match tx {
                Ok(pending_tx) => pending_tx,
                Err(e) => {
                    tracing::error!(error = %e, request_id = %request_id, "Failed to interact with fulfiller contract");
                    transaction_results.push(Err(e));
                    continue;
                }
            };

            // Transaction was transmitted, now wait for confirmations or timeout
            match pending_tx.await {
                Ok(tx_hash) => {
                    tracing::debug!(
                        request_id = %request_id,
                        "Transaction was successfully mined in transaction with hash: {}",
                        tx_hash
                    );
                    transaction_results.push(Ok(()));
                }
                Err(e) => {
                    tracing::error!(error = %e, request_id = %request_id, "Pending transaction failed");
                    transaction_results.push(Err(e.into()));
                }
            }
        }

        transaction_results
    }

    #[tracing::instrument(skip_all,
        fields(
            fulfillment_contract_addr = %self.contract_address,
            wallet_address = %self.provider.default_signer_address(),
            %request_id
        ))
    ]
    async fn fulfil_request<'a>(
        &self,
        request_id: U256,
        fulfillment_call: &impl SolCall,
    ) -> Result<
        impl Future<Output = Result<TxHash, alloy::providers::PendingTransactionError>> + 'a,
        GenericFulfillerError,
    > {
        let fulfillment_params = self
            .payment_estimator
            .get_fulfillment_params(request_id, fulfillment_call, &self.contract_address)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    "Failed to estimate request fulfillment parameters"
                );
                e
            })?;

        let pending_tx_or_none = if self.simulate_tx {
            // Do not send a transaction
            tracing::info!("Simulation enabled, not sending transaction");
            None
        } else {
            // Send the transaction with a custom gas cost and gas price
            let call =
                SolCallBuilder::new_sol(&self.provider, &self.contract_address, fulfillment_call)
                    .max_fee_per_gas(fulfillment_params.max_fee_per_gas)
                    .max_priority_fee_per_gas(fulfillment_params.max_priority_fee_per_gas)
                    .gas(fulfillment_params.gas_limit);

            let pending_tx = call.send().await.map_err(|e| {
                tracing::error!(
                    error = ?e,
                    calldata = %call.calldata(),
                    "Failed to send fulfillment transaction"
                );
                GenericFulfillerError::FailedToSendTransaction(e)
            })?;

            tracing::info!(tx_hash = %pending_tx.tx_hash(), "Transaction sent");
            Some(pending_tx)
        };

        let tx_hash_future = {
            let timeout = self.timeout;
            let required_confirmations = self.required_confirmations;
            async move {
                let Some(pending_tx) = pending_tx_or_none else {
                    // If we're simulating, resolve with a default TxHash
                    return Ok(TxHash::default());
                };

                let receipt = pending_tx
                    .with_required_confirmations(required_confirmations)
                    .with_timeout(Some(timeout))
                    .get_receipt()
                    .await?;

                tracing::info!(
                    request_id = %request_id,
                    fulfilled_block_number = receipt.block_number(),
                    gas_used = receipt.gas_used(),
                    gas_price = receipt.effective_gas_price(),
                    "Obtained receipt for transaction"
                );
                Ok(receipt.transaction_hash())
            }
        };

        Ok(tx_hash_future)
    }
}

impl From<PaymentEstimatorError> for GenericFulfillerError {
    fn from(value: PaymentEstimatorError) -> Self {
        match value {
            PaymentEstimatorError::Other(e) => GenericFulfillerError::OtherPaymentEstimator(e),
            PaymentEstimatorError::Cost(e) => GenericFulfillerError::CostError(e),
        }
    }
}
