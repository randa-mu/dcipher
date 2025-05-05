//! Concrete implementation of a [`TransactionFulfiller`] for the signature sender contract.
//! [`SignatureFulfiller`] attempts to fulfil signature requests sequentially with a transaction
//! per fulfillment.

use crate::fulfiller::TransactionFulfiller;
use crate::signature_sender::SignedSignatureRequest;
use crate::signature_sender::contracts::SignatureSender;
use alloy::primitives::TxHash;
use alloy::providers::{Provider, WalletProvider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
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
}

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct SignatureFulfiller<P> {
    signature_sender_instance: SignatureSender::SignatureSenderInstance<P>,
    required_confirmations: u64,
    timeout: Duration,
    gas_buffer_percent: u16,
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
        }
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
    /// Get the current gas price from the rpc provider
    async fn get_current_gas_price(&self) -> Result<u128, SignatureFulfillerError> {
        self.signature_sender_instance
            .provider()
            .get_gas_price()
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to obtain current gas price");
                SignatureFulfillerError::RpcWithTransportErrorKind(e, "failed to get gas price")
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
        let current_gas_price = self.get_current_gas_price().await?;

        // Estimate gas limit to fulfil the call
        let gas_estimation_call = self
            .signature_sender_instance
            .fulfilSignatureRequest(ready_request.id, ready_request.signature.clone())
            .gas_price(current_gas_price);
        let estimated_gas = gas_estimation_call
            .clone()
            .estimate_gas()
            .await
            .map_err(|e| {
                let calldata = gas_estimation_call.calldata();
                tracing::error!(
                    error = ?e,
                    signature = %ready_request.signature,
                    gas_price = current_gas_price,
                    calldata = %calldata,
                    "Failed to simulate call to SignatureSender::fulfilSignatureRequest"
                );

                SignatureFulfillerError::Contract(
                    e,
                    "failed to call DecryptionSender::fulfillDecryptionRequest",
                )
            })?;

        // Make sure that the max affordable gas is higher than the current gas with a buffer
        let current_gas_with_buffer =
            (current_gas_price * (100u128 + self.gas_buffer_percent as u128)) / 100u128;
        let estimated_gas_with_buffer =
            (estimated_gas * (100u64 + self.gas_buffer_percent as u64)) / 100u64;

        tracing::info!(
            request_id = %ready_request.id,
            estimated_gas,
            estimated_gas_with_buffer,
            current_gas_price,
            current_gas_with_buffer,
            "Calculated estimated gas cost for request"
        );

        // Send the transaction with a custom gas cost and gas price
        let pending_tx = self
            .signature_sender_instance
            .fulfilSignatureRequest(ready_request.id, ready_request.signature.clone())
            .max_fee_per_gas(current_gas_with_buffer)
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

        let tx_hash_future = pending_tx
            .with_required_confirmations(self.required_confirmations)
            .with_timeout(Some(self.timeout))
            .get_receipt()
            .map(move |r| {
                let receipt = match r {
                    Ok(receipt) => receipt,
                    Err(e) => Err(e)?,
                };

                tracing::info!(
                    request_id = %ready_request.id,
                    fulfilled_block_number = receipt.block_number,
                    gas_used = receipt.gas_used,
                    gas_price = receipt.effective_gas_price,
                    "Obtained receipt for transaction"
                );
                Ok(receipt.transaction_hash)
            });

        Ok(tx_hash_future)
    }
}
