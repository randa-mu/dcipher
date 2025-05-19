//! Estimate gas costs to ensure that fulfilling a request results in profits.

use crate::agents::payment::{PaymentConfig, PaymentContract, RequestDetails};
use alloy::contract::SolCallBuilder;
use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::{MulticallItem, Provider};
use alloy::sol_types::SolCall;
use std::marker::PhantomData;

/// Fulfillment parameters that must be used to fulfil a request.
#[derive(Copy, Clone, Debug)]
pub struct RequestFulfillmentParams {
    pub gas_limit: u64,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: u128,
}

/// Failed to estimate the request fulfillment parameters.
#[derive(thiserror::Error, Debug)]
pub enum PaymentEstimatorError {
    /// Error not directly linked to user payments.
    #[error(transparent)]
    Other(#[from] OtherPaymentEstimatorError),

    /// The user does not have sufficient funds to allow covering the request w/ a sufficient
    /// profit margin.
    #[error(transparent)]
    InsufficientFunds(InsufficientFundsError),
}

#[derive(thiserror::Error, Debug)]
#[error("not enough funds available to cover request: {available_funds} < {request_cost}")]
pub struct InsufficientFundsError {
    pub available_funds: u128,
    pub request_cost: u128,
}

/// Other errors, generally due to an issue while estimating the costs.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum OtherPaymentEstimatorError {
    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),

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

    #[error("integer oveflow: {0}")]
    IntegerOverflow(&'static str),
}

/// Struct used to estimate gas costs of a request and ensuring that a set profit threshold is met.
#[derive(Clone)]
pub struct RequestFulfillmentEstimator<P, N, PC> {
    gas_price_buffer_percent: u16,
    payment_contract: PC,
    gas_buffer_percent: u16,
    profit_percent_threshold: u8,
    _p: PhantomData<fn(P) -> P>,
    _n: PhantomData<fn(N) -> N>,
}

impl<P, N, PC> RequestFulfillmentEstimator<P, N, PC> {
    /// Creates a new instance with given parameters.
    pub fn new(
        payment_contract: PC,
        gas_buffer_percent: u16,
        gas_price_buffer_percent: u16,
        profit_percent_threshold: u8,
    ) -> Self {
        Self {
            payment_contract,
            gas_buffer_percent,
            gas_price_buffer_percent,
            profit_percent_threshold,
            _p: PhantomData,
            _n: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
struct Eip1559GasEstimates {
    pub max_priority_fee_per_gas: u128,
    pub max_fee_per_gas: u128,
}

impl<P, N, PC> RequestFulfillmentEstimator<P, N, PC>
where
    P: Provider<N>,
    N: Network,
    PC: PaymentContract<P, N>,
{
    /// Attempt to calculate request fulfillment parameters
    #[tracing::instrument(skip_all,
        fields(
            payment_contract_addr = %self.payment_contract.address(),
            request_id = %request_id
        ))
    ]
    pub async fn get_fulfillment_params<'a, C>(
        &self,
        request_id: U256,
        fulfil_call: &C,
        fulfil_call_address: &Address,
    ) -> Result<RequestFulfillmentParams, PaymentEstimatorError>
    where
        C: SolCall,
    {
        let call_builder = SolCallBuilder::new_sol(
            self.payment_contract.provider(),
            fulfil_call_address,
            fulfil_call,
        );

        let request = self
            .payment_contract
            .get_request_details(request_id)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    "Failed to to obtain request details"
                );
                OtherPaymentEstimatorError::Contract(e, "failed to obtain request details")
            })?;

        let config = self.payment_contract.get_config().await.map_err(|e| {
            tracing::error!(
                error = ?e,
                "Failed to to obtain payment config"
            );
            OtherPaymentEstimatorError::Contract(e, "failed to obtain payment config")
        })?;

        // Calculate flat fee from config
        let flat_fee_wei =
            1_000_000_000_000u128 * u128::from(config.fulfillment_flat_fee_native_ppm()); // cannot overflow, 2**40 * 2**32

        // Get gas estimates
        let tx_gas_estimates = self.estimate_eip_1559().await?;
        let estimated_gas = self
            .estimate_fulfillment_gas(call_builder.clone(), &request, &tx_gas_estimates)
            .await?;

        // Do not process the request if we don't make enough profits
        let max_gas = self
            .assert_profits(&request, &tx_gas_estimates, estimated_gas, flat_fee_wei)
            .await?;

        tracing::info!(
            %request_id,
            max_gas,
            max_fee_per_gas = tx_gas_estimates.max_fee_per_gas,
            max_priority_fee_per_gas = tx_gas_estimates.max_priority_fee_per_gas,
            "Calculated estimated gas cost for request"
        );

        Ok(RequestFulfillmentParams {
            gas_limit: max_gas,
            max_fee_per_gas: tx_gas_estimates.max_fee_per_gas,
            max_priority_fee_per_gas: tx_gas_estimates.max_priority_fee_per_gas,
        })
    }

    /// Estimate amount of gas required to fulfil a decryption request.
    async fn estimate_fulfillment_gas<MaybeRefP, C>(
        &self,
        call_builder: SolCallBuilder<MaybeRefP, C, N>,
        request: &impl RequestDetails,
        tx_gas_estimates: &Eip1559GasEstimates,
    ) -> Result<u64, PaymentEstimatorError>
    where
        MaybeRefP: Provider<N>,
        C: SolCall,
    {
        // Estimate gas limit for fulfillDecryptionRequest call
        let gas_estimation_call = call_builder
            .max_priority_fee_per_gas(tx_gas_estimates.max_priority_fee_per_gas)
            .max_fee_per_gas(tx_gas_estimates.max_fee_per_gas);

        let estimated_gas = gas_estimation_call.estimate_gas().await.map_err(|e| {
            let calldata = gas_estimation_call.calldata();
            let input = gas_estimation_call.input();
            let callback_addr = request.callback();
            tracing::error!(
                error = ?e,
                max_fee_per_gas = tx_gas_estimates.max_fee_per_gas,
                max_priority_fee_per_gas = tx_gas_estimates.max_priority_fee_per_gas,
                calldata = %calldata,
                input = %input,
                callback_addr = %callback_addr,
                "Failed to simulate call to estimate gas costs"
            );

            OtherPaymentEstimatorError::Contract(e, "failed to simulate call to estimate gas costs")
        })?;

        let estimated_gas = estimated_gas
            .checked_add(request.callback_gas_limit().into())
            .ok_or(OtherPaymentEstimatorError::IntegerOverflow(
                "failed to add callback to estimated gas",
            ))?; // add the callback to the estimate

        // Add buffer to the estimate
        self.gas_with_buffer(estimated_gas)
    }

    /// Estimate the eip 1559 fees
    async fn estimate_eip_1559(&self) -> Result<Eip1559GasEstimates, PaymentEstimatorError> {
        let fee_data = self
            .payment_contract
            .provider()
            .estimate_eip1559_fees()
            .await
            .map_err(|e| {
                OtherPaymentEstimatorError::RpcWithTransportErrorKind(
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
        request: &impl RequestDetails,
        tx_gas_params: &Eip1559GasEstimates,
        estimated_gas: u64,
        flat_fee_wei: u128,
    ) -> Result<u64, PaymentEstimatorError> {
        // Calculate an upper bound on the request cost
        let gas_price = tx_gas_params
            .max_priority_fee_per_gas
            .checked_add(tx_gas_params.max_fee_per_gas)
            .ok_or(OtherPaymentEstimatorError::IntegerOverflow(
                "failed to calculate total gas price",
            ))?;
        let request_cost_upper_bound = gas_price.checked_mul(estimated_gas.into()).ok_or(
            OtherPaymentEstimatorError::IntegerOverflow("failed to calculate total request cost"),
        )?;

        // Ensure that the user can cover the upper bound of the cost
        let available_funds = self.get_user_available_funds(request).await?;
        let profit = available_funds
            .checked_sub(request_cost_upper_bound)
            .ok_or_else(|| {
                PaymentEstimatorError::InsufficientFunds(InsufficientFundsError {
                    available_funds,
                    request_cost: request_cost_upper_bound,
                })
            })?;
        let profit = profit.min(flat_fee_wei); // bound profit by at most flat_fee_wei
        let profit_percent =
            profit
                .checked_mul(100)
                .ok_or(OtherPaymentEstimatorError::IntegerOverflow(
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
            Err(PaymentEstimatorError::InsufficientFunds(
                InsufficientFundsError {
                    available_funds,
                    request_cost: request_cost_upper_bound,
                },
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
    fn gas_price_with_buffer(&self, gas_price: u128) -> Result<u128, PaymentEstimatorError> {
        let gas_price_mul_100 = gas_price
            .checked_mul(100u128 + u128::from(self.gas_price_buffer_percent))
            .ok_or(OtherPaymentEstimatorError::IntegerOverflow(
                "overflow calculating gas_price_with_buffer",
            ))?;
        Ok(gas_price_mul_100 / 100)
    }

    /// Compute the gas with a buffer as (gas * (100 + gas_buffer_percent)) / 100
    fn gas_with_buffer(&self, gas: u64) -> Result<u64, PaymentEstimatorError> {
        let gas_mul_100 = gas
            .checked_mul(100u64 + u64::from(self.gas_buffer_percent))
            .ok_or(OtherPaymentEstimatorError::IntegerOverflow(
                "overflow calculating gas_with_buffer",
            ))?;
        Ok(gas_mul_100 / 100)
    }

    /// Return the current user's balance for subscription. For direct funding, return the amount
    /// paid by the user for the request.
    async fn get_user_available_funds(
        &self,
        request: &impl RequestDetails,
    ) -> Result<u128, PaymentEstimatorError> {
        if let Some(direct_fee_paid) = request.direct_fee_paid() {
            // User has already paid for the request
            direct_fee_paid.try_into().map_err(|e| {
                OtherPaymentEstimatorError::SolFromUint(e, "failed to cast funding fee to u128")
                    .into()
            })
        } else {
            let native_balance = request
                .subscription_balance()
                .expect("native balance cannot be none if direct_fee_paid is none");
            Ok(native_balance)
        }
    }
}
