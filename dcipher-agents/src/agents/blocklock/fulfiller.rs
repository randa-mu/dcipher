//! Concrete implementation of a [`TransactionFulfiller`] for the blocklock contract.
//! [`BlocklockFulfiller`] attempts to fulfil decryption requests sequentially with a transaction
//! per fulfillment.

use crate::agents::blocklock::contracts::BlocklockSender;
use crate::agents::blocklock::metrics::Metrics;
use crate::agents::payment::estimator::{
    InsufficientFundsError, OtherPaymentEstimatorError, PaymentEstimatorError,
    RequestFulfillmentEstimator,
};
use crate::decryption_sender::SignedDecryptionRequest;
use crate::decryption_sender::contracts::DecryptionSender;
use crate::fulfiller::TransactionFulfiller;
use alloy::network::{Ethereum, Network, ReceiptResponse};
use alloy::primitives::TxHash;
use alloy::providers::{Provider, WalletProvider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum BlocklockFulfillerError {
    #[error(transparent)]
    PendingTransaction(#[from] alloy::providers::PendingTransactionError),

    #[error("failed to send transaction")]
    FailedToSendTransaction(#[source] alloy::contract::Error),

    #[error(transparent)]
    OtherPaymentEstimator(#[from] OtherPaymentEstimatorError),

    #[error(transparent)]
    InsufficientFunds(#[from] InsufficientFundsError),
}

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct BlocklockFulfiller<P, N = Ethereum> {
    decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<P, N>,
    blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<P, N>,
    required_confirmations: u64,
    timeout: Duration,
    payment_fulfiller:
        RequestFulfillmentEstimator<P, N, BlocklockSender::BlocklockSenderInstance<P, N>>,
    simulate_tx: bool,
}

impl<P, N> BlocklockFulfiller<P, N>
where
    P: Clone,
    N: Clone,
{
    /// Creates a new instance with given parameters.
    pub fn new(
        decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<P, N>,
        blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<P, N>,
        required_confirmations: u64,
        timeout: Duration,
        gas_buffer_percent: u16,
        gas_price_buffer_percent: u16,
        profit_percent_threshold: u8,
    ) -> Self {
        let payment_fulfiller = RequestFulfillmentEstimator::<P, N, _>::new(
            blocklock_sender_instance.clone(),
            gas_buffer_percent,
            gas_price_buffer_percent,
            profit_percent_threshold,
        );
        Self {
            decryption_sender_instance,
            blocklock_sender_instance,
            required_confirmations,
            timeout,
            payment_fulfiller,
            simulate_tx: false, // by default, do not simulate the transactions
        }
    }

    /// Allows to simulate call while never submitting transactions.
    pub fn set_simulate_tx(&mut self) {
        self.simulate_tx = true;
    }
}

impl<P, N> TransactionFulfiller for BlocklockFulfiller<P, N>
where
    P: Provider<N> + WalletProvider + 'static,
    N: Network,
{
    type SignedRequest = SignedDecryptionRequest<'static>;
    type Error = BlocklockFulfillerError;

    fn fulfil_requests<'lt_self, 'lt_sr, I>(
        &'lt_self self,
        requests: I,
    ) -> BoxFuture<'lt_self, Vec<Result<(), Self::Error>>>
    where
        I: IntoIterator<Item = &'lt_sr Self::SignedRequest> + Send + 'lt_self,
        I::IntoIter: Send,
    {
        async move {
            // Evaluate the iterator and send register transactions sequentially
            let transactions = {
                let mut transactions = vec![];
                for req in requests {
                    let request_id = req.id;
                    let pending_tx_res = self.fulfil_blocklock_request(req.to_owned()).await;

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
                        tracing::error!(error = %e, request_id = %request_id, "Failed to interact with decryption sender contract");
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
        }.boxed()
    }
}

impl<P, N> BlocklockFulfiller<P, N>
where
    P: Provider<N>,
    N: Network,
{
    #[tracing::instrument(skip_all,
        fields(
            decryption_sender_addr = %self.decryption_sender_instance.address(),
            blocklock_sender_addr = %self.blocklock_sender_instance.address(),
            wallet_address = %self.decryption_sender_instance.provider().default_signer_address(),
            request_id = %ready_request.id
        ))
    ]
    async fn fulfil_blocklock_request<'a>(
        &self,
        ready_request: SignedDecryptionRequest<'a>,
    ) -> Result<
        impl Future<Output = Result<TxHash, alloy::providers::PendingTransactionError>> + 'a,
        BlocklockFulfillerError,
    >
    where
        P: WalletProvider,
    {
        let fulfillment_call = DecryptionSender::fulfillDecryptionRequestCall {
            requestID: ready_request.id,
            decryptionKey: ready_request.decryption_key,
            signature: ready_request.signature.clone().into_owned(),
        };

        let fulfillment_params = self
            .payment_fulfiller
            .get_fulfillment_params(
                ready_request.id,
                &fulfillment_call,
                self.decryption_sender_instance.address(),
            )
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
            let pending_tx = self
                .decryption_sender_instance
                .call_builder(&fulfillment_call)
                .max_fee_per_gas(fulfillment_params.max_fee_per_gas)
                .max_priority_fee_per_gas(fulfillment_params.max_priority_fee_per_gas)
                .gas(fulfillment_params.gas_limit)
                .send()
                .await
                .map_err(|e| {
                    tracing::error!(
                        error = ?e,
                        signature = %ready_request.signature,
                        "Failed to send fulfillment transaction"
                    );
                    BlocklockFulfillerError::FailedToSendTransaction(e)
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

                Metrics::report_decryption_success();
                tracing::info!(
                    request_id = %ready_request.id,
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

impl From<PaymentEstimatorError> for BlocklockFulfillerError {
    fn from(value: PaymentEstimatorError) -> Self {
        match value {
            PaymentEstimatorError::Other(e) => BlocklockFulfillerError::OtherPaymentEstimator(e),
            PaymentEstimatorError::InsufficientFunds(e) => {
                BlocklockFulfillerError::InsufficientFunds(e)
            }
        }
    }
}
