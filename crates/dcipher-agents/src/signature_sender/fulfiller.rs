//! Concrete implementation of a [`TransactionFulfiller`] for the signature sender contract.
//! [`SignatureFulfiller`] attempts to fulfil signature requests sequentially with a transaction
//! per fulfillment.

use crate::fulfiller::TransactionFulfiller;
use crate::signature_sender::SignedSignatureRequest;
use alloy::primitives::TxHash;
use alloy::providers::utils::Eip1559Estimation;
use alloy::providers::{Provider, WalletProvider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use generated::randomness::signature_sender::SignatureSender;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum SignatureFulfillerError {
    #[error(transparent)]
    PendingTransaction(#[from] alloy::providers::PendingTransactionError),

    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),

    #[error(transparent)]
    MultiCall(#[from] alloy::providers::MulticallError),

    #[error("failed to call rpc: {1}")]
    RpcWithTransportErrorKind(
        #[source] alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
        &'static str,
    ),

    #[error("failed to cast to u128: {1}")]
    SolFromUint(
        #[source] alloy::primitives::ruint::FromUintError<u128>,
        &'static str,
    ),

    #[error("failed to cast to u128: {1}")]
    TryFromInt(#[source] std::num::TryFromIntError, &'static str),

    #[error("request price is smaller than flat fee")]
    RequestPriceTooLow,

    #[error("not enough funds in subscription to cover request")]
    SubscriptionInsufficientFunds,

    #[error("the fee to cover the request was insufficient")]
    DirectFundingInsufficientFee,

    #[error("the current gas cost is too high")]
    CurrentGasCostTooHigh,

    #[error("integer overflow: {0}")]
    IntegerOverflow(&'static str),
}

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct SignatureFulfiller<P> {
    signature_sender_instance: SignatureSender::SignatureSenderInstance<P>,
    required_confirmations: u64,
    timeout: Duration,
    gas_buffer_percent: u16,
    simulate_tx: bool,
}

impl<P> SignatureFulfiller<P> {
    /// Creates a new instance with given parameters.
    pub fn new(
        signature_sender_instance: SignatureSender::SignatureSenderInstance<P>,
        required_confirmations: u64,
        timeout: Duration,
        gas_buffer_percent: u16,
    ) -> Self {
        Self {
            signature_sender_instance,
            required_confirmations,
            timeout,
            gas_buffer_percent,
            simulate_tx: false, // by default, do not simulate the transactions
        }
    }

    /// Allows to simulate call while never submitting transactions.
    pub fn set_simulate_tx(&mut self) {
        self.simulate_tx = true;
    }
}

impl<P> TransactionFulfiller for SignatureFulfiller<P>
where
    P: Provider + WalletProvider + 'static,
{
    type SignedRequest = SignedSignatureRequest;
    type Error = SignatureFulfillerError;

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
                    let pending_tx_res = self.fulfil_signature_request(req.to_owned()).await;

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
                        tracing::error!(error = %e, request_id = %request_id, "Failed to interact with signature sender contract");
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

impl<P> SignatureFulfiller<P>
where
    P: Provider,
{
    fn with_gas_cost_buffer(&self, gas_cost: u128) -> Result<u128, SignatureFulfillerError> {
        Ok(gas_cost
            .checked_mul(100u128 + self.gas_buffer_percent as u128)
            .ok_or(SignatureFulfillerError::IntegerOverflow(
                "failed to calculate max_priority_fee_per_gas",
            ))?
            / 100u128)
    }

    fn with_gas_buffer(&self, gas: u64) -> Result<u64, SignatureFulfillerError> {
        Ok(gas
            .checked_mul(100u64 + self.gas_buffer_percent as u64)
            .ok_or(SignatureFulfillerError::IntegerOverflow(
                "failed to calculate max_priority_fee_per_gas",
            ))?
            / 100u64)
    }

    /// Get EIP-1559 estimates for maxPriorityFeePerGas and maxFeePerGas
    async fn get_eip1559_gas_estimates(
        &self,
    ) -> Result<Eip1559Estimation, SignatureFulfillerError> {
        let Eip1559Estimation {
            max_priority_fee_per_gas,
            max_fee_per_gas,
        } = self
            .signature_sender_instance
            .provider()
            .estimate_eip1559_fees()
            .await
            .map_err(|e| {
                SignatureFulfillerError::RpcWithTransportErrorKind(
                    e,
                    "failed to get eip 1559 gas estimates",
                )
            })?;

        // (max_priority_fee_per_gas * (100 + gas_buffer_percent)) / 100
        let max_priority_fee_per_gas = self.with_gas_cost_buffer(max_priority_fee_per_gas)?;
        // (max_fee_per_gas * (100 + gas_buffer_percent)) / 100
        let max_fee_per_gas = self.with_gas_cost_buffer(max_fee_per_gas)?;

        Ok(Eip1559Estimation {
            max_fee_per_gas,
            max_priority_fee_per_gas,
        })
    }

    #[tracing::instrument(skip_all,
        fields(
            signature_sender_addr = %self.signature_sender_instance.address(),
            wallet_address = %self.signature_sender_instance.provider().default_signer_address(),
            request_id = %ready_request.id
        ))
    ]
    async fn fulfil_signature_request<'a>(
        &self,
        ready_request: SignedSignatureRequest,
    ) -> Result<
        impl Future<Output = Result<TxHash, alloy::providers::PendingTransactionError>> + 'a,
        SignatureFulfillerError,
    >
    where
        P: WalletProvider,
    {
        let Eip1559Estimation {
            max_fee_per_gas,
            max_priority_fee_per_gas,
        } = self.get_eip1559_gas_estimates().await?;

        // Estimate gas limit to fulfil the call
        let gas_estimation_call = self
            .signature_sender_instance
            .fulfillSignatureRequest(ready_request.id, ready_request.signature.clone())
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas);
        let estimated_gas = gas_estimation_call
            .clone()
            .estimate_gas()
            .await
            .map_err(|e| {
                let calldata = gas_estimation_call.calldata();
                tracing::error!(
                    error = ?e,
                    signature = %ready_request.signature,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    calldata = %calldata,
                    "Failed to simulate call to SignatureSender::fulfilSignatureRequest"
                );

                SignatureFulfillerError::Contract(
                    e,
                    "failed to call DecryptionSender::fulfillDecryptionRequest",
                )
            })?;

        // Make sure that the max affordable gas is higher than the current gas with a buffer
        let estimated_gas_with_buffer = self.with_gas_buffer(estimated_gas)?;

        tracing::info!(
            request_id = %ready_request.id,
            estimated_gas,
            estimated_gas_with_buffer,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            "Calculated estimated gas cost for request"
        );

        // Send the transaction with a custom gas cost and gas price
        let pending_tx_or_none = if self.simulate_tx {
            // Do not send a transaction
            tracing::info!("Simulation enabled, not sending transaction");
            None
        } else {
            let pending_tx = self
                .signature_sender_instance
                .fulfillSignatureRequest(ready_request.id, ready_request.signature.clone())
                .max_fee_per_gas(max_fee_per_gas)
                .max_priority_fee_per_gas(max_priority_fee_per_gas)
                .gas(estimated_gas_with_buffer)
                .send()
                .await
                .map_err(|e| {
                    tracing::error!(
                        error = ?e,
                        signature = %ready_request.signature,
                        "Failed to send fulfillment transaction"
                    );
                    SignatureFulfillerError::Contract(
                        e,
                        "failed to call real DecryptionSender::fulfillDecryptionRequest",
                    )
                })?;

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
                    request_id = %ready_request.id,
                    fulfilled_block_number = receipt.block_number,
                    gas_used = receipt.gas_used,
                    gas_price = receipt.effective_gas_price,
                    "Obtained receipt for transaction"
                );
                Ok(receipt.transaction_hash)
            }
        };

        Ok(tx_hash_future)
    }
}
