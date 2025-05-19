//! Solidity imports for the blocklock contracts.

pub use blocklock_sender::*;

mod blocklock_sender {
    use crate::agents::payment::{
        DefaultRequestDetails, PaymentConfig, PaymentContract, impl_payment_config,
    };
    use alloy::contract::Error;
    use alloy::network::Network;
    use alloy::primitives::{Address, U256};
    use alloy::providers::Provider;

    alloy::sol!(
        #[allow(clippy::too_many_arguments)]
        #[derive(Debug)]
        #[sol(rpc)]
        BlocklockSender,
        "../blocklock-solidity/out/BlocklockSender.sol/BlocklockSender.json"
    );

    impl_payment_config!(BlocklockSender::getConfigReturn);

    impl<P, N> PaymentContract<P, N> for BlocklockSender::BlocklockSenderInstance<P, N>
    where
        P: Provider<N>,
        N: Network,
    {
        type PaymentConfig = BlocklockSender::getConfigReturn;
        type RequestDetails = DefaultRequestDetails;

        async fn get_config(&self) -> Result<Self::PaymentConfig, Error> {
            self.getConfig().call().await
        }

        async fn get_request_details(&self, id: U256) -> Result<Self::RequestDetails, Error> {
            let details = self.getRequest(id).call().await?;
            let mut request_details = DefaultRequestDetails {
                id,
                callback: details.callback,
                callback_gas_limit: details.callbackGasLimit,
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
                            "Failed to call BlocklockSender::getSubscription"
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

        fn address(&self) -> &Address {
            self.address()
        }
    }
}
