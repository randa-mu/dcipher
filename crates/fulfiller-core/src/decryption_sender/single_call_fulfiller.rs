//! Concrete implementation of a [`TransactionFulfiller`] for the decryption sender contract.
//! [`SingleCallTxFulfiller`] attempts to fulfil decryption requests sequentially with a transaction
//! per fulfillment.

use alloy::primitives::TxHash;
use alloy::providers::Provider;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use std::time::Duration;

use crate::fulfiller::TransactionFulfiller;
use crate::decryption_sender::SignedDecryptionRequest;

#[derive(thiserror::Error, Debug)]
pub enum SingleCallTxFullfillerError {
    #[error(transparent)]
    PendingTransaction(#[from] alloy::providers::PendingTransactionError),

    #[error(transparent)]
    Contract(#[from] alloy::contract::Error),
}

/// Implementation of [`TransactionFulfiller`] where each call is done in a separate transaction.
pub struct SingleCallTxFulfiller<P> {
    decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<P>,
    required_confirmations: u64,
    timeout: Duration,
}

impl<P> SingleCallTxFulfiller<P> {
    /// Creates a new instance with given parameters.
    pub fn new(
        decryption_sender_instance: DecryptionSender::DecryptionSenderInstance<P>,
        required_confirmations: u64,
        timeout: Duration,
    ) -> Self {
        Self {
            decryption_sender_instance,
            required_confirmations,
            timeout,
        }
    }
}

impl<P> TransactionFulfiller for SingleCallTxFulfiller<P>
where
    P: Provider + 'static,
{
    type SignedRequest = SignedDecryptionRequest<'static>;
    type Error = SingleCallTxFullfillerError;

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
                    let pending_tx_res = self.fulfil_decryption_request(req.to_owned()).await;

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
                        transaction_results.push(Err(e.into()));
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

impl<P> SingleCallTxFulfiller<P>
where
    P: Provider,
{
    async fn fulfil_decryption_request<'a>(
        &self,
        ready_request: SignedDecryptionRequest<'a>,
    ) -> Result<
        impl Future<Output = Result<TxHash, alloy::providers::PendingTransactionError>> + 'a,
        alloy::contract::Error,
    > {
        let pending_tx = self
            .decryption_sender_instance
            .fulfillDecryptionRequest(
                ready_request.id,
                ready_request.decryption_key,
                ready_request.signature.into_owned(),
            )
            .send()
            .await?;

        Ok(pending_tx
            .with_required_confirmations(self.required_confirmations)
            .with_timeout(Some(self.timeout))
            .watch())
    }
}
