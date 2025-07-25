use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;

/// Details of a request with various payment information.
#[allow(unused)]
pub trait RequestDetails {
    fn id(&self) -> &U256;

    fn callback(&self) -> &Address;

    fn callback_gas_limit(&self) -> u32;

    fn is_subscription(&self) -> bool;

    fn direct_fee_paid(&self) -> Option<U256>;

    fn subscription_balance(&self) -> Option<u128>;
}

/// Configuration of a [`PaymentContract`].
#[allow(unused)]
pub trait PaymentConfig {
    fn max_gas_limit(&self) -> u32;
    fn gas_after_payment_calculation(&self) -> u32;
    fn fulfillment_flat_fee_native_ppm(&self) -> u32;
    fn wei_per_unit_gas(&self) -> u32;
    fn bls_pairing_check_overhead(&self) -> u32;
    fn native_premium_percentage(&self) -> u8;
    fn gas_for_call_exact_check(&self) -> u32;
}

/// A contract containing a payment configuration and requests.
pub trait PaymentContract<P, N>
where
    P: Provider<N>,
    N: Network,
{
    type PaymentConfig: PaymentConfig;
    type RequestDetails: RequestDetails;

    async fn get_config(&self) -> Result<Self::PaymentConfig, alloy::contract::Error>;

    async fn get_request_details(
        &self,
        id: U256,
    ) -> Result<Self::RequestDetails, alloy::contract::Error>;

    fn provider(&self) -> &P;

    fn address(&self) -> &Address;
}

/// Helper struct implementing [`RequestDetails`]
#[derive(Debug, Clone)]
pub struct DefaultRequestDetails {
    pub id: U256,
    pub callback: Address,
    pub callback_gas_limit: u32,
    pub is_subscription: bool,
    pub direct_fee_paid: Option<U256>,
    pub subscription_balance: Option<u128>,
}

impl RequestDetails for DefaultRequestDetails {
    fn id(&self) -> &U256 {
        &self.id
    }

    fn callback(&self) -> &Address {
        &self.callback
    }

    fn callback_gas_limit(&self) -> u32 {
        self.callback_gas_limit
    }

    fn is_subscription(&self) -> bool {
        self.is_subscription
    }

    fn direct_fee_paid(&self) -> Option<U256> {
        self.direct_fee_paid
    }

    fn subscription_balance(&self) -> Option<u128> {
        self.subscription_balance
    }
}

/// A default implementation of [`PaymentConfig`] for a solidity struct.
#[macro_export]
macro_rules! impl_payment_config {
    ($ty:ty) => {
        impl PaymentConfig for $ty {
            fn max_gas_limit(&self) -> u32 {
                self.maxGasLimit
            }

            fn gas_after_payment_calculation(&self) -> u32 {
                self.gasAfterPaymentCalculation
            }

            fn fulfillment_flat_fee_native_ppm(&self) -> u32 {
                self.fulfillmentFlatFeeNativePPM
            }

            fn wei_per_unit_gas(&self) -> u32 {
                self.weiPerUnitGas
            }

            fn bls_pairing_check_overhead(&self) -> u32 {
                self.blsPairingCheckOverhead
            }

            fn native_premium_percentage(&self) -> u8 {
                self.nativePremiumPercentage
            }

            fn gas_for_call_exact_check(&self) -> u32 {
                self.gasForCallExactCheck
            }
        }
    };
}

/// A default implementation of [`PaymentContract`] for a solidity contract.
#[macro_export]
macro_rules! impl_payment_contract {
    ($module:ident,$instance:ident) => {
        impl<P, N> PaymentContract<P, N> for $module::$instance<P, N>
        where
            P: alloy::providers::Provider<N>,
            N: alloy::network::Network,
        {
            type PaymentConfig = $module::getConfigReturn;
            type RequestDetails = DefaultRequestDetails;

            async fn get_config(&self) -> Result<Self::PaymentConfig, alloy::contract::Error> {
                self.getConfig().call().await
            }

            async fn get_request_details(
                &self,
                id: alloy::primitives::U256,
            ) -> Result<Self::RequestDetails, alloy::contract::Error> {
                let details = self.getRequest(id).call().await?;
                let mut request_details = DefaultRequestDetails {
                    id,
                    callback: details.callback,
                    callback_gas_limit: details.callbackGasLimit,
                    is_subscription: !details.subId.is_zero(),
                    direct_fee_paid: None,
                    subscription_balance: None,
                };

                if details.subId.is_zero() {
                    request_details.direct_fee_paid = Some(details.directFundingFeePaid);
                } else {
                    let sub = self
                        .getSubscription(details.subId)
                        .call()
                        .await
                        .map_err(|e| {
                            tracing::error!(
                                error = ?e,
                                sub_id = %details.subId,
                                "Failed to call {}::getSubscription",
                                stringify!($module)
                            );
                            e
                        })?;

                    request_details.subscription_balance =
                        Some(sub.nativeBalance.try_into().expect("u96 fits in u128"));
                }

                Ok(request_details)
            }

            fn provider(&self) -> &P {
                self.provider()
            }

            fn address(&self) -> &alloy::primitives::Address {
                self.address()
            }
        }
    };
}

// Submodule declarations
pub mod estimator;
pub mod fulfiller;

// Re-export key items from submodules
pub use estimator::*;
pub use fulfiller::*;
