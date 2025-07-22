//! Concrete implementation of a [`TransactionFulfiller`] for the randomness sender contract.
//! [`RandomnessFulfiller`] attempts to fulfil randomness requests sequentially with a transaction
//! per fulfillment.

use payment_warp::estimator::{PaymentEstimatorCostError, RequestFulfillmentEstimator};
use payment_warp::fulfiller::{GenericFulfiller, GenericFulfillerError};
use crate::contracts::RandomnessSender;
use crate::metrics::Metrics;
use enc_core::fulfiller::TransactionFulfiller;
use enc_core::signature_sender::{SignedSignatureRequest, contracts::SignatureSender};
use alloy::network::{Ethereum, Network};
use alloy::providers::{Provider, WalletProvider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use std::time::Duration;

pub type RandomnessFulfillerError = GenericFulfillerError;

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct RandomnessFulfiller<P, N = Ethereum> {
    fulfiller: GenericFulfiller<P, N, RandomnessSender::RandomnessSenderInstance<P, N>>,
}

impl<P, N> RandomnessFulfiller<P, N>
where
    P: Provider<N> + Clone,
    N: Network,
{
    /// Creates a new instance with given parameters.
    pub fn new(
        signature_sender_instance: SignatureSender::SignatureSenderInstance<P, N>,
        randomness_sender_instance: RandomnessSender::RandomnessSenderInstance<P, N>,
        required_confirmations: u64,
        timeout: Duration,
        gas_buffer_percent: u16,
        gas_price_buffer_percent: u16,
        profit_percent_threshold: u8,
    ) -> Self {
        let payment_estimator = RequestFulfillmentEstimator::<P, N, _>::new(
            randomness_sender_instance.clone(),
            gas_buffer_percent,
            gas_price_buffer_percent,
            profit_percent_threshold,
        );

        let fulfiller = GenericFulfiller::new(
            signature_sender_instance.provider().to_owned(),
            signature_sender_instance.address().to_owned(),
            payment_estimator,
            required_confirmations,
            timeout,
        );

        Self { fulfiller }
    }

    /// Allows to simulate call while never submitting transactions.
    pub fn set_simulate_tx(&mut self) {
        self.fulfiller.set_simulate_tx();
    }
}

impl<P, N> TransactionFulfiller for RandomnessFulfiller<P, N>
where
    P: Provider<N> + WalletProvider<N> + 'static,
    N: Network,
{
    type SignedRequest = SignedSignatureRequest;
    type Error = RandomnessFulfillerError;

    fn fulfil_requests<'lt_self, 'lt_sr, I>(
        &'lt_self self,
        requests: I,
    ) -> BoxFuture<'lt_self, Vec<Result<(), Self::Error>>>
    where
        I: IntoIterator<Item = &'lt_sr Self::SignedRequest> + Send + 'lt_self,
        I::IntoIter: Send,
    {
        async move {
            let calls: Vec<_> = requests
                .into_iter()
                .cloned()
                .map(|req| {
                    let call = SignatureSender::fulfillSignatureRequestCall {
                        requestID: req.id,
                        signature: req.signature,
                    };

                    (req.id, call)
                })
                .collect();

            let results = self.fulfiller.fulfil_calls(calls).await;
            results.iter().for_each(|res| match &res {
                Ok(_) => {
                    Metrics::report_randomness_fulfilled();
                }
                Err(GenericFulfillerError::CostError(
                    PaymentEstimatorCostError::SubscriptionInsufficientFunds(_),
                )) => {
                    Metrics::report_subscription_insufficient_funds();
                }
                Err(GenericFulfillerError::CostError(
                    PaymentEstimatorCostError::FulfillmentCostTooHigh(_),
                )) => {
                    Metrics::report_fulfillment_cost_too_high();
                }
                Err(GenericFulfillerError::CostError(PaymentEstimatorCostError::ProfitTooLow(
                    _,
                ))) => {
                    Metrics::report_fulfillment_profit_too_low();
                }
                Err(_) => {
                    Metrics::report_fulfillment_error();
                }
            });

            results
        }
        .boxed()
    }
}
