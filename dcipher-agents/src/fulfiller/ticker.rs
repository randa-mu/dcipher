//! Implementation of a [`TickerBasedFulfiller`] for any request, signed requests, request signing
//! registry and transaction fulfillers.

use crate::fulfiller::failure::{RequestRetryStrategy, RetryStrategy, RetryableRequest};
use crate::fulfiller::{
    Fulfiller, Identifier, RequestChannel, RequestSigningRegistry, Stopper, Ticker,
    TickerBasedFulfiller, TransactionFulfiller,
};
use itertools::{Either, Itertools};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

/// Alias of the type used to store requests
type PendingRequestStorage<R> = Arc<tokio::sync::Mutex<Vec<R>>>;
type ReadyRequestStorage<SR> = Arc<tokio::sync::Mutex<VecDeque<SR>>>;
type RetryRequestStorage<SR> = Arc<tokio::sync::Mutex<VecDeque<RetryableRequest<SR>>>>;

/// Structure used to fulfill requests by requesting a signed request from a [`RequestSigningRegistry`],
/// before submitting it to a [`TransactionFulfiller`] with multiple attempts.
pub struct TickerFulfiller<R, SR, RS, TF> {
    // Storage for pending, ready and requests to retry
    pending_requests: PendingRequestStorage<R>,
    ready_requests: ReadyRequestStorage<SR>,
    retry_requests: RetryRequestStorage<SR>,

    // Number of requests that were processed this tick, use a mutex for longer locks
    num_fulfilment_curr_tick: tokio::sync::Mutex<usize>,

    // Real transaction fulfiller
    fulfiller: TF,

    // Registry used to request and fetch ReadyRequests
    signing_registry: Arc<RS>,

    // Various configuration parameters
    max_fulfilment_per_tick: usize,
    retry_strategy: RetryStrategy,
}

/// Implementation of a [`RequestChannel`] using tokio's unbounded channel.
pub struct UnboundedRequestChannel<R> {
    tx: tokio::sync::mpsc::UnboundedSender<Vec<R>>,
}

/// Implementation of a [`Stopper`] using tokio's oneshot channel.
pub struct OneshotStopper {
    tx: tokio::sync::oneshot::Sender<()>,
}

impl<R, SR, RS, TF> TickerFulfiller<R, SR, RS, TF> {
    pub(crate) fn new(
        signing_registry: RS,
        transaction_fulfiller: TF,
        max_fulfilment_per_tick: usize,
        retry_strategy: RetryStrategy,
    ) -> Self {
        Self {
            pending_requests: Arc::new(tokio::sync::Mutex::new(Vec::new())),
            ready_requests: Arc::new(tokio::sync::Mutex::new(VecDeque::new())),
            retry_requests: Arc::new(tokio::sync::Mutex::new(VecDeque::new())),
            num_fulfilment_curr_tick: max_fulfilment_per_tick.into(),
            fulfiller: transaction_fulfiller,
            signing_registry: Arc::new(signing_registry),
            max_fulfilment_per_tick,
            retry_strategy,
        }
    }
}

impl<R, SR, RS, TF> Fulfiller for TickerFulfiller<R, SR, RS, TF>
where
    R: Identifier + Send + Sync + 'static,
    SR: Identifier + Send + Sync + 'static,
    RS: RequestSigningRegistry<Request = R, SignedRequest = SR>,
    TF: TransactionFulfiller<SignedRequest = SR>,
{
    type Request = R;
    type SignedRequest = SR;
    type RequestSigningRegistry = RS;
    type TransactionFulfiller = TF;

    type RequestChannel = UnboundedRequestChannel<R>;
    type Stop = OneshotStopper;
}

impl<R, SR, RS, TF> TickerBasedFulfiller for TickerFulfiller<R, SR, RS, TF>
where
    R: Identifier + Send + Sync + 'static,
    SR: Identifier + Send + Sync + 'static,
    RS: RequestSigningRegistry<Request = R, SignedRequest = SR>,
    TF: TransactionFulfiller<SignedRequest = SR>,
{
    fn run(self, ticker: impl Ticker) -> (Self::Stop, Self::RequestChannel) {
        let arc_self = Arc::new(self);

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();
        let cancellation_token = CancellationToken::new();

        tokio::task::spawn(async move {
            tokio::select! {
                _ = stop_rx => {
                    tracing::info!("Ticker-based fulfiller received stop signal, exiting...");
                    cancellation_token.cancel();
                },

                _ = arc_self.main(ticker, rx, cancellation_token.child_token()) => {
                    unreachable!("main should not terminate");
                }
            }
        });

        (
            OneshotStopper { tx: stop_tx },
            UnboundedRequestChannel { tx },
        )
    }
}

impl<R, SR, RS, TF> TickerFulfiller<R, SR, RS, TF>
where
    R: Identifier + Send + Sync + 'static,
    SR: Identifier + Send + Sync + 'static,
    RS: RequestSigningRegistry<Request = R, SignedRequest = SR>,
    TF: TransactionFulfiller<SignedRequest = SR>,
{
    #[tracing::instrument(skip_all)]
    async fn main(
        self: Arc<Self>,
        mut ticker: impl Ticker,
        rx: tokio::sync::mpsc::UnboundedReceiver<Vec<R>>,
        cancellation_token: CancellationToken,
    ) {
        // Create a thread to asynchronously process incoming requests
        tokio::task::spawn(
            self.clone()
                .process_new_requests_task(rx, cancellation_token.clone()),
        );

        tracing::info!(
            retry_strategy = ?self.retry_strategy,
            max_fulfillment_per_tick = self.max_fulfilment_per_tick,
            "Ticker-based fulfiller ready to serve requests"
        );

        // Main thread
        loop {
            // Wait for the next tick
            ticker.tick().await;

            // Reset the number of fulfilled requests this tick
            *self.num_fulfilment_curr_tick.lock().await = self.max_fulfilment_per_tick;

            // Process pending requests
            self.process_pending_requests().await;

            // Get a vec of requests to fulfil
            let requests_to_fulfil = self.requests_to_fulfil().await;

            // Try to fulfil the requests
            if !requests_to_fulfil.is_empty() {
                tokio::task::spawn(
                    self.clone()
                        .fulfil_requests_task(requests_to_fulfil, cancellation_token.child_token()),
                );
            }
        }
    }

    /// Task processing new requests
    #[instrument(skip_all)]
    async fn process_new_requests_task(
        self: Arc<Self>,
        mut rx: tokio::sync::mpsc::UnboundedReceiver<Vec<R>>,
        cancellation_token: CancellationToken,
    ) {
        let inner_fn = async move {
            tracing::debug!("Recv task processing new requests started");

            loop {
                tracing::trace!("Waiting for new requests");
                let Some(requests) = rx.recv().await else {
                    tracing::error!("Requests receiving channel closed... Exiting recv task.");
                    return;
                };
                tracing::debug!(
                    n_requests = requests.len(),
                    "Received new requests through channel"
                );

                // Process only the newly arrived requests, pending requests will be processed on the next tick.
                self.process_requests(requests).await;
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping recv task, reason: cancellation token");
            },

            _ = inner_fn => (),
        }
    }

    /// Helper function that attempts to obtain signed requests from the pending requests.
    async fn process_pending_requests(&self) {
        let requests: Vec<_> = self.pending_requests.lock().await.drain(..).collect();
        if requests.is_empty() {
            tracing::debug!("No pending requests to process");
        } else {
            self.process_requests(requests).await;
        }
    }

    /// Process requests by attempting to transform them into signed requests through the
    /// signing registry.
    async fn process_requests(&self, requests: Vec<R>) {
        let n_pending_requests = requests.len();
        tracing::debug!(
            "Trying to fetch `{n_pending_requests}` requests from the signing registry"
        );
        // Forward the requests to the signer registry and collect
        let signed_requests: Vec<_> = self
            .signing_registry
            .try_fetch_signed_requests(requests.iter())
            .collect();

        // Partition requests based on whether they were signed or not
        let (signed_requests, pending_decryption_requests): (Vec<_>, Vec<_>) = requests
            .into_iter()
            .zip(signed_requests)
            .partition_map(|(req, signed_request)| {
                if let Some(signed_request) = signed_request {
                    Either::Left(signed_request)
                } else {
                    Either::Right(req)
                }
            });

        tracing::info!(
            "Obtained `{}` signed requests out of `{n_pending_requests}` from the signing registry",
            signed_requests.len()
        );

        // Store the pending requests
        {
            let mut pending_requests = self.pending_requests.lock().await;
            pending_decryption_requests.into_iter().for_each(|req| {
                pending_requests.push(req);
            });
        } // drop mutex guard

        // Store the signed requests
        {
            let mut ready_requests = self.ready_requests.lock().await;
            signed_requests.into_iter().for_each(|req| {
                ready_requests.push_back(req);
            });
        } // drop mutex guard
    }

    /// Generates a vector of requests to fulfil for this tck.
    async fn requests_to_fulfil(&self) -> Vec<RetryableRequest<SR>> {
        // Lock mutex until the end of the function
        let mut num_fulfilment_curr_tick = self.num_fulfilment_curr_tick.lock().await;

        // First, try to use fresh ready requests
        let mut ready_requests = self.ready_requests.lock().await;
        let n_req_readys = num_fulfilment_curr_tick.min(ready_requests.len());
        let ready_requests = ready_requests
            .drain(0..n_req_readys)
            .map(|req| RetryableRequest::new(req, self.retry_strategy));

        // Reduce the number of requests that can be fulfilled
        *num_fulfilment_curr_tick -= n_req_readys;

        // Then, use requests from the retry list
        let mut retry_requests = self.retry_requests.lock().await;
        let n_req_retries = num_fulfilment_curr_tick.min(retry_requests.len());
        let requests_to_fulfil = retry_requests.drain(0..n_req_retries);

        // Reduce the number of requests that can be fulfilled
        *num_fulfilment_curr_tick -= n_req_retries;

        tracing::debug!(
            "Found `{n_req_retries}` requests to fulfil from the retry list and `{n_req_readys}` from the ready list"
        );

        // Drain holds a mutable reference to self, easier to simply collect
        requests_to_fulfil.chain(ready_requests).collect()
    }

    /// Task responsible to fulfil requests.
    #[tracing::instrument(skip_all, fields(number_of_requests = requests_to_fulfil.len()))]
    async fn fulfil_requests_task(
        self: Arc<Self>,
        requests_to_fulfil: Vec<RetryableRequest<SR>>,
        cancellation_token: CancellationToken,
    ) {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping fulfil requests task, reason: cancellation token");
            },

            _ = self.fulfil_requests(requests_to_fulfil) => (),
        }
    }

    /// Fulfil
    #[tracing::instrument(skip_all, fields(number_of_requests = requests_to_fulfil.len()))]
    async fn fulfil_requests(&self, requests_to_fulfil: Vec<RetryableRequest<SR>>) {
        tracing::info!("Attempting to fulfil requests");
        let results = self
            .fulfiller
            .fulfil_decryption_requests(
                requests_to_fulfil
                    .iter()
                    .map(|retryable_request| &retryable_request.req),
            )
            .await;

        let (mut success, mut failure, mut dropped) = (0usize, 0usize, 0usize);
        // Log and decide whether to retry next tick or not
        for (mut request, result) in requests_to_fulfil.into_iter().zip(results) {
            let error = match result {
                Ok(_) => {
                    tracing::info!(request_id = %request.req.id(), "Decryption request successfully fulfilled");
                    success += 1;
                    continue;
                }
                Err(e) => e,
            };

            failure += 1;
            tracing::error!(%error, request_id = %request.req.id(), "Failed to fulfil transaction");
            if request.retry_strategy.should_retry_and_update() {
                tracing::info!(request_id = %request.req.id(), "Adding request to retry list");
                self.retry_requests.lock().await.push_back(request);
            } else {
                tracing::warn!(request_id = %request.req.id(), "Retry strategy not allowing retry, dropping request");
                dropped += 1;
            }
        }

        tracing::info!(success, failure, dropped, "Fulfil requests completed");
    }
}

impl<R> RequestChannel for UnboundedRequestChannel<R> {
    type Request = R;

    fn register_requests<I>(&mut self, requests: I)
    where
        I: IntoIterator,
        I::Item: Into<R>,
    {
        let requests = requests.into_iter().map(Into::into).collect();
        if let Err(e) = self.tx.send(requests) {
            tracing::error!(error = %e, "Failed to send request to unbounded channel: channel closed / receiver dropped");
        }
    }
}

impl Stopper for OneshotStopper {
    async fn stop(self) {
        if self.tx.send(()).is_err() {
            tracing::error!("Failed to send stop signal to task: receiver dropped");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decryption_sender::single_party_signer::StandaloneSigner;
    use crate::decryption_sender::{DecryptionRequest, SignedDecryptionRequest};
    use crate::ibe_helper::IbeIdentityOnBn254G1Suite;
    use crate::ser::tests::bn254::encode_ciphertext;
    use alloy::primitives::{Bytes, U256};
    use ark_ec::AffineRepr;
    use ark_ff::{BigInteger, MontFp, PrimeField};
    use futures_util::FutureExt;
    use futures_util::future::BoxFuture;

    fn create_ciphertext(id: ark_bn254::G2Affine) -> Bytes {
        let (x, y) = id.xy().unwrap();
        let (x, y) = (*x, *y);
        let x0 = x.c0.into_bigint().to_bytes_be();
        let x1 = x.c1.into_bigint().to_bytes_be();
        let y0 = y.c0.into_bigint().to_bytes_be();
        let y1 = y.c1.into_bigint().to_bytes_be();

        encode_ciphertext(&x0, &x1, &y0, &y1)
    }

    #[derive(thiserror::Error, Debug)]
    #[error("fake error")]
    struct FakeError;
    struct FakeFulfiller;

    impl TransactionFulfiller for FakeFulfiller {
        type SignedRequest = SignedDecryptionRequest<'static>;
        type Error = FakeError;

        fn fulfil_decryption_requests<'lt_self, 'lt_sr, I>(
            &'lt_self self,
            requests: I,
        ) -> BoxFuture<'lt_self, Vec<Result<(), Self::Error>>>
        where
            I: IntoIterator<Item = &'lt_sr Self::SignedRequest> + Send + 'lt_self,
            I::IntoIter: Send,
        {
            async move { requests.into_iter().map(|_| Err(FakeError)).collect() }.boxed()
        }
    }

    #[tokio::test]
    async fn test_requests_to_fulfil_retry_never() {
        use ark_bn254::Fr;

        // bn254 ciphersuite
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST_IBE", 31337);
        let sk: Fr = MontFp!("0102030405060708091011121314151617181920");
        let signer = StandaloneSigner::new(cs.clone(), sk);
        let signer_registry = signer.registry();

        // Static ephemeral pk / condition
        let eph_pk = ark_bn254::g2::G2Affine::generator();
        let condition = Bytes::from(b"MyCustomCondition");

        let retry_strategy = RetryStrategy::Never;
        let retry_never_fulfiller =
            TickerFulfiller::new(signer_registry, FakeFulfiller, 100, retry_strategy);

        // Add a pending request to the fulfiller
        let req = DecryptionRequest {
            id: U256::from(1u64),
            condition,
            ciphertext: create_ciphertext(eph_pk),
        };
        retry_never_fulfiller
            .pending_requests
            .lock()
            .await
            .push(req.clone());

        // Process pending requests, this should transform the request into a ReadyRequest
        retry_never_fulfiller.process_pending_requests().await;
        assert_eq!(retry_never_fulfiller.ready_requests.lock().await.len(), 1);
        assert_eq!(
            retry_never_fulfiller.ready_requests.lock().await[0].id(),
            &req.id
        );

        // Get a vec of requests to fulfil
        let requests_to_fulfil = retry_never_fulfiller.requests_to_fulfil().await;
        assert_eq!(retry_never_fulfiller.ready_requests.lock().await.len(), 0); // should have been moved from ready_requests
        assert_eq!(requests_to_fulfil.len(), 1);
        assert_eq!(requests_to_fulfil[0].req.id(), &req.id);
        assert_eq!(requests_to_fulfil[0].retry_strategy, retry_strategy.into());

        // Try to fulfil request, which should fail due to the FakeFulfiller
        retry_never_fulfiller
            .fulfil_requests(requests_to_fulfil)
            .await;
        // The request should be dropped
        assert_eq!(retry_never_fulfiller.retry_requests.lock().await.len(), 0);
        assert_eq!(retry_never_fulfiller.ready_requests.lock().await.len(), 0);
        assert_eq!(retry_never_fulfiller.pending_requests.lock().await.len(), 0);
    }

    #[tokio::test]
    async fn test_requests_to_fulfil_retry_times() {
        use ark_bn254::Fr;

        // bn254 ciphersuite
        let cs = IbeIdentityOnBn254G1Suite::new(b"TEST_IBE", 31337);
        let sk: Fr = MontFp!("0102030405060708091011121314151617181920");
        let signer = StandaloneSigner::new(cs.clone(), sk);
        let signer_registry = signer.registry();

        // Static ephemeral pk / condition
        let eph_pk = ark_bn254::g2::G2Affine::generator();
        let condition = Bytes::from(b"MyCustomCondition");

        let retry_strategy = RetryStrategy::Times(2);
        let retries_fulfiller =
            TickerFulfiller::new(signer_registry, FakeFulfiller, 100, retry_strategy);

        // Add a pending request to the fulfiller
        let req = DecryptionRequest {
            id: U256::from(1u64),
            condition,
            ciphertext: create_ciphertext(eph_pk),
        };
        retries_fulfiller
            .pending_requests
            .lock()
            .await
            .push(req.clone());

        // Process pending requests, this should transform the request into a ReadyRequest
        retries_fulfiller.process_pending_requests().await;
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 1);
        assert_eq!(
            retries_fulfiller.ready_requests.lock().await[0].id(),
            &req.id
        );

        // Get a vec of requests to fulfil
        let requests_to_fulfil = retries_fulfiller.requests_to_fulfil().await;
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 0); // should have been moved from ready_requests
        assert_eq!(requests_to_fulfil.len(), 1);
        assert_eq!(requests_to_fulfil[0].req.id(), &req.id);
        assert_eq!(requests_to_fulfil[0].retry_strategy, retry_strategy.into());

        // Try to fulfil request, which should fail due to the FakeFulfiller
        retries_fulfiller.fulfil_requests(requests_to_fulfil).await;
        // The request should be added to retry_requests
        assert_eq!(retries_fulfiller.retry_requests.lock().await.len(), 1);
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 0);
        assert_eq!(retries_fulfiller.pending_requests.lock().await.len(), 0);

        // Get a vec of requests to fulfil
        let requests_to_fulfil = retries_fulfiller.requests_to_fulfil().await;
        assert_eq!(requests_to_fulfil.len(), 1);
        assert_eq!(requests_to_fulfil[0].req.id(), &req.id);

        // Try to fulfil request again, which should fail due to the FakeFulfiller
        retries_fulfiller.fulfil_requests(requests_to_fulfil).await;
        // The request should be added to retry_requests
        assert_eq!(retries_fulfiller.retry_requests.lock().await.len(), 1);
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 0);
        assert_eq!(retries_fulfiller.pending_requests.lock().await.len(), 0);

        // Get a vec of requests to fulfil
        let requests_to_fulfil = retries_fulfiller.requests_to_fulfil().await;
        assert_eq!(requests_to_fulfil.len(), 1);
        assert_eq!(requests_to_fulfil[0].req.id(), &req.id);

        // Try to fulfil request, which should fail due to the FakeFulfiller
        retries_fulfiller.fulfil_requests(requests_to_fulfil).await;
        // The request should be dropped on the third try (initial attempt + two retries)
        assert_eq!(retries_fulfiller.retry_requests.lock().await.len(), 0);
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 0);
        assert_eq!(retries_fulfiller.pending_requests.lock().await.len(), 0);
    }
}
