//! Concrete implementation of a [`TransactionFulfiller`] for the blocklock contract.
//! [`BlocklockFulfiller`] attempts to fulfil decryption requests sequentially with a transaction
//! per fulfillment.

use crate::agents::blocklock::metrics::Metrics;
use crate::agents::payment::estimator::{PaymentEstimatorCostError, RequestFulfillmentEstimator};
use crate::agents::payment::fulfiller::{GenericFulfiller, GenericFulfillerError};
use crate::decryption_sender::SignedDecryptionRequest;
use crate::fulfiller::TransactionFulfiller;
use alloy::network::{Ethereum, Network};
use alloy::providers::{Provider, WalletProvider};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use generated::blocklock::blocklock_sender::BlocklockSender;
use generated::blocklock::decryption_sender::DecryptionSender;
use std::time::Duration;

pub type BlocklockFulfillerError = GenericFulfillerError;

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
#[derive(Clone)]
pub struct BlocklockFulfiller<P, N = Ethereum> {
    fulfiller: GenericFulfiller<P, N, BlocklockSender::BlocklockSenderInstance<P, N>>,
}

impl<P, N> BlocklockFulfiller<P, N>
where
    P: Provider<N> + Clone,
    N: Network,
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
        let payment_estimator = RequestFulfillmentEstimator::<P, N, _>::new(
            blocklock_sender_instance.clone(),
            gas_buffer_percent,
            gas_price_buffer_percent,
            profit_percent_threshold,
        );

        let fulfiller = GenericFulfiller::new(
            decryption_sender_instance.provider().to_owned(),
            decryption_sender_instance.address().to_owned(),
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

impl<P, N> TransactionFulfiller for BlocklockFulfiller<P, N>
where
    P: Provider<N> + WalletProvider<N> + 'static,
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
            let calls: Vec<_> = requests
                .into_iter()
                .cloned()
                .map(|req| {
                    let call = DecryptionSender::fulfillDecryptionRequestCall {
                        requestId: req.id,
                        signature: req.signature.into_owned(),
                        decryptionKey: req.decryption_key,
                    };

                    (req.id, call)
                })
                .collect();

            let results = self.fulfiller.fulfil_calls(calls).await;
            results.iter().for_each(|res| match &res {
                Ok(_) => {
                    Metrics::report_decryption_success();
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
                    Metrics::report_decryption_error();
                }
            });

            results
        }
        .boxed()
    }
}
