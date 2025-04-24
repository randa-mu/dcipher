//! Concrete implementation of a [`TransactionFulfiller`] for the blocklock contract.
//! [`BlocklockFulfiller`] attempts to fulfil decryption requests sequentially with a transaction
//! per fulfillment.

use crate::agents::blocklock::contracts::BlocklockSender;
use crate::decryption_sender::SignedDecryptionRequest;
use crate::decryption_sender::contracts::DecryptionSender;
use crate::fulfiller::TransactionFulfiller;
use alloy::primitives::TxHash;
use alloy::providers::{MulticallBuilder, Provider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum BlocklockFulfillerError {
    #[error(transparent)]
    PendingTransaction(#[from] alloy::providers::PendingTransactionError),

    #[error(transparent)]
    Contract(#[from] alloy::contract::Error),

    #[error(transparent)]
    MultiCall(#[from] alloy::providers::MulticallError),

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
}

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct BlocklockFulfiller<P> {
    decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<(), P>,
    blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<(), P>,
    required_confirmations: u64,
    timeout: Duration,
    custom_gas_price: u128,
}

impl<P> BlocklockFulfiller<P> {
    /// Creates a new instance with given parameters.
    pub fn new(
        decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<(), P>,
        blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<(), P>,
        required_confirmations: u64,
        timeout: Duration,
        custom_gas_price_wei: u64,
    ) -> Self {
        Self {
            decryption_sender_instance,
            blocklock_sender_instance,
            required_confirmations,
            timeout,
            custom_gas_price: u128::from(custom_gas_price_wei),
        }
    }
}

impl<P> TransactionFulfiller for BlocklockFulfiller<P>
where
    P: Provider + 'static,
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
    async fn fulfil_blocklock_request<'a>(
        &self,
        ready_request: SignedDecryptionRequest<'a>,
    ) -> Result<
        impl Future<Output = Result<TxHash, alloy::providers::PendingTransactionError>> + 'a,
        BlocklockFulfillerError,
    > {
        let decryption_request = self
            .decryption_sender_instance
            .getRequest(ready_request.id)
            .call()
            .await?
            ._0;

        // Group 2 calls into a multicall to reduce RPC usage
        let (blocklock_request, config) =
            MulticallBuilder::new(self.blocklock_sender_instance.provider())
                // Get blocklock request details
                .add(self.blocklock_sender_instance.getRequest(ready_request.id))
                // Get flat fee from config
                .add(self.blocklock_sender_instance.getConfig())
                .aggregate()
                .await?;

        // Get the estimated request price using our base gas fee
        let request_price = self
            .blocklock_sender_instance
            .calculateRequestPriceNative(decryption_request.callbackGasLimit)
            .gas_price(self.custom_gas_price)
            .call()
            .await?;

        // Calculate flat fee
        let flat_fee_wei = 1_000_000_000_000u128 * u128::from(config.fulfillmentFlatFeeNativePPM); // cannot overflow, 2**40 * 2**32

        // Calculate max cost base on our fee and the request price
        let request_price = u128::try_from(request_price._0).map_err(|e| {
            BlocklockFulfillerError::SolFromUint(e, "failed to cast request price to u128")
        })?;
        if request_price < flat_fee_wei {
            // Request price is too low
            Err(BlocklockFulfillerError::RequestPriceTooLow)?
        }
        let available_for_gas = request_price - flat_fee_wei;

        // Estimate gas limit for fulfillDecryptionRequest call
        let estimated_gas: u128 = self
            .decryption_sender_instance
            .fulfillDecryptionRequest(
                ready_request.id,
                ready_request.decryption_key.clone(),
                ready_request.signature.clone().into_owned(),
            )
            .gas_price(self.custom_gas_price)
            .estimate_gas()
            .await?
            .into();

        // Calculate total estimated cost in wei
        let mut estimated_cost = self.custom_gas_price * estimated_gas;

        // Log all the parameters
        tracing::debug!(
            flat_fee_native = config.fulfillmentFlatFeeNativePPM,
            flat_fee_wei,
            request_price,
            estimated_gas,
            custom_gas_price = self.custom_gas_price,
            estimated_cost,
            max_allowed_gas = available_for_gas,
            available_for_gas,
            "call parameters"
        );

        tracing::info!(
            request_id = %ready_request.id,
            estimated_gas,
            custom_gas_price = self.custom_gas_price,
            estimated_cost,
            max_allowed_gas = available_for_gas,
            "Calculated estimated gas cost for request"
        );

        // Lower the gas price if the estimated cost is too high
        let gas_price = if estimated_cost > available_for_gas {
            let max_affordable_gas_price = available_for_gas / estimated_gas;
            tracing::warn!(
                request_id = %ready_request.id,
                estimated_gas,
                custom_gas_price = self.custom_gas_price,
                reduced_gas_price = max_affordable_gas_price,
                estimated_cost,
                max_allowed_gas = available_for_gas,
                "Estimated cost too high, reducing gas cost"
            );

            // Update the estimated_cost
            estimated_cost = max_affordable_gas_price * estimated_gas;

            max_affordable_gas_price
        } else {
            self.custom_gas_price
        };

        // If the user has a subscription, ensure that they can cover the gas costs
        if !blocklock_request._0.subId.is_zero() {
            let sub = self
                .blocklock_sender_instance
                .getSubscription(blocklock_request._0.subId)
                .call()
                .await?;
            let native_balance = sub.nativeBalance; // native balance in wei
            if estimated_cost > u128::try_from(native_balance).expect("u128 must hold a u96") {
                tracing::warn!(
                    request_id = %ready_request.id,
                    estimated_cost,
                    subscription_balance = ?native_balance,
                    subscription_id = ?blocklock_request._0.subId,
                    "Not enough funds in subscription to cover request"
                );
                Err(BlocklockFulfillerError::SubscriptionInsufficientFunds)?
            }
        }

        // Send the transaction with a custom gas cost and gas price
        let pending_tx = self
            .decryption_sender_instance
            .fulfillDecryptionRequest(
                ready_request.id,
                ready_request.decryption_key,
                ready_request.signature.into_owned(),
            )
            .gas_price(gas_price)
            .gas(estimated_gas.try_into().map_err(|e| {
                BlocklockFulfillerError::TryFromInt(e, "estimated gas cost does not fit in u64")
            })?)
            .send()
            .await?;

        let tx_hash_future = pending_tx
            .with_required_confirmations(self.required_confirmations)
            .with_timeout(Some(self.timeout))
            .get_receipt()
            .map(move |r| {
                let receipt = match r {
                    Ok(receipt) => receipt,
                    Err(e) => Err(e)?,
                };

                tracing::error!(
                    request_id = %ready_request.id,
                    fulfilled_block_number = receipt.block_number,
                    gas_used = receipt.gas_used,
                    gas_price = receipt.effective_gas_price,
                    "got receipt"
                );
                Ok(receipt.transaction_hash)
            });

        Ok(tx_hash_future)
    }
}
