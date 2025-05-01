//! Implementation of a [`TickerBasedFulfiller`] for any request, signed requests, request signing
//! registry and transaction fulfillers.

use crate::fulfiller::failure::{RequestRetryStrategy, RetryStrategy, RetryableRequest};
use crate::fulfiller::{
    Fulfiller, Identifier, RequestChannel, Stopper, Ticker, TickerBasedFulfiller,
    TransactionFulfiller,
};
use crate::signer::AsynchronousSigner;
use futures_util::stream::FuturesUnordered;
use futures_util::{FutureExt, StreamExt};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

/// Alias of the type used to store requests
type ReadyRequestStorage<SR> = Arc<tokio::sync::Mutex<VecDeque<SR>>>;
type RetryRequestStorage<SR> = Arc<tokio::sync::Mutex<VecDeque<RetryableRequest<SR>>>>;

/// Structure used to fulfill requests by requesting a signed request from a [`RequestSigningRegistry`],
/// before submitting it to a [`TransactionFulfiller`] with multiple attempts.
pub struct TickerFulfiller<R, SR, S, TF> {
    // Storage for pending, ready and requests to retry
    ready_requests: ReadyRequestStorage<SR>,
    retry_requests: RetryRequestStorage<SR>,

    // Number of requests that were processed this tick, use a mutex for longer locks
    num_left_to_fulfil_curr_tick: tokio::sync::Mutex<usize>,

    // Real transaction fulfiller
    fulfiller: TF,

    // Registry used to request and fetch ReadyRequests
    signer: Arc<S>,

    // Various configuration parameters
    max_fulfilment_per_tick: usize,
    retry_strategy: RetryStrategy,

    _r: PhantomData<R>,
}

/// Implementation of a [`RequestChannel`] using tokio's unbounded channel.
pub struct UnboundedRequestChannel<R> {
    tx: tokio::sync::mpsc::UnboundedSender<Vec<R>>,
}

/// Implementation of a [`Stopper`] using tokio's oneshot channel.
pub struct OneshotStopper {
    tx: tokio::sync::oneshot::Sender<()>,
}

impl<R, SR, S, TF> TickerFulfiller<R, SR, S, TF> {
    pub(crate) fn new(
        signer: S,
        transaction_fulfiller: TF,
        max_fulfilment_per_tick: usize,
        retry_strategy: RetryStrategy,
    ) -> Self {
        Self {
            ready_requests: Arc::new(tokio::sync::Mutex::new(VecDeque::new())),
            retry_requests: Arc::new(tokio::sync::Mutex::new(VecDeque::new())),
            num_left_to_fulfil_curr_tick: max_fulfilment_per_tick.into(),
            fulfiller: transaction_fulfiller,
            signer: Arc::new(signer),
            max_fulfilment_per_tick,
            retry_strategy,
            _r: PhantomData,
        }
    }
}

impl<R, SR, S, TF> Fulfiller for TickerFulfiller<R, SR, S, TF>
where
    R: Identifier + Send + Sync + 'static,
    SR: Identifier + Send + Sync + 'static,
    S: AsynchronousSigner<R> + Send + Sync + 'static,
    TF: TransactionFulfiller<SignedRequest = SR>,
{
    type Request = R;
    type SignedRequest = SR;
    type Signer = S;
    type TransactionFulfiller = TF;

    type RequestChannel = UnboundedRequestChannel<R>;
    type Stop = OneshotStopper;
}

impl<R, SR, S, TF> TickerBasedFulfiller for TickerFulfiller<R, SR, S, TF>
where
    R: Identifier + Send + Sync + 'static,
    SR: Identifier + Send + Sync + 'static,
    S: AsynchronousSigner<R, Signature = SR> + Send + Sync + 'static,
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

impl<R, SR, S, TF> TickerFulfiller<R, SR, S, TF>
where
    R: Identifier + Send + Sync + 'static,
    SR: Identifier + Send + Sync + 'static,
    S: AsynchronousSigner<R, Signature = SR> + Send + Sync + 'static,
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
            *self.num_left_to_fulfil_curr_tick.lock().await = self.max_fulfilment_per_tick;

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
        let cloned_token = cancellation_token.clone();
        let inner_fn = async move {
            tracing::debug!("Recv task processing new requests started");
            let mut signatures = FuturesUnordered::new(); // <BoxFuture<'a, Result<S::Signature, S::Error>>>;

            loop {
                tokio::select! {
                    // Handle new requests
                    requests = rx.recv() => {
                        let Some(requests) = requests else {
                            tracing::error!("Requests receiving channel closed... Exiting recv task.");
                            return;
                        };
                        tracing::debug!(
                            n_requests = requests.len(),
                            "Received new requests through channel"
                        );

                        // Request a new signature for each request
                        let futs = requests.into_iter().map(|r| {
                            let request_id = r.id().clone();
                            self.signer.async_sign(r).map(|res| (request_id, res)).boxed()
                        });
                        signatures.extend(futs);
                    },

                    // Handle new signature
                    signed_request = signatures.next(), if !signatures.is_empty() => {
                        match signed_request.expect("signed_request cannot return None due to precondition") {
                            (id, Ok(req)) => {
                                tracing::info!(request_id = %id, "Obtained signed request");

                                // Can we process the signature immediately?
                                let fulfil_now = {
                                    let mut num_left_to_fulfil_curr_tick = self.num_left_to_fulfil_curr_tick.lock().await;
                                    if *num_left_to_fulfil_curr_tick > 0 {
                                        *num_left_to_fulfil_curr_tick -= 1;
                                        true
                                    } else {
                                        false
                                    }
                                };

                                if fulfil_now {
                                    let req = RetryableRequest::new(req, self.retry_strategy);
                                    tokio::task::spawn(
                                        self.clone().fulfil_requests_task(vec![req], cloned_token.clone()),
                                    );
                                } else {
                                    self.ready_requests
                                        .lock()
                                        .await
                                        .push_back(req);
                                }
                            }
                            (id, Err(e)) => {
                                tracing::error!(error = ?e, request_id = %id, "Failed to obtain signed request from signer");
                            }
                        }
                    }
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping recv task, reason: cancellation token");
            },

            _ = inner_fn => (),
        }
    }

    /// Generates a vector of requests to fulfil for this tck.
    async fn requests_to_fulfil(&self) -> Vec<RetryableRequest<SR>> {
        // Lock mutex until the end of the function
        let mut num_fulfilment_curr_tick = self.num_left_to_fulfil_curr_tick.lock().await;

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
            .fulfil_requests(
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

        fn fulfil_requests<'lt_self, 'lt_sr, I>(
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
        let sk: Fr = MontFp!("0102030405060708091011121314151617181920");
        let cs = IbeIdentityOnBn254G1Suite::new_signer(b"TEST_IBE", 31337, sk);
        let signer = StandaloneSigner::new(cs.clone());

        // Static ephemeral pk / condition
        let eph_pk = ark_bn254::g2::G2Affine::generator();
        let condition = Bytes::from(b"MyCustomCondition");

        let retry_strategy = RetryStrategy::Never;
        let retry_never_fulfiller =
            TickerFulfiller::new(signer.clone(), FakeFulfiller, 100, retry_strategy);

        // Add a ready request to the fulfiller
        let req = DecryptionRequest {
            id: U256::from(1u64),
            condition,
            ciphertext: create_ciphertext(eph_pk),
        };
        let signed_req = signer.async_sign(req.clone()).await.unwrap();
        retry_never_fulfiller
            .ready_requests
            .lock()
            .await
            .push_back(signed_req);
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
    }

    #[tokio::test]
    async fn test_requests_to_fulfil_retry_times() {
        use ark_bn254::Fr;

        // bn254 ciphersuite
        let sk: Fr = MontFp!("0102030405060708091011121314151617181920");
        let cs = IbeIdentityOnBn254G1Suite::new_signer(b"TEST_IBE", 31337, sk);
        let signer = StandaloneSigner::new(cs.clone());

        // Static ephemeral pk / condition
        let eph_pk = ark_bn254::g2::G2Affine::generator();
        let condition = Bytes::from(b"MyCustomCondition");

        let retry_strategy = RetryStrategy::Times(2);
        let retries_fulfiller =
            TickerFulfiller::new(signer.clone(), FakeFulfiller, 100, retry_strategy);

        // Add a ready request to the fulfiller
        let req = DecryptionRequest {
            id: U256::from(1u64),
            condition,
            ciphertext: create_ciphertext(eph_pk),
        };
        let signed_req = signer.async_sign(req.clone()).await.unwrap();
        retries_fulfiller
            .ready_requests
            .lock()
            .await
            .push_back(signed_req);
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

        // Get a vec of requests to fulfil
        let requests_to_fulfil = retries_fulfiller.requests_to_fulfil().await;
        assert_eq!(requests_to_fulfil.len(), 1);
        assert_eq!(requests_to_fulfil[0].req.id(), &req.id);

        // Try to fulfil request again, which should fail due to the FakeFulfiller
        retries_fulfiller.fulfil_requests(requests_to_fulfil).await;
        // The request should be added to retry_requests
        assert_eq!(retries_fulfiller.retry_requests.lock().await.len(), 1);
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 0);

        // Get a vec of requests to fulfil
        let requests_to_fulfil = retries_fulfiller.requests_to_fulfil().await;
        assert_eq!(requests_to_fulfil.len(), 1);
        assert_eq!(requests_to_fulfil[0].req.id(), &req.id);

        // Try to fulfil request, which should fail due to the FakeFulfiller
        retries_fulfiller.fulfil_requests(requests_to_fulfil).await;
        // The request should be dropped on the third try (initial attempt + two retries)
        assert_eq!(retries_fulfiller.retry_requests.lock().await.len(), 0);
        assert_eq!(retries_fulfiller.ready_requests.lock().await.len(), 0);
    }
}
