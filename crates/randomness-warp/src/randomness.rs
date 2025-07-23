use fulfiller_core::{RequestId, BlockNumber, fulfiller::RequestChannel, signature_sender::{SignatureRequest, contracts::{SignatureSender, TypesLib}}};
use crate::metrics::Metrics;
use alloy::network::Ethereum;
use alloy::primitives::U256;
use alloy::primitives::ruint::FromUintError;
use alloy::providers::{Dynamic, MulticallBuilder, MulticallError, Provider};
use std::ops::{Add, Sub};
use tracing::Instrument;

#[derive(thiserror::Error, Debug)]
enum InternalRandomnessAgentError {
    #[error("failed to cast solidity type to u64: {1}")]
    FromUintCast(#[source] FromUintError<u64>, &'static str),

    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),
}

pub struct RandomnessAgent<F, P> {
    scheme_id: String,
    sync_batch_size: usize,
    last_seen_block: BlockNumber,
    last_seen_request_id: RequestId,
    fulfiller_channel: F,
    signature_sender: SignatureSender::SignatureSenderInstance<P>,
}

impl<F, P> RandomnessAgent<F, P> {
    pub fn new(
        scheme_id: &str,
        sync_batch_size: usize, // batch size to use when sync'ing state
        fulfiller_channel: F,
        ro_instance: SignatureSender::SignatureSenderInstance<P>,
    ) -> Self {
        Self {
            scheme_id: scheme_id.to_owned(),
            sync_batch_size,
            last_seen_block: 0.into(),
            last_seen_request_id: U256::from(0u64).into(),
            fulfiller_channel,
            signature_sender: ro_instance,
        }
    }
}

impl<F, P> RandomnessAgent<F, P>
where
    F: RequestChannel<Request = SignatureRequest>,
    P: Provider + Clone + 'static,
{
    /// Handles a new block in the following way:
    ///     1) If the block has been seen before (i.e. lower than last seen block), simply ignore it.
    ///     2) else, if the block is not the next block in the sequence (i.e., we have missed some blocks),
    ///         we synchronize the current state of the contract with the on-chain state.
    ///     3) Otherwise, update last seen block
    #[tracing::instrument(skip(self))]
    pub async fn handle_new_block(&mut self, block_number: BlockNumber) {
        tracing::debug!("Randomness agent received NewBlock event: {block_number:?}");
        Metrics::report_chain_height(block_number.into());

        if self.last_seen_block >= block_number {
            // Ignore the block if it has already been processed
            tracing::debug!("Block has already been processed");
        } else if self.last_seen_block.0 + 1 != block_number.0 {
            // Missed some blocks, sync state and continue execution
            self.handle_missed_events().await;
        } else {
            // Next block in the sequence
            self.last_seen_block = block_number;
        }
    }

    /// Handles a new signature requested event in the following way:
    ///     1) If the request id has already been seen, ignore it.
    ///     2) If the request id is not the next in the sequence, synchronize current state with
    ///         on-chain contract.
    ///     3) If no requests were missed, forward the request to the fulfiller.
    #[tracing::instrument(skip_all, fields(request_id = %signature_requested.requestID))]
    pub async fn handle_signature_requested(
        &mut self,
        signature_requested: SignatureSender::SignatureRequested,
    ) {
        tracing::info!(
            "Randomness agent detected signature requested event: {signature_requested:?}"
        );

        let request_id: RequestId = signature_requested.requestID.into();
        if self.last_seen_request_id >= request_id {
            // Request has already been seen
            tracing::info!("Signature request has already been seen.");
        } else if self.last_seen_request_id.0.add(U256::from(1u64)) != request_id.0 {
            // Not the next request_id, we likely missed some requests
            self.handle_missed_events().await;
        } else {
            // No missed requests
            self.handle_new_request(request_id, signature_requested);
        }
    }

    /// Handle a new request by parsing it and forwarding it to the fulfiller.
    fn handle_new_request(
        &mut self,
        request_id: RequestId,
        signature_requested: SignatureSender::SignatureRequested,
    ) {
        // No missed requests, set the last seen to the current request id
        self.last_seen_request_id = request_id;

        // Ignore request if scheme id is not supported
        if signature_requested.schemeID != self.scheme_id {
            tracing::info!(
                "Ignoring request due to unsupported scheme id: got `{}`, expected `{}`",
                signature_requested.schemeID,
                self.scheme_id
            );
            return;
        }

        // Otherwise, forward the request to the fulfiller
        self.fulfiller_channel
            .register_requests(vec![SignatureRequest::from(signature_requested)]);
        Metrics::report_randomness_requested();
    }

    /// Handle missed blocks / requests by synchronizing the current state with the on-chain state.
    async fn handle_missed_events(&mut self) {
        // Missed some requests, try to sync state
        match self.sync_state().await {
            Ok(_) => {
                Metrics::report_sync_success();
                tracing::info!("State synchronized from on-chain contract");
            }
            Err(e) => {
                Metrics::report_sync_error();
                tracing::error!(error = ?e, "Failed to synchronize state from on-chain contract");
            }
        }
    }

    /// Sync the state of the agent with the on-chain contract.
    #[tracing::instrument(skip(self))]
    pub async fn sync_state(&mut self) -> Result<(), InternalRandomnessAgentError> {
        let last_seen_request_id = self.last_seen_request_id.0;

        // Query the current block first to make sure that it's either the latest, or a previous block
        //  at the end of the sync.
        let last_block_number = match self.signature_sender.provider().get_block_number().await {
            Ok(block_number) => {
                Metrics::report_chain_height(block_number);
                BlockNumber(block_number)
            }
            Err(e) => {
                tracing::error!(error = ?e, "Failed to fetch the last block number, using last_seen_block(`{:?}`) instead", self.last_seen_block);
                self.last_seen_block
            }
        };

        // Recover last_request_id
        let last_request_id = self
            .signature_sender
            .lastRequestID()
            .call()
            .await
            .map_err(|e| {
                InternalRandomnessAgentError::Contract(
                    e,
                    "failed to call SignatureSender::lastRequestID()",
                )
            })?;
        let missing_requests = last_request_id.sub(last_seen_request_id);

        // Hopefully, we haven't missed more than 2**64 requests
        let missing_requests: u64 = missing_requests.try_into().map_err(|e| {
            InternalRandomnessAgentError::FromUintCast(
                e,
                "cannot handle more than 2**64 missing requests",
            )
        })?;
        tracing::info!("Sync detected {missing_requests} missing requests");
        Metrics::report_missing_events(missing_requests);

        // Create an iterator of missing requests. Scan returns last_seen_request_id + 1, up to last_request_id
        let missing_requests = std::iter::repeat_n(U256::from(1), missing_requests as usize).scan(
            last_seen_request_id,
            |curr_request_id, one| {
                *curr_request_id += one;
                Some(*curr_request_id)
            },
        );

        // Loop over each chunk
        for batched_requests in self.batch_get_requests(missing_requests, true) {
            // Execute the future to get back requests
            let requests = match batched_requests.await {
                Ok(requests) => requests,
                Err(e) => {
                    Metrics::report_fetch_requests_error();
                    tracing::error!(error = %e, "Failed to get batched requests");
                    continue;
                }
            };

            // Send each request to the fulfiller
            self.fulfiller_channel.register_requests(requests);
        }

        // Update the last seen block & request id
        self.last_seen_block = last_block_number;
        self.last_seen_request_id = last_request_id.into();
        Ok(())
    }

    /// Creates an iterator of future that can be used to obtain a batch of requests.
    /// Both the futures and iterators are lazy, hence they are only awaited once evaluating the iterator,
    /// and awaiting the future.
    fn batch_get_requests<'lt_self, 'lt_iter, 'lt_out>(
        &'lt_self self,
        missing_requests: impl IntoIterator<Item = U256> + 'lt_iter,
        only_unfulfilled: bool,
    ) -> impl Iterator<
        Item = impl Future<Output = Result<Vec<TypesLibSignatureRequest>, MulticallError>> + 'lt_out,
    > + 'lt_out {
        let missing_requests: Vec<_> = missing_requests.into_iter().collect();
        let iter = missing_requests
            .chunks(self.sync_batch_size)
            .enumerate()
            .map(move |(batch_id, batch)| {
                // Create owned values to remove lifetime bound on 'lt_self in the iterator / the futures.
                let batch = batch.to_vec();
                let signature_sender = self.signature_sender.to_owned();
                let scheme_id = self.scheme_id.to_owned();
                async move {
                    let batch_size = batch.len();
                    let (req_ids, multicall) =
                        Self::create_multicall_with_ids(batch.into_iter(), signature_sender);

                    tracing::debug!(
                        "Sending aggregate call to rpc to recover {} missing requests",
                        batch_size
                    );
                    let batched_requests = multicall
                        .aggregate()
                        .instrument(tracing::debug_span!("requests_multicall", batch_id = batch_id))
                        .await
                        .map_err(|e| {
                            tracing::error!(error = ?e, "Failed to execute multicall");
                            e
                        })?;

                    // Filter only valid requests, i.e., that have a valid scheme id, and,
                    // optionally, that are unfulfilled.
                    let requests = req_ids.into_iter().zip(batched_requests).filter_map(|(id, req)| {
                        // A request with a null scheme implies that the request does not exist => error
                        if req.schemeID.is_empty() {
                            Metrics::report_scheme_error();
                            tracing::error!(request_id = %id, returned_request = ?req, "Failed to obtain request details");
                            None
                        } else if req.schemeID != scheme_id {
                            tracing::debug!(request_id = %id, "Ignoring request with unsupported scheme id");
                            None
                        } else if only_unfulfilled && req.isFulfilled {
                            tracing::debug!(request_id = %id, "Ignoring fulfilled request");
                            None
                        } else {
                            Some((id, req))
                        }
                    }).map(|(id, req)| {
                        TypesLibSignatureRequest::from_typeslib_signature_request(id, req)
                    });

                    Ok(requests.collect())
                }
            });

        let futures: Vec<_> = iter.collect();
        futures.into_iter()
    }

    /// Creates a new multicall that fetches multiple requests.
    /// Returns a vector of request ids, and a multicall.
    fn create_multicall_with_ids(
        requests: impl IntoIterator<Item = U256> + ExactSizeIterator,
        decryption_sender: SignatureSender::SignatureSenderInstance<P>,
    ) -> (
        Vec<U256>,
        MulticallBuilder<Dynamic<SignatureSender::getRequestCall>, P, Ethereum>,
    ) {
        let batch_size = requests.len();
        let mut request_ids = Vec::with_capacity(batch_size);
        let mut multicall = MulticallBuilder::new_dynamic(decryption_sender.provider().clone());
        for request_id in requests {
            let get_request_call = decryption_sender.getRequest(request_id);
            multicall = multicall.add_dynamic(get_request_call);
            request_ids.push(request_id);
        }

        (request_ids, multicall)
    }
}

/// Helper struct used for TypesLibSignaturesRequest
struct TypesLibSignatureRequest {
    inner: TypesLib::SignatureRequest,
    id: U256,
}

impl TypesLibSignatureRequest {
    pub fn from_typeslib_signature_request(
        request_id: U256,
        value: TypesLib::SignatureRequest,
    ) -> Self {
        Self {
            id: request_id,
            inner: value,
        }
    }
}

impl From<TypesLibSignatureRequest> for SignatureRequest {
    fn from(value: TypesLibSignatureRequest) -> Self {
        Self {
            id: value.id,
            condition: value.inner.condition,
            message_to_sign: value.inner.message,
        }
    }
}
