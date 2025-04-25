//! Concrete implementation of a [`TransactionFulfiller`] for the blocklock contract.
//! [`BlocklockFulfiller`] attempts to fulfil decryption requests sequentially with a transaction
//! per fulfillment.

use crate::agents::blocklock::contracts::BlocklockSender;
use crate::agents::blocklock::contracts::TypesLib::BlocklockRequest;
use crate::decryption_sender::SignedDecryptionRequest;
use crate::decryption_sender::contracts::DecryptionSender;
use crate::decryption_sender::contracts::TypesLib::DecryptionRequest;
use crate::fulfiller::TransactionFulfiller;
use alloy::primitives::TxHash;
use alloy::providers::{MulticallBuilder, Provider, WalletProvider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum BlocklockFulfillerError {
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
pub struct BlocklockFulfiller<P> {
    decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<(), P>,
    blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<(), P>,
    required_confirmations: u64,
    timeout: Duration,
    gas_buffer_percent: u16,
}

impl<P> BlocklockFulfiller<P> {
    /// Creates a new instance with given parameters.
    pub fn new(
        decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<(), P>,
        blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<(), P>,
        required_confirmations: u64,
        timeout: Duration,
        gas_buffer_percent: u16,
    ) -> Self {
        Self {
            decryption_sender_instance,
            blocklock_sender_instance,
            required_confirmations,
            timeout,
            gas_buffer_percent,
        }
    }
}

impl<P> TransactionFulfiller for BlocklockFulfiller<P>
where
    P: Provider + WalletProvider + 'static,
{
    type SignedRequest = SignedDecryptionRequest<'static>;
    type Error = BlocklockFulfillerError;

    fn fulfil_decryption_requests<'lt_self, 'lt_sr, I>(
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

impl<P> BlocklockFulfiller<P>
where
    P: Provider,
{
    /// Get the current gas price from the rpc provider
    async fn get_current_gas_price(&self) -> Result<u128, BlocklockFulfillerError> {
        self.decryption_sender_instance
            .provider()
            .get_gas_price()
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to obtain current gas price");
                BlocklockFulfillerError::RpcWithTransportErrorKind(e, "failed to get gas price")
            })
    }

    #[tracing::instrument(skip_all,
        fields(
            decryption_sender_addr = %self.decryption_sender_instance.address(),
            blocklock_sender_addr = %self.decryption_sender_instance.address(),
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
        // Get the complete request details
        let decryption_request = self
            .decryption_sender_instance
            .getRequest(ready_request.id)
            .call()
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to call DecryptionSender::getRequest");
                BlocklockFulfillerError::Contract(e, "failed to call DecryptionSender::getRequest")
            })?
            ._0;

        // Group 2 calls into a multicall to reduce RPC usage
        let (blocklock_request, config) = MulticallBuilder::new(
            self.blocklock_sender_instance.provider(),
        )
            // Get blocklock request details
            .add(self.blocklock_sender_instance.getRequest(ready_request.id))
            // Get flat fee from config
            .add(self.blocklock_sender_instance.getConfig())
            .aggregate()
            .await
            .map_err(|e| {
                tracing::error!(
                error = ?e,
                "Failed to call multicall(BlocklockSender::getRequest, BlocklockSender::getConfig)"
            );
                BlocklockFulfillerError::MultiCall(e)
            })?;

        // Calculate flat fee from config
        let flat_fee_wei = 1_000_000_000_000u128 * u128::from(config.fulfillmentFlatFeeNativePPM); // cannot overflow, 2**40 * 2**32

        // Get the current network price
        let current_gas_price = self.get_current_gas_price().await?;

        // Ensure that the user can cover the estimated price
        let available_for_gas = if blocklock_request._0.subId.is_zero() {
            self.available_for_gas_direct_funding(&blocklock_request._0)?
        } else {
            self.available_for_gas_subscription(
                &decryption_request,
                &blocklock_request._0,
                current_gas_price,
                flat_fee_wei,
            )
                .await?
        };

        // Estimate gas limit for fulfillDecryptionRequest call
        let estimated_gas = self
            .decryption_sender_instance
            .fulfillDecryptionRequest(
                ready_request.id,
                ready_request.decryption_key.clone(),
                ready_request.signature.clone().into_owned(),
            )
            .gas_price(current_gas_price)
            .estimate_gas()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    signature = %ready_request.signature,
                    gas_price = current_gas_price,
                    "Failed to simulate call to DecryptionSender::fulfillDecryptionRequest"
                );
                BlocklockFulfillerError::Contract(
                    e,
                    "failed to call DecryptionSender::fulfillDecryptionRequest",
                )
            })?;

        // Now, we set the gas price to the maximum price, such that our flat fee is covered
        let max_affordable_gas_price = available_for_gas / u128::from(estimated_gas);

        // Log all the parameters
        tracing::debug!(
            flat_fee_native = config.fulfillmentFlatFeeNativePPM,
            flat_fee_wei,
            estimated_gas,
            current_gas_price,
            custom_gas_price = max_affordable_gas_price,
            available_for_gas,
            "call parameters"
        );

        tracing::info!(
            request_id = %ready_request.id,
            estimated_gas,
            current_gas_price,
            custom_gas_price = max_affordable_gas_price,
            "Calculated estimated gas cost for request"
        );

        // Make sure that the max affordable gas is higher than the current gas with a buffer
        let current_gas_with_buffer =
            (current_gas_price * (100u128 + self.gas_buffer_percent as u128)) / 100u128;
        if max_affordable_gas_price < current_gas_with_buffer {
            tracing::warn!(
                max_affordable_gas_price,
                current_gas_with_buffer,
                "Maximum affordable gas price is currently insufficient"
            );
            Err(BlocklockFulfillerError::CurrentGasCostTooHigh)?
        }

        // Send the transaction with a custom gas cost and gas price
        let pending_tx = self
            .decryption_sender_instance
            .fulfillDecryptionRequest(
                ready_request.id,
                ready_request.decryption_key,
                ready_request.signature.clone().into_owned(),
            )
            .max_fee_per_gas(max_affordable_gas_price)
            .gas(estimated_gas)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    signature = %ready_request.signature,
                    "Failed to send fulfillment transaction"
                );
                BlocklockFulfillerError::Contract(
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

    /// Direct funding: ensure that the user has paid enough
    fn available_for_gas_direct_funding(
        &self,
        blocklock_request: &BlocklockRequest,
    ) -> Result<u128, BlocklockFulfillerError> {
        blocklock_request
            .directFundingFeePaid
            .try_into()
            .map_err(|e| {
                BlocklockFulfillerError::SolFromUint(e, "failed to cast direct funding fee paid")
            })
    }

    async fn available_for_gas_subscription(
        &self,
        decryption_request: &DecryptionRequest,
        blocklock_request: &BlocklockRequest,
        current_gas_price: u128,
        flat_fee_wei: u128,
    ) -> Result<u128, BlocklockFulfillerError> {
        // Get the estimated request price using the current network gas price
        let estimated_cost = self
            .blocklock_sender_instance
            .calculateRequestPriceNative(decryption_request.callbackGasLimit)
            .gas_price(current_gas_price)
            .call()
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to call BlocklockSender::calculateRequestPriceNative");
                BlocklockFulfillerError::Contract(
                    e,
                    "failed to call BlocklockSender::calculateRequestPriceNative",
                )
            })?;

        // Ensure that the estimated price allows to at least cover our flat fee
        let estimated_cost = u128::try_from(estimated_cost._0).map_err(|e| {
            BlocklockFulfillerError::SolFromUint(e, "failed to cast request price to u128")
        })?;
        let Some(available_for_gas) = estimated_cost.checked_sub(flat_fee_wei) else {
            // Request price is too low
            Err(BlocklockFulfillerError::RequestPriceTooLow)?
        };

        let sub = self
            .blocklock_sender_instance
            .getSubscription(blocklock_request.subId)
            .call()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    sub_id = %blocklock_request.subId,
                    "Failed to call BlocklockSender::getSubscription"
                );
                BlocklockFulfillerError::Contract(
                    e,
                    "failed to call BlocklockSender::getSubscription",
                )
            })?;

        let native_balance = sub.nativeBalance; // native balance in wei
        if estimated_cost > u128::try_from(native_balance).expect("u128 must hold a u96") {
            tracing::warn!(
                estimated_cost,
                subscription_balance = ?native_balance,
                subscription_id = ?blocklock_request.subId,
                "Not enough funds in subscription to cover request"
            );
            Err(BlocklockFulfillerError::SubscriptionInsufficientFunds)?
        }

        Ok(available_for_gas)
    }
}
