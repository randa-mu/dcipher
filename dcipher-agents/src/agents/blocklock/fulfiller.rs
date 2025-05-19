//! Concrete implementation of a [`TransactionFulfiller`] for the blocklock contract.
//! [`BlocklockFulfiller`] attempts to fulfil decryption requests sequentially with a transaction
//! per fulfillment.

use crate::agents::blocklock::contracts::BlocklockSender;
use crate::agents::blocklock::contracts::TypesLib::BlocklockRequest;
use crate::agents::blocklock::metrics::Metrics;
use crate::decryption_sender::SignedDecryptionRequest;
use crate::decryption_sender::contracts::DecryptionSender;
use crate::fulfiller::TransactionFulfiller;
use alloy::primitives::TxHash;
use alloy::providers::{MulticallBuilder, MulticallItem, Provider, WalletProvider};
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

    /// (available funds, request cost)
    #[error("not enough funds available to cover request: {0} < {1}")]
    InsufficientFunds(u128, u128),

    #[error("integer oveflow: {0}")]
    IntegerOverflow(&'static str),
}

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct BlocklockFulfiller<P> {
    decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<P>,
    blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<P>,
    required_confirmations: u64,
    timeout: Duration,
    gas_price_buffer_percent: u16,
    gas_buffer_percent: u16,
    profit_percent_threshold: u8,
    simulate_tx: bool,
}

impl<P> BlocklockFulfiller<P> {
    /// Creates a new instance with given parameters.
    pub fn new(
        decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<P>,
        blocklock_sender_instance: BlocklockSender::BlocklockSenderInstance<P>,
        required_confirmations: u64,
        timeout: Duration,
        gas_buffer_percent: u16,
        gas_price_buffer_percent: u16,
        profit_percent_threshold: u8,
    ) -> Self {
        Self {
            decryption_sender_instance,
            blocklock_sender_instance,
            required_confirmations,
            timeout,
            gas_buffer_percent,
            gas_price_buffer_percent,
            profit_percent_threshold,
            simulate_tx: false, // by default, do not simulate the transactions
        }
    }

    /// Allows to simulate call while never submitting transactions.
    pub fn set_simulate_tx(&mut self) {
        self.simulate_tx = true;
    }
}

impl<P> TransactionFulfiller for BlocklockFulfiller<P>
where
    P: Provider + WalletProvider + 'static,
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

#[derive(Clone, Debug)]
struct Eip1559GasEstimates {
    max_priority_fee_per_gas: u128,
    max_fee_per_gas: u128,
}

impl<P> BlocklockFulfiller<P>
where
    P: Provider,
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

        // Get gas estimates
        let tx_gas_estimates = self.estimate_eip_1559().await?;
        let estimated_gas = self
            .estimate_fulfillment_gas(&ready_request, &blocklock_request, &tx_gas_estimates)
            .await?;

        // Do not process the request if we don't make enough profits
        let max_gas = self
            .assert_profits(
                &blocklock_request,
                &tx_gas_estimates,
                estimated_gas,
                flat_fee_wei,
            )
            .await?;

        tracing::info!(
            request_id = %ready_request.id,
            max_gas,
            max_fee_per_gas = tx_gas_estimates.max_fee_per_gas,
            max_priority_fee_per_gas = tx_gas_estimates.max_priority_fee_per_gas,
            "Calculated estimated gas cost for request"
        );

        let pending_tx_or_none = if self.simulate_tx {
            // Do not send a transaction
            tracing::info!("Simulation enabled, not sending transaction");
            None
        } else {
            // Send the transaction with a custom gas cost and gas price
            let pending_tx = self
                .decryption_sender_instance
                .fulfillDecryptionRequest(
                    ready_request.id,
                    ready_request.decryption_key,
                    ready_request.signature.clone().into_owned(),
                )
                .max_fee_per_gas(tx_gas_estimates.max_fee_per_gas)
                .max_priority_fee_per_gas(tx_gas_estimates.max_priority_fee_per_gas)
                .gas(max_gas)
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

    /// Estimate amount of gas required to fulfil a decryption request.
    async fn estimate_fulfillment_gas(
        &self,
        ready_request: &SignedDecryptionRequest<'_>,
        blocklock_request: &BlocklockRequest,
        tx_gas_estimates: &Eip1559GasEstimates,
    ) -> Result<u64, BlocklockFulfillerError> {
        // Estimate gas limit for fulfillDecryptionRequest call
        let gas_estimation_call = self
            .decryption_sender_instance
            .fulfillDecryptionRequest(
                ready_request.id,
                ready_request.decryption_key.clone(),
                ready_request.signature.clone().into_owned(),
            )
            .max_priority_fee_per_gas(tx_gas_estimates.max_priority_fee_per_gas)
            .max_fee_per_gas(tx_gas_estimates.max_fee_per_gas);

        let estimated_gas = gas_estimation_call
            .clone()
            .estimate_gas()
            .await
            .map_err(|e| {
                let calldata = gas_estimation_call.calldata();
                let input = gas_estimation_call.input();
                let callback_addr = blocklock_request.callback;
                tracing::error!(
                    error = ?e,
                    signature = %ready_request.signature,
                    max_fee_per_gas = tx_gas_estimates.max_fee_per_gas,
                    max_priority_fee_per_gas = tx_gas_estimates.max_priority_fee_per_gas,
                    calldata = %calldata,
                    input = %input,
                    callback_addr = %callback_addr,
                    "Failed to simulate call to DecryptionSender::fulfillDecryptionRequest"
                );

                BlocklockFulfillerError::Contract(
                    e,
                    "failed to call DecryptionSender::fulfillDecryptionRequest",
                )
            })?;

        let estimated_gas = estimated_gas
            .checked_add(blocklock_request.callbackGasLimit.into())
            .ok_or(BlocklockFulfillerError::IntegerOverflow(
                "failed to add callback to estimated gas",
            ))?; // add the callback to the estimate

        // Add buffer to the estimate
        self.gas_with_buffer(estimated_gas)
    }

    /// Estimate the eip 1559 fees
    async fn estimate_eip_1559(&self) -> Result<Eip1559GasEstimates, BlocklockFulfillerError> {
        let fee_data = self
            .decryption_sender_instance
            .provider()
            .estimate_eip1559_fees()
            .await
            .map_err(|e| {
                BlocklockFulfillerError::RpcWithTransportErrorKind(
                    e,
                    "failed to call estimate_eip1559_fees",
                )
            })?;
        let max_fee_per_gas = self.gas_price_with_buffer(fee_data.max_fee_per_gas)?;
        let max_priority_fee_per_gas =
            self.gas_price_with_buffer(fee_data.max_priority_fee_per_gas)?;

        Ok(Eip1559GasEstimates {
            max_fee_per_gas,
            max_priority_fee_per_gas,
        })
    }

    /// Make sure that we make profit by computing gas estimates and an estimated cost against the
    /// amount paid by the user (direct funding), or the amount available in a subscription.
    async fn assert_profits(
        &self,
        blocklock_request: &BlocklockRequest,
        tx_gas_params: &Eip1559GasEstimates,
        estimated_gas: u64,
        flat_fee_wei: u128,
    ) -> Result<u64, BlocklockFulfillerError> {
        // Calculate an upper bound on the request cost
        let gas_price = tx_gas_params
            .max_priority_fee_per_gas
            .checked_add(tx_gas_params.max_fee_per_gas)
            .ok_or(BlocklockFulfillerError::IntegerOverflow(
                "failed to calculate total gas price",
            ))?;
        let request_cost_upper_bound = gas_price.checked_mul(estimated_gas.into()).ok_or(
            BlocklockFulfillerError::IntegerOverflow("failed to calculate total request cost"),
        )?;

        // Ensure that the user can cover the upper bound of the cost
        let available_funds = self.get_user_available_funds(blocklock_request).await?;
        let profit = available_funds
            .checked_sub(request_cost_upper_bound)
            .ok_or_else(|| {
                BlocklockFulfillerError::InsufficientFunds(
                    available_funds,
                    request_cost_upper_bound,
                )
            })?;
        let profit = profit.min(flat_fee_wei); // bound profit by at most flat_fee_wei
        let profit_percent =
            profit
                .checked_mul(100)
                .ok_or(BlocklockFulfillerError::IntegerOverflow(
                    "profit overflowed",
                ))?
                / flat_fee_wei; // (profit * 100) / flat_fee_wei
        if profit_percent < self.profit_percent_threshold.into() {
            tracing::warn!(
                profit_percent,
                profit_percent_threshold = self.profit_percent_threshold,
                available_funds,
                request_cost_upper_bound,
                flat_fee_wei,
                custom_gas_price = gas_price,
                max_fee_per_gas = tx_gas_params.max_fee_per_gas,
                max_priority_fee_per_gas = tx_gas_params.max_priority_fee_per_gas,
                "The amount paid by the user is lower than the profit_percent_threshold"
            );
            Err(BlocklockFulfillerError::InsufficientFunds(
                available_funds,
                request_cost_upper_bound,
            ))?
        }

        // Log all the call parameters
        tracing::debug!(
            profit_percent,
            profit_percent_threshold = self.profit_percent_threshold,
            available_funds,
            flat_fee_wei,
            request_cost_upper_bound,
            max_fee_per_gas = tx_gas_params.max_fee_per_gas,
            max_priority_fee_per_gas = tx_gas_params.max_priority_fee_per_gas,
            estimated_gas,
            "call parameters"
        );

        Ok(estimated_gas)
    }

    /// Get the current gas price from the rpc provider w/ a buffer as
    /// (gas_price * (100 + gas_price_buffer_percent)) / 100
    fn gas_price_with_buffer(&self, gas_price: u128) -> Result<u128, BlocklockFulfillerError> {
        let gas_price_mul_100 = gas_price
            .checked_mul(100u128 + u128::from(self.gas_price_buffer_percent))
            .ok_or(BlocklockFulfillerError::IntegerOverflow(
                "overflow calculating gas_price_with_buffer",
            ))?;
        Ok(gas_price_mul_100 / 100)
    }

    /// Compute the gas with a buffer as (gas * (100 + gas_buffer_percent)) / 100
    fn gas_with_buffer(&self, gas: u64) -> Result<u64, BlocklockFulfillerError> {
        let gas_mul_100 = gas
            .checked_mul(100u64 + u64::from(self.gas_buffer_percent))
            .ok_or(BlocklockFulfillerError::IntegerOverflow(
                "overflow calculating gas_with_buffer",
            ))?;
        Ok(gas_mul_100 / 100)
    }

    /// Return the current user's balance for subscription. For direct funding, return the amount
    /// paid by the user for the request.
    async fn get_user_available_funds(
        &self,
        blocklock_request: &BlocklockRequest,
    ) -> Result<u128, BlocklockFulfillerError> {
        if blocklock_request.subId.is_zero() {
            // User has already paid for the request
            blocklock_request
                .directFundingFeePaid
                .try_into()
                .map_err(|e| {
                    BlocklockFulfillerError::SolFromUint(e, "failed to cast funding fee to u128")
                })
        } else {
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

            Ok(sub.nativeBalance.try_into().expect("u96 must fit in u128"))
        }
    }
}
