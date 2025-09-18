//! Agent managing the state of the blocklock smart contract and forwarding fulfilled request to a
//! fulfiller's request channel.

use crate::RequestId;
use crate::agents::blocklock::BlockNumber;
use crate::agents::blocklock::condition_resolver::{
    BlocklockConditionResolver, BlocklockConditionResolverError, BlocklockConditionUpdate,
};
use crate::agents::blocklock::metrics::Metrics;
use crate::decryption_sender::DecryptionRequest;

use crate::fulfiller::RequestChannel;
use alloy::network::Ethereum;
use alloy::primitives::U256;
use alloy::primitives::ruint::FromUintError;
use alloy::providers::{Dynamic, MulticallBuilder, MulticallError, Provider};
use generated::blocklock::decryption_sender::DecryptionSender::{
    DecryptionRequested, DecryptionSenderInstance,
};
use generated::blocklock::decryption_sender::TypesLib::DecryptionRequest as GeneratedDecryptionRequest;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(thiserror::Error, Debug)]
pub enum BlocklockAgentError {
    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),
}

#[derive(thiserror::Error, Debug)]
enum InternalBlocklockAgentError {
    #[error(transparent)]
    ConditionResolver(#[from] BlocklockConditionResolverError),

    #[error("failed to cast solidity type to u64: {1}")]
    FromUintCast(#[source] FromUintError<u64>, &'static str),

    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),
}

pub struct BlocklockAgent<F, P> {
    scheme_id: String,
    sync_batch_size: usize,
    condition_resolver: BlocklockConditionResolver<RequestId>,
    last_seen_block: BlockNumber,
    last_seen_request_id: RequestId,
    decryption_requests: HashMap<RequestId, DecryptionRequest>,
    fulfiller_channel: F,
    decryption_sender: DecryptionSenderInstance<P>,
}

#[derive(Serialize, Deserialize)]
pub struct BlocklockAgentSavedState {
    last_seen_block: BlockNumber,
    last_seen_request_id: RequestId,
    decryption_requests: Vec<DecryptionRequest>,
}

impl<F, P> BlocklockAgent<F, P> {
    pub fn new(
        scheme_id: &str,
        sync_batch_size: usize, // batch size to use when sync'ing state
        fulfiller_channel: F,
        ro_instance: DecryptionSenderInstance<P>,
    ) -> Self {
        Self {
            scheme_id: scheme_id.to_owned(),
            sync_batch_size,
            condition_resolver: BlocklockConditionResolver::new(),
            last_seen_block: 0.into(),
            last_seen_request_id: U256::from(0u64).into(),
            decryption_requests: HashMap::new(),
            fulfiller_channel,
            decryption_sender: ro_instance,
        }
    }
}

impl<'a, F, P> BlocklockAgent<F, P>
where
    F: RequestChannel<Request = DecryptionRequest>,
    P: Provider + Clone + 'static,
{
    /// Create a new agent from a state
    pub async fn from_state(
        scheme_id: &str,
        sync_batch_size: usize, // batch size to use when sync'ing state
        fulfiller_channel: F,
        ro_instance: DecryptionSenderInstance<P>,
        state: BlocklockAgentSavedState,
    ) -> Result<Self, BlocklockAgentError> {
        let mut agent = Self::new(scheme_id, sync_batch_size, fulfiller_channel, ro_instance);
        agent.last_seen_block = state.last_seen_block;

        let unfulfilled_requests = agent
            .decryption_sender
            .getAllUnfulfilledRequestIds()
            .call()
            .await
            .map_err(|e| {
                BlocklockAgentError::Contract(e, "failed to call getAllUnfulfilledRequestIds")
            })?;
        let last_request_id = unfulfilled_requests.last().copied();
        let mut unfulfilled_requests: HashSet<_> = HashSet::from_iter(unfulfilled_requests);

        // Store requests present in state
        tracing::info!(
            num_requests = state.decryption_requests.len(),
            "Restoring requests from state"
        );
        state.decryption_requests.into_iter().for_each(|req| {
            let id = req.id;

            // Only store the request if it has not been fulfilled
            if unfulfilled_requests.remove(&id) {
                agent.try_store_request(req);
            }
        });

        // Recover any missed requests
        if !unfulfilled_requests.is_empty() {
            tracing::info!(
                num_missing_requests = unfulfilled_requests.len(),
                "Detected missing requests from saved state."
            );
            let missing_requests = unfulfilled_requests.into_iter();
            for batched_requests in agent.batch_get_requests(missing_requests, true) {
                let requests = match batched_requests.await {
                    Ok(requests) => requests,
                    Err(e) => {
                        Metrics::report_fetch_requests_error();
                        tracing::error!(error = %e, "Failed to get batched requests");
                        continue;
                    }
                };

                // Try to add each request to the store
                requests.into_iter().for_each(|req| {
                    agent.try_store_request(req.into());
                });
            }

            // Update the last_seen_request_id necessary
            let last_seen_request_id = last_request_id
                .expect("last seen request id cannot be empty if there are unfulfilled requests");
            agent.last_seen_request_id = RequestId::from(last_seen_request_id);
        }

        // Sync the agent with the contract
        if let Err(e) = agent.sync_state().await {
            tracing::error!(error = ?e, "Failed to sync agent with on-chain contract");
        }

        // Try to resolve conditions immediately
        agent.resolve_requests(BlocklockConditionUpdate::BlockNumber(agent.last_seen_block));

        // Return the agent
        Ok(agent)
    }

    /// Store the state of the agent
    pub fn save_state(&self) -> BlocklockAgentSavedState {
        let decryption_requests = self.decryption_requests.values().cloned();
        BlocklockAgentSavedState {
            last_seen_block: self.last_seen_block,
            last_seen_request_id: self.last_seen_request_id,
            decryption_requests: Vec::from_iter(decryption_requests),
        }
    }

    /// Handles a new block in the following way:
    ///     1) If the block has been seen before (i.e. lower than last seen block), simply ignore it.
    ///     2) If the block is not the next block in the sequence (i.e., we have missed some blocks),
    ///         we synchronize the current state of the contract with the on-chain state. Then, we
    ///         proceed with 3).
    ///     3) Else, the block is the next in the sequence. Check if any requests are resolved,
    ///         forward them to the fulfiller and remove them from the agent's storage.
    #[tracing::instrument(skip(self))]
    pub async fn handle_new_block(&'a mut self, mut block_number: BlockNumber) {
        tracing::debug!("Blocklock agent received NewBlock event: {block_number:?}");
        Metrics::report_chain_height(block_number.into());

        if self.last_seen_block >= block_number {
            // Ignore the block if it has already been processed
            tracing::info!("Block has already been processed");
            return;
        }
        if self.last_seen_block.0 + 1 != block_number.0 {
            // Missed some blocks, sync state and continue execution
            self.handle_missed_events().await;
            // Override block_number to the latest one, that may have been updated during sync
            block_number = self.last_seen_block;
        }

        // Try to resolve requests, if any
        self.resolve_requests(BlocklockConditionUpdate::BlockNumber(block_number));

        // Update the last seen block number
        self.last_seen_block = block_number;
    }

    /// Handles a new decryption requested event in the following way:
    ///     1) If the request id has already been seen, verify its the same as the one stored locally.
    ///     2) If the request id is not the next in the sequence, synchronize current state with
    ///         on-chain contract.
    ///     3) If no requests were missed, store the new request.
    #[tracing::instrument(skip_all, fields(request_id = %decryption_requested.requestId))]
    pub async fn handle_decryption_requested(&mut self, decryption_requested: DecryptionRequested) {
        tracing::info!(
            "Blocklock agent detected decryption requested event: {decryption_requested:?}"
        );

        let request_id: RequestId = decryption_requested.requestId.into();
        if self.last_seen_request_id >= request_id {
            // Request has already been seen
            self.handle_seen_request(request_id, decryption_requested);
        } else if self.last_seen_request_id.0.add(U256::from(1u64)) != request_id.0 {
            // Not the next request_id, we likely missed some requests
            self.handle_missed_events().await;
        } else {
            // No missed requests
            self.handle_new_request(request_id, decryption_requested);
        }
    }

    /// Check if any requests are resolved, forward them to the fulfiller and remove them from the
    /// agent's storage.
    fn resolve_requests(&mut self, condition_update: BlocklockConditionUpdate) {
        // Signal the updated block number to the condition resolver and get back an iterator of resolved requests
        let satisfied_request_ids = self.condition_resolver.update_condition(&condition_update);

        // Filter the requests and log incoherent states (i.e., request id missing from local requests)
        let (request_ids, satisfied_requests): (Vec<_>, Vec<_>) = satisfied_request_ids
            .filter_map(|request_id| {
                let Some(decryption_request) = self.decryption_requests.remove(&request_id) else {
                    tracing::error!(request_id = ?request_id, "Condition resolver resolved a condition with an identifier not in local storage, incoherent state");
                    None?
                };
                Some((request_id, decryption_request))
            })
            .unzip();

        if !satisfied_requests.is_empty() {
            tracing::info!(
                "Detected {} requests to fulfil: {:?}",
                satisfied_requests.len(),
                satisfied_requests
            );

            // Send requests to fulfiller
            self.fulfiller_channel.register_requests(satisfied_requests);

            // Remove requests from resolver
            self.remove_requests(request_ids.iter());
        }
    }

    /// Handle a new request by adding it to the condition resolver.
    fn handle_new_request(
        &mut self,
        request_id: RequestId,
        decryption_requested: DecryptionRequested,
    ) {
        // No missed requests, set the last seen to the current request id
        self.last_seen_request_id = request_id;

        // Ignore request if scheme id is not supported
        if decryption_requested.schemeID != self.scheme_id {
            tracing::info!(
                "Ignoring request due to unsupported scheme id: got `{}`, expected `{}`",
                decryption_requested.schemeID,
                self.scheme_id
            );
            return;
        }

        // Otherwise, try to store the request.
        self.try_store_request(decryption_requested.into());
    }

    /// Handle a decryption request that has already been seen.
    fn handle_seen_request(
        &self,
        request_id: RequestId,
        decryption_requested: DecryptionRequested,
    ) {
        // That request id was already seen, make sure that it is equal to whatever is stored internally
        tracing::info!("Decryption request has already been seen, checking against internal state");

        // Return if a decryption request with that id already exists, otherwise log and continue processing
        if let Some(stored_decryption_request) = self.decryption_requests.get(&request_id) {
            if stored_decryption_request.eq(&decryption_requested.into()) {
                // Same request, simply ignore it
                tracing::info!("Ignoring decryption request, same object already stored");
            } else {
                tracing::error!(
                    "Detected two decryption requests with same ids but different contents"
                );
            };
        } else {
            tracing::warn!(
                "Decryption request already seen, but not stored in internal state, likely already fulfilled"
            );
        }
    }

    /// Handle missed blocks / requests by synchronizing the current state with the on-chain state.
    async fn handle_missed_events(&mut self) {
        let num_requests_pre_sync = self.decryption_requests.len();

        // Missed some requests, try to sync state
        match self.sync_state().await {
            Ok(_) => {
                let num_requests_post_sync = self.decryption_requests.len();
                Metrics::report_sync_success();
                tracing::info!(
                    num_requests_pre_sync,
                    num_requests_post_sync,
                    "State synchronized from on-chain contract"
                );
            }
            Err(e) => {
                Metrics::report_sync_error();
                tracing::error!(error = ?e, "Failed to synchronize state from on-chain contract");
            }
        }
    }

    /// Try to add a request to the condition resolver and local storage.
    /// If it fails, log an error and return.
    fn try_store_request(&mut self, decryption_requested: DecryptionRequest) {
        let request_id = RequestId::from(decryption_requested.id);

        // Try to add the request to the condition resolver, log an error and return otherwise.
        if let Err(e) = self
            .condition_resolver
            .add_condition(request_id, decryption_requested.condition.as_ref())
        {
            Metrics::report_storage_error();
            tracing::error!(error = %e, request_id = ?request_id, "Failed to add request to decryption resolver");
            return;
        }

        // If all went right, add it to the local storage
        self.decryption_requests
            .insert(request_id, decryption_requested);
        Metrics::report_decryption_requested();
    }

    /// Remove requests from the condition resolver and the local storage.
    fn remove_requests<'lt_self, 'lt_req>(
        &'lt_self mut self,
        requests: impl IntoIterator<Item = &'lt_req RequestId>,
    ) {
        // Remove requests from resolver
        requests.into_iter().for_each(|req_id| {
            // Remove from condition resolver
            self.condition_resolver.remove_condition(req_id);

            // Remove it from local storage
            self.decryption_requests.remove(req_id);
        });
    }

    /// Sync the state of the agent with the on-chain contract.
    #[tracing::instrument(skip(self))]
    async fn sync_state(&mut self) -> Result<(), InternalBlocklockAgentError> {
        let last_seen_request_id = self.last_seen_request_id.0;

        // Query the current block first to make sure that it's either the latest, or a previous block
        //  at the end of the sync.
        let last_block_number = match self.decryption_sender.provider().get_block_number().await {
            Ok(block_number) => BlockNumber(block_number),
            Err(e) => {
                tracing::error!(error = ?e, "Failed to fetch the last block number, using last_seen_block(`{:?}`) instead", self.last_seen_block);
                self.last_seen_block
            }
        };

        // Recover last_request_id
        let last_request_id = self
            .decryption_sender
            .lastRequestId()
            .call()
            .await
            .map_err(|e| {
                InternalBlocklockAgentError::Contract(
                    e,
                    "failed to call DecryptionSender::lastRequestId()",
                )
            })?;
        let missing_requests = last_request_id.sub(last_seen_request_id);

        // Hopefully, we haven't missed more than 2**64 requests
        let missing_requests: u64 = missing_requests.try_into().map_err(|e| {
            InternalBlocklockAgentError::FromUintCast(
                e,
                "cannot handle more than 2**64 missing requests",
            )
        })?;
        Metrics::report_missing_events(missing_requests);
        tracing::info!("Sync detected {missing_requests} missing requests");

        // Create an iterator of missing requests. Scan returns last_seen_request_id + 1, up to last_request_id
        let missing_requests = std::iter::repeat_n(U256::from(1), missing_requests as usize).scan(
            last_seen_request_id,
            |curr_request_id, one| {
                *curr_request_id += one;
                Some(*curr_request_id)
            },
        );

        // Loop over each chunk
        for batched_requests in self.batch_get_requests(missing_requests, false) {
            // Execute the future to get back requests
            let requests = match batched_requests.await {
                Ok(requests) => requests,
                Err(e) => {
                    tracing::error!(error = %e, "Failed to get batched requests");
                    continue;
                }
            };

            requests.into_iter().for_each(|req| {
                if req.inner.isFulfilled {
                    // If the request is fulfilled, remove it from the agent
                    self.remove_requests(std::iter::once(&RequestId::from(req.id)));
                } else {
                    // Otherwise, store it.
                    self.try_store_request(req.into());
                }
            });
        }

        // Update the last seen block
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
        Item = impl Future<Output = Result<Vec<TypesLibDecryptionRequest>, MulticallError>> + 'lt_out,
    > + 'lt_out {
        let missing_requests: Vec<_> = missing_requests.into_iter().collect();
        let iter = missing_requests
            .chunks(self.sync_batch_size)
            .enumerate()
            .map(move |(batch_id, batch)| {
                // Create owned values to remove lifetime bound on 'lt_self in the iterator / the futures.
                let batch = batch.to_vec();
                let decryption_sender = self.decryption_sender.to_owned();
                let scheme_id = self.scheme_id.to_owned();
                async move {
                    let batch_size = batch.len();
                    let (req_ids, multicall) =
                        Self::create_multicall_with_ids(batch.into_iter(), decryption_sender);

                    tracing::debug_span!("requests_multicall", batch_id = batch_id);
                    tracing::debug!(
                        "Sending aggregate call to rpc to recover {} missing requests",
                        batch_size
                    );
                    let batched_requests = match multicall.aggregate().await {
                        Ok(requests) => requests,
                        Err(e) => {
                            tracing::error!(error = ?e, "Failed to execute multicall");
                            Err(e)?
                        }
                    };

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
                        TypesLibDecryptionRequest::from_typeslib_decryption_request(id, req)
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
        decryption_sender: DecryptionSenderInstance<P>,
    ) -> (
        Vec<U256>,
        MulticallBuilder<
            Dynamic<generated::blocklock::decryption_sender::DecryptionSender::getRequestCall>,
            P,
            Ethereum,
        >,
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

impl Default for BlocklockAgentSavedState {
    fn default() -> Self {
        Self {
            decryption_requests: vec![],
            last_seen_request_id: U256::from(0u64).into(),
            last_seen_block: 0.into(),
        }
    }
}

/// Helper struct used for TypesLibDecryptionRequest
struct TypesLibDecryptionRequest {
    inner: GeneratedDecryptionRequest,
    id: U256,
}

impl TypesLibDecryptionRequest {
    pub fn from_typeslib_decryption_request(
        request_id: U256,
        value: GeneratedDecryptionRequest,
    ) -> Self {
        Self {
            id: request_id,
            inner: value,
        }
    }
}

impl From<TypesLibDecryptionRequest> for DecryptionRequest {
    fn from(value: TypesLibDecryptionRequest) -> Self {
        Self {
            id: value.id,
            condition: value.inner.condition,
            ciphertext: value.inner.ciphertext,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::blocklock::BlocklockCondition;
    use crate::ibe_helper::{IbeIdentityOnBn254G1Suite, PairingIbeCipherSuite, PairingIbeSigner};
    use alloy::consensus::constants::ETH_TO_WEI;
    use alloy::hex;
    use alloy::network::{Ethereum, TransactionBuilder};
    use alloy::primitives::{Address, Bytes};
    use alloy::providers::ext::AnvilApi;
    use alloy::providers::{Provider, ProviderBuilder, RootProvider, WalletProvider};
    use alloy::rpc::types::TransactionRequest;
    use alloy::sol_types::{SolEvent, SolValue};
    use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
    use ark_ff::{BigInteger, MontFp, PrimeField};
    use ark_std::UniformRand;
    use ark_std::rand::thread_rng;
    use futures_util::StreamExt;
    use futures_util::future::join_all;
    use generated::blocklock::blocklock_sender::BlocklockSender;
    use generated::blocklock::blocklock_signature_scheme::BlocklockSignatureScheme;
    use generated::blocklock::decryption_sender::DecryptionSender;
    use generated::blocklock::decryption_sender::DecryptionSender::{
        DecryptionRequested, DecryptionSenderInstance,
    };
    use generated::blocklock::mock_blocklock_receiver::BLS;
    use generated::blocklock::mock_blocklock_receiver::MockBlocklockReceiver;
    use generated::blocklock::mock_blocklock_receiver::MockBlocklockReceiver::MockBlocklockReceiverInstance;
    use generated::blocklock::mock_blocklock_receiver::TypesLib::Ciphertext;
    use generated::blocklock::signature_scheme_address_provider::SignatureSchemeAddressProvider;
    use generated::blocklock::uups_proxy::UUPSProxy;
    use std::collections::VecDeque;
    use std::ops::AddAssign;
    use std::rc::Rc;
    use std::str::FromStr;
    use std::sync::Mutex;
    use std::time::Duration;
    use utils::serialize::point::PointSerializeUncompressed;

    const SK: ark_bn254::Fr =
        MontFp!("3742516928081212610066329633174215531795997236046512785163691679786522890575");
    const BLOCKLOCK_SCHEME_ID: &str = "BN254-BLS-BLOCKLOCK";
    const OTHER_SCHEME_ID: &str = "OTHER-SCHEME-ID";
    const MULTICALL3_PRESIGNED_TX: &str = "f90f538085174876e800830f42408080b90f00608060405234801561001057600080fd5b50610ee0806100206000396000f3fe6080604052600436106100f35760003560e01c80634d2301cc1161008a578063a8b0574e11610059578063a8b0574e1461025a578063bce38bd714610275578063c3077fa914610288578063ee82ac5e1461029b57600080fd5b80634d2301cc146101ec57806372425d9d1461022157806382ad56cb1461023457806386d516e81461024757600080fd5b80633408e470116100c65780633408e47014610191578063399542e9146101a45780633e64a696146101c657806342cbb15c146101d957600080fd5b80630f28c97d146100f8578063174dea711461011a578063252dba421461013a57806327e86d6e1461015b575b600080fd5b34801561010457600080fd5b50425b6040519081526020015b60405180910390f35b61012d610128366004610a85565b6102ba565b6040516101119190610bbe565b61014d610148366004610a85565b6104ef565b604051610111929190610bd8565b34801561016757600080fd5b50437fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0140610107565b34801561019d57600080fd5b5046610107565b6101b76101b2366004610c60565b610690565b60405161011193929190610cba565b3480156101d257600080fd5b5048610107565b3480156101e557600080fd5b5043610107565b3480156101f857600080fd5b50610107610207366004610ce2565b73ffffffffffffffffffffffffffffffffffffffff163190565b34801561022d57600080fd5b5044610107565b61012d610242366004610a85565b6106ab565b34801561025357600080fd5b5045610107565b34801561026657600080fd5b50604051418152602001610111565b61012d610283366004610c60565b61085a565b6101b7610296366004610a85565b610a1a565b3480156102a757600080fd5b506101076102b6366004610d18565b4090565b60606000828067ffffffffffffffff8111156102d8576102d8610d31565b60405190808252806020026020018201604052801561031e57816020015b6040805180820190915260008152606060208201528152602001906001900390816102f65790505b5092503660005b8281101561047757600085828151811061034157610341610d60565b6020026020010151905087878381811061035d5761035d610d60565b905060200281019061036f9190610d8f565b6040810135958601959093506103886020850185610ce2565b73ffffffffffffffffffffffffffffffffffffffff16816103ac6060870187610dcd565b6040516103ba929190610e32565b60006040518083038185875af1925050503d80600081146103f7576040519150601f19603f3d011682016040523d82523d6000602084013e6103fc565b606091505b50602080850191909152901515808452908501351761046d577f08c379a000000000000000000000000000000000000000000000000000000000600052602060045260176024527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060445260846000fd5b5050600101610325565b508234146104e6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f4d756c746963616c6c333a2076616c7565206d69736d6174636800000000000060448201526064015b60405180910390fd5b50505092915050565b436060828067ffffffffffffffff81111561050c5761050c610d31565b60405190808252806020026020018201604052801561053f57816020015b606081526020019060019003908161052a5790505b5091503660005b8281101561068657600087878381811061056257610562610d60565b90506020028101906105749190610e42565b92506105836020840184610ce2565b73ffffffffffffffffffffffffffffffffffffffff166105a66020850185610dcd565b6040516105b4929190610e32565b6000604051808303816000865af19150503d80600081146105f1576040519150601f19603f3d011682016040523d82523d6000602084013e6105f6565b606091505b5086848151811061060957610609610d60565b602090810291909101015290508061067d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060448201526064016104dd565b50600101610546565b5050509250929050565b43804060606106a086868661085a565b905093509350939050565b6060818067ffffffffffffffff8111156106c7576106c7610d31565b60405190808252806020026020018201604052801561070d57816020015b6040805180820190915260008152606060208201528152602001906001900390816106e55790505b5091503660005b828110156104e657600084828151811061073057610730610d60565b6020026020010151905086868381811061074c5761074c610d60565b905060200281019061075e9190610e76565b925061076d6020840184610ce2565b73ffffffffffffffffffffffffffffffffffffffff166107906040850185610dcd565b60405161079e929190610e32565b6000604051808303816000865af19150503d80600081146107db576040519150601f19603f3d011682016040523d82523d6000602084013e6107e0565b606091505b506020808401919091529015158083529084013517610851577f08c379a000000000000000000000000000000000000000000000000000000000600052602060045260176024527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060445260646000fd5b50600101610714565b6060818067ffffffffffffffff81111561087657610876610d31565b6040519080825280602002602001820160405280156108bc57816020015b6040805180820190915260008152606060208201528152602001906001900390816108945790505b5091503660005b82811015610a105760008482815181106108df576108df610d60565b602002602001015190508686838181106108fb576108fb610d60565b905060200281019061090d9190610e42565b925061091c6020840184610ce2565b73ffffffffffffffffffffffffffffffffffffffff1661093f6020850185610dcd565b60405161094d929190610e32565b6000604051808303816000865af19150503d806000811461098a576040519150601f19603f3d011682016040523d82523d6000602084013e61098f565b606091505b506020830152151581528715610a07578051610a07576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060448201526064016104dd565b506001016108c3565b5050509392505050565b6000806060610a2b60018686610690565b919790965090945092505050565b60008083601f840112610a4b57600080fd5b50813567ffffffffffffffff811115610a6357600080fd5b6020830191508360208260051b8501011115610a7e57600080fd5b9250929050565b60008060208385031215610a9857600080fd5b823567ffffffffffffffff811115610aaf57600080fd5b610abb85828601610a39565b90969095509350505050565b6000815180845260005b81811015610aed57602081850181015186830182015201610ad1565b81811115610aff576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b600082825180855260208086019550808260051b84010181860160005b84811015610bb1578583037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001895281518051151584528401516040858501819052610b9d81860183610ac7565b9a86019a9450505090830190600101610b4f565b5090979650505050505050565b602081526000610bd16020830184610b32565b9392505050565b600060408201848352602060408185015281855180845260608601915060608160051b870101935082870160005b82811015610c52577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0888703018452610c40868351610ac7565b95509284019290840190600101610c06565b509398975050505050505050565b600080600060408486031215610c7557600080fd5b83358015158114610c8557600080fd5b9250602084013567ffffffffffffffff811115610ca157600080fd5b610cad86828701610a39565b9497909650939450505050565b838152826020820152606060408201526000610cd96060830184610b32565b95945050505050565b600060208284031215610cf457600080fd5b813573ffffffffffffffffffffffffffffffffffffffff81168114610bd157600080fd5b600060208284031215610d2a57600080fd5b5035919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b600082357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81833603018112610dc357600080fd5b9190910192915050565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112610e0257600080fd5b83018035915067ffffffffffffffff821115610e1d57600080fd5b602001915036819003821315610a7e57600080fd5b8183823760009101908152919050565b600082357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc1833603018112610dc357600080fd5b600082357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1833603018112610dc357600080fdfea2646970667358221220bb2b5c71a328032f97c676ae39a1ec2148d3e5d6f73d95e9b17910152d61f16264736f6c634300080c00331ca0edce47092c0f398cebf3ffc267f05c8e7076e3b89445e0fe50f6332273d4569ba01b0b9d000e19b24c5869b0fc3b22b0d6fa47cd63316875cbbd577d76e6fde086";

    fn pk() -> ark_bn254::G2Affine {
        let pk = ark_bn254::G2Projective::generator() * SK;
        pk.into_affine()
    }

    fn to_sol_g2(p: ark_bn254::G2Affine) -> ([U256; 2], [U256; 2]) {
        let (x, y) = p.xy().unwrap();
        let x0 = U256::from_be_bytes::<32>(x.c0.into_bigint().to_bytes_be().try_into().unwrap());
        let x1 = U256::from_be_bytes::<32>(x.c1.into_bigint().to_bytes_be().try_into().unwrap());
        let y0 = U256::from_be_bytes::<32>(y.c0.into_bigint().to_bytes_be().try_into().unwrap());
        let y1 = U256::from_be_bytes::<32>(y.c1.into_bigint().to_bytes_be().try_into().unwrap());

        ([x0, x1], [y0, y1])
    }

    fn sol_pk() -> ([U256; 2], [U256; 2]) {
        to_sol_g2(pk())
    }

    async fn deploy_contracts<P>(
        provider: P,
    ) -> (
        DecryptionSenderInstance<P>,
        MockBlocklockReceiverInstance<P>,
    )
    where
        P: Provider + WalletProvider + Clone,
    {
        let (pk_x, pk_y) = sol_pk();
        let wallet_address = provider.default_signer_address();

        // Deploy signature scheme address provider
        let signature_scheme_address_provider =
            SignatureSchemeAddressProvider::deploy(&provider, wallet_address)
                .await
                .unwrap();

        // Deploy blocklock scheme
        let blocklock_scheme = BlocklockSignatureScheme::deploy(&provider, pk_x, pk_y)
            .await
            .unwrap();
        let blocklock_scheme_id = blocklock_scheme.SCHEME_ID().call().await.unwrap();
        signature_scheme_address_provider
            .updateSignatureScheme(blocklock_scheme_id, blocklock_scheme.address().to_owned())
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Deploy other scheme id (which is blocklock, but with a different id)
        let blocklock_scheme = BlocklockSignatureScheme::deploy(&provider, pk_x, pk_y)
            .await
            .unwrap();
        signature_scheme_address_provider
            .updateSignatureScheme(
                OTHER_SCHEME_ID.to_owned(),
                blocklock_scheme.address().to_owned(),
            )
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Deploy decryption sender
        let decryption_sender_impl = DecryptionSender::deploy(&provider).await.unwrap();
        let decryption_sender_proxy = UUPSProxy::deploy(
            &provider,
            decryption_sender_impl.address().to_owned(),
            Bytes::new(),
        )
        .await
        .unwrap();
        let decryption_sender = DecryptionSender::DecryptionSenderInstance::new(
            decryption_sender_proxy.address().to_owned(),
            provider.clone(),
        );
        decryption_sender
            .initialize(
                wallet_address,
                signature_scheme_address_provider.address().to_owned(),
            )
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        let blocklock_impl = BlocklockSender::deploy(&provider).await.unwrap();
        let blocklock_proxy =
            UUPSProxy::deploy(&provider, blocklock_impl.address().to_owned(), Bytes::new())
                .await
                .unwrap();
        let blocklock = BlocklockSender::BlocklockSenderInstance::new(
            blocklock_proxy.address().to_owned(),
            &provider,
        );
        blocklock
            .initialize(wallet_address, decryption_sender_proxy.address().to_owned())
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
        // set blocklockSender contract config
        let max_gas_limit = 500_000;
        let gas_after_payment_calculation = 400_000;
        let fulfillment_flat_fee_native_ppm = 100_000;
        let wei_per_unit_gas = 3_000_000;
        let bls_pairing_check_overhead = 800_000;
        let native_premium_percentage = 10;
        let gas_for_call_exact_check = 5000;
        blocklock
            .setConfig(
                max_gas_limit,
                gas_after_payment_calculation,
                fulfillment_flat_fee_native_ppm,
                wei_per_unit_gas,
                bls_pairing_check_overhead,
                native_premium_percentage,
                gas_for_call_exact_check,
            )
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Deploy Multicall3 contract
        deploy_imulticall3(&provider).await;

        // Deploy mockblocklock receiver and fund it in order to create requests
        let mockblocklock_receiver = MockBlocklockReceiver::MockBlocklockReceiverInstance::deploy(
            provider.clone(),
            blocklock_proxy.address().to_owned(),
        )
        .await
        .unwrap();
        mockblocklock_receiver
            .createSubscriptionAndFundNative()
            .value(U256::from(10 * ETH_TO_WEI))
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        (decryption_sender.clone(), mockblocklock_receiver)
    }

    async fn deploy_imulticall3<P>(provider: &P)
    where
        P: Provider + WalletProvider,
    {
        // Send 0.1 eth to the deployer address
        let deployer_address =
            Address::from_str("0x05f32b3cc3888453ff71b01135b34ff8e41263f2").unwrap();
        let tx = TransactionRequest::default()
            .with_from(provider.default_signer_address())
            .with_to(deployer_address)
            .with_value(U256::from(100_000_000_000_000_000u128));
        provider
            .send_transaction(tx)
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Publish pre-signed multicall3 deployment transaction
        let multicall3_raw_tx = hex::decode(MULTICALL3_PRESIGNED_TX).unwrap();
        provider
            .send_raw_transaction(&multicall3_raw_tx)
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
    }

    fn rand_ciphertext() -> Ciphertext {
        let u = ark_bn254::G2Affine::rand(&mut thread_rng());
        let (x, y) = to_sol_g2(u);
        Ciphertext {
            u: BLS::PointG2 { x, y },
            v: Bytes::from(vec![0; 32]),
            w: Bytes::from(vec![0; 4]),
        }
    }

    async fn register_ciphertext<P>(
        decryption_sender: &DecryptionSenderInstance<P>,
        mockblocklock_receiver: &MockBlocklockReceiverInstance<P>,
        ct: Ciphertext,
        condition: BlocklockCondition,
    ) -> DecryptionRequested
    where
        P: Provider + WalletProvider,
    {
        // Create stream to log events
        let mut decryption_requested_stream = decryption_sender
            .DecryptionRequested_filter()
            .watch()
            .await
            .unwrap()
            .poller
            .with_poll_interval(Duration::from_millis(20))
            .into_stream();

        mockblocklock_receiver
            .createTimelockRequestWithSubscription(500_000, condition.into(), ct)
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        let decryption_requested_log = decryption_requested_stream.next().await.unwrap();
        DecryptionRequested::decode_log(&decryption_requested_log[0].clone().into())
            .unwrap()
            .data
    }

    #[derive(Clone)]
    struct RequestChannelBuffer<R>(Rc<Mutex<Vec<R>>>);

    impl<R> Default for RequestChannelBuffer<R> {
        fn default() -> Self {
            Self(Rc::new(Mutex::new(Vec::default())))
        }
    }

    impl<R> RequestChannel for RequestChannelBuffer<R> {
        type Request = R;

        fn register_requests<I>(&mut self, requests: I)
        where
            I: IntoIterator,
            I::Item: Into<Self::Request>,
        {
            self.0
                .lock()
                .unwrap()
                .extend(requests.into_iter().map(Into::into));
        }
    }

    #[derive(Clone)]
    struct UnimplementedProvider;

    impl Provider for UnimplementedProvider {
        fn root(&self) -> &RootProvider<Ethereum> {
            unimplemented!("test provider")
        }
    }

    #[tokio::test]
    async fn should_increase_block_number_sequentially_without_rpc_calls() {
        let provider = UnimplementedProvider;
        let decryption_sender = DecryptionSenderInstance::new(Address::default(), provider);

        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            RequestChannelBuffer::default(),
            decryption_sender,
        );
        blocklock.handle_new_block(1.into()).await;
        assert_eq!(blocklock.last_seen_block, 1.into());

        blocklock.handle_new_block(1.into()).await;
        assert_eq!(blocklock.last_seen_block, 1.into());

        blocklock.handle_new_block(2.into()).await;
        assert_eq!(blocklock.last_seen_block, 2.into());

        blocklock.handle_new_block(3.into()).await;
        assert_eq!(blocklock.last_seen_block, 3.into());
    }

    #[tokio::test]
    async fn should_increase_request_id_sequentially_without_rpc_calls() {
        let provider = UnimplementedProvider;
        let decryption_sender = DecryptionSenderInstance::new(Address::default(), provider);

        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            RequestChannelBuffer::default(),
            decryption_sender,
        );

        let mut req = DecryptionRequested {
            requestId: U256::from(1),
            schemeID: BLOCKLOCK_SCHEME_ID.to_owned(),
            condition: BlocklockCondition::BlockNumber(999.into()).into(),
            ciphertext: Bytes::from(b"ciphertext"),
            callback: Address::default(),
            requestedAt: U256::from(0),
        };
        blocklock.handle_decryption_requested(req.clone()).await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(1).into());

        blocklock.handle_decryption_requested(req.clone()).await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(1).into());

        req.requestId.add_assign(U256::from(1));
        blocklock.handle_decryption_requested(req.clone()).await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(2).into());

        req.requestId.add_assign(U256::from(1));
        blocklock.handle_decryption_requested(req.clone()).await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(3).into());
    }

    #[tokio::test]
    async fn should_ignore_requests_with_unsupported_scheme_id() {
        let provider = UnimplementedProvider;
        let decryption_sender = DecryptionSenderInstance::new(Address::default(), provider);

        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            RequestChannelBuffer::default(),
            decryption_sender,
        );

        // Try to store a request with an invalid scheme id, it should not be stored
        let req = DecryptionRequested {
            requestId: U256::from(1),
            schemeID: OTHER_SCHEME_ID.to_owned(),
            condition: BlocklockCondition::BlockNumber(999.into()).into(),
            ciphertext: Bytes::from(b"ciphertext"),
            callback: Address::default(),
            requestedAt: U256::from(0),
        };
        blocklock.handle_decryption_requested(req).await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(1).into());
        assert_eq!(blocklock.decryption_requests.len(), 0);

        // Try to store a request with a valid scheme id, it should be stored
        let req = DecryptionRequested {
            requestId: U256::from(2),
            schemeID: BLOCKLOCK_SCHEME_ID.to_owned(),
            condition: BlocklockCondition::BlockNumber(999.into()).into(),
            ciphertext: Bytes::from(b"ciphertext"),
            callback: Address::default(),
            requestedAt: U256::from(0),
        };
        blocklock.handle_decryption_requested(req).await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(2).into());
        assert_eq!(blocklock.decryption_requests.len(), 1);
    }

    #[tokio::test]
    async fn should_send_request_through_channel_on_block_reached() {
        let provider = UnimplementedProvider;
        let decryption_sender = DecryptionSenderInstance::new(Address::default(), provider);

        let request_channel_buffer = RequestChannelBuffer::default();
        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            request_channel_buffer.clone(),
            decryption_sender,
        );

        let req_1_block_2 = DecryptionRequested {
            requestId: U256::from(1),
            schemeID: BLOCKLOCK_SCHEME_ID.to_owned(),
            condition: BlocklockCondition::BlockNumber(2.into()).into(),
            ciphertext: Bytes::from(b"ciphertext"),
            callback: Address::default(),
            requestedAt: U256::from(0),
        };
        let req_2_block_5 = DecryptionRequested {
            requestId: U256::from(2),
            schemeID: BLOCKLOCK_SCHEME_ID.to_owned(),
            condition: BlocklockCondition::BlockNumber(5.into()).into(),
            ciphertext: Bytes::from(b"ciphertext"),
            callback: Address::default(),
            requestedAt: U256::from(0),
        };
        let req_3_block_5 = DecryptionRequested {
            requestId: U256::from(3),
            schemeID: BLOCKLOCK_SCHEME_ID.to_owned(),
            condition: BlocklockCondition::BlockNumber(5.into()).into(),
            ciphertext: Bytes::from(b"ciphertext"),
            callback: Address::default(),
            requestedAt: U256::from(0),
        };

        // Register all three requests
        blocklock
            .handle_decryption_requested(req_1_block_2.clone())
            .await;
        blocklock
            .handle_decryption_requested(req_2_block_5.clone())
            .await;
        blocklock
            .handle_decryption_requested(req_3_block_5.clone())
            .await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(3).into());

        // Go to block 2
        blocklock.handle_new_block(1.into()).await;
        blocklock.handle_new_block(2.into()).await;

        // Request 1 should have been transmitted to the request channel
        {
            let reqs = request_channel_buffer.0.lock().unwrap();
            assert!(
                vec![req_1_block_2.clone()]
                    .into_iter()
                    .all(|r| reqs.contains(&r.into()))
            );
            assert_eq!(reqs.len(), 1);
        }

        // Go to block 5
        blocklock.handle_new_block(3.into()).await;
        blocklock.handle_new_block(4.into()).await;
        blocklock.handle_new_block(5.into()).await;

        // Request 1, 2, and 3 should have been transmitted to the request channel
        {
            let reqs = request_channel_buffer.0.lock().unwrap();
            assert!(
                vec![req_1_block_2, req_2_block_5, req_3_block_5]
                    .into_iter()
                    .all(|r| reqs.contains(&r.into()))
            );
            assert_eq!(reqs.len(), 3);
        }
    }

    #[tokio::test]
    async fn should_sync_with_contract_when_missing_blocks() {
        let provider = ProviderBuilder::new().connect_anvil_with_wallet();
        let (decryption_sender, mockblocklock_receiver) = deploy_contracts(provider).await;

        let request_channel_buffer = RequestChannelBuffer::default();
        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            request_channel_buffer.clone(),
            decryption_sender.clone(),
        );

        // Request id 1 w/ a valid scheme id
        let req_1_ct = rand_ciphertext();
        register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            req_1_ct.clone(),
            BlocklockCondition::BlockNumber(999.into()),
        )
        .await;

        // Request id 2 w/ an valid scheme id
        let req_2_ct = rand_ciphertext();
        register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            req_2_ct.clone(),
            BlocklockCondition::BlockNumber(999.into()),
        )
        .await;

        // Go to block 100
        blocklock.handle_new_block(100.into()).await;

        // Request 1 and 2 should now be stored
        assert_eq!(blocklock.last_seen_request_id, U256::from(2).into());
        assert_eq!(blocklock.decryption_requests.len(), 2);
        let r_1 = blocklock
            .decryption_requests
            .get(&U256::from(1).into())
            .cloned();
        let r_2 = blocklock
            .decryption_requests
            .get(&U256::from(2).into())
            .cloned();
        assert_eq!(
            r_1.map(|r| r.ciphertext),
            Some(req_1_ct.abi_encode().into())
        );
        assert_eq!(
            r_2.map(|r| r.ciphertext),
            Some(req_2_ct.abi_encode().into())
        );
    }

    #[tokio::test]
    async fn should_sync_with_contract_when_missing_requests() {
        let provider = ProviderBuilder::new().connect_anvil_with_wallet();
        let (decryption_sender, mockblocklock_receiver) = deploy_contracts(provider).await;

        let request_channel_buffer = RequestChannelBuffer::default();
        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            request_channel_buffer.clone(),
            decryption_sender.clone(),
        );

        for _ in 1..=5 {
            register_ciphertext(
                &decryption_sender,
                &mockblocklock_receiver,
                rand_ciphertext(),
                BlocklockCondition::BlockNumber(999.into()),
            )
            .await;
        }

        let mut decryption_requested: VecDeque<_> = join_all((1..=5).map(async |req_id| {
            let req = decryption_sender
                .getRequest(U256::from(1))
                .call()
                .await
                .unwrap();
            DecryptionRequested {
                requestId: U256::from(req_id),
                schemeID: req.schemeID,
                ciphertext: req.ciphertext,
                condition: req.condition,
                callback: req.callback,
                requestedAt: U256::from(0),
            }
        }))
        .await
        .into();

        // Notify of first request
        blocklock
            .handle_decryption_requested(decryption_requested.pop_front().unwrap())
            .await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(1).into());
        assert_eq!(blocklock.decryption_requests.len(), 1);

        // Notify of last request
        blocklock
            .handle_decryption_requested(decryption_requested.pop_back().unwrap())
            .await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(5).into());
        assert_eq!(blocklock.decryption_requests.len(), 5);
    }

    #[tokio::test]
    async fn sync_should_ignore_fulfilled_requests() {
        let provider = ProviderBuilder::new().connect_anvil_with_wallet();
        let (decryption_sender, mockblocklock_receiver) = deploy_contracts(provider).await;

        let request_channel_buffer = RequestChannelBuffer::default();
        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            request_channel_buffer.clone(),
            decryption_sender.clone(),
        );

        // Register two requests
        let req_ct = rand_ciphertext();
        let condition = BlocklockCondition::BlockNumber(999.into());
        register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            req_ct.clone(),
            condition,
        )
        .await;
        register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            req_ct.clone(),
            condition,
        )
        .await;

        // Create ciphersuite for blocklock
        let cs = IbeIdentityOnBn254G1Suite::new_signer(b"BLOCKLOCK", 31337, SK);
        let identity = cs.h1(&condition.to_bytes());
        let signature = cs.decryption_key(identity); // the signature is the ibe decryption key

        // Manually fulfil request 1
        decryption_sender
            .fulfillDecryptionRequest(
                U256::from(1),
                Bytes::from(b"decryption key"),
                signature.ser_uncompressed().unwrap().into(),
            )
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
        let fulfilled_reqs = decryption_sender
            .getAllFulfilledRequestIds()
            .call()
            .await
            .unwrap();
        assert_eq!(fulfilled_reqs.len(), 1);
        assert_eq!(fulfilled_reqs[0], U256::from(1));

        // Sync the agent
        blocklock.sync_state().await.unwrap();
        assert_eq!(blocklock.last_seen_request_id, U256::from(2).into());
        assert_eq!(blocklock.decryption_requests.len(), 1);
        assert_eq!(
            blocklock
                .decryption_requests
                .get(&U256::from(1).into())
                .map(|r| r.id),
            None
        );
        assert_eq!(
            blocklock
                .decryption_requests
                .get(&U256::from(2).into())
                .map(|r| r.id),
            Some(U256::from(2))
        );
    }

    #[tokio::test]
    async fn saved_state_should_preserve_conditions() {
        let provider = ProviderBuilder::new().connect_anvil_with_wallet();
        let (decryption_sender, mockblocklock_receiver) = deploy_contracts(provider.clone()).await;

        // Create ciphersuite for blocklock
        let cs = IbeIdentityOnBn254G1Suite::new_signer(b"BLOCKLOCK", 31337, SK);

        let request_channel_buffer = RequestChannelBuffer::default();
        let mut blocklock = BlocklockAgent::new(
            BLOCKLOCK_SCHEME_ID,
            20,
            request_channel_buffer.clone(),
            decryption_sender.clone(),
        );

        let curr_block = provider.get_block_number().await.unwrap();

        // Register three ciphertexts on-chain
        let req_1_block_plus_5 = register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            rand_ciphertext(),
            BlocklockCondition::BlockNumber((curr_block + 5).into()),
        )
        .await;
        let req_2_block_500 = register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            rand_ciphertext(),
            BlocklockCondition::BlockNumber(500.into()),
        )
        .await;
        let req_3_block_500 = register_ciphertext(
            &decryption_sender,
            &mockblocklock_receiver,
            rand_ciphertext(),
            BlocklockCondition::BlockNumber(500.into()),
        )
        .await;

        // Register all three requests with the agent
        blocklock
            .handle_decryption_requested(req_1_block_plus_5.clone())
            .await;
        blocklock
            .handle_decryption_requested(req_2_block_500.clone())
            .await;
        blocklock
            .handle_decryption_requested(req_3_block_500.clone())
            .await;
        assert_eq!(blocklock.last_seen_request_id, U256::from(3).into());

        // Skip 10 blocks
        provider.anvil_mine(Some(5), None).await.unwrap();
        blocklock.handle_new_block((curr_block + 5).into()).await;

        // Request 1 should have been transmitted to the request channel
        {
            let reqs = request_channel_buffer.0.lock().unwrap();
            assert!(
                vec![req_1_block_plus_5.clone()]
                    .into_iter()
                    .all(|r| reqs.contains(&r.into()))
            );
            assert_eq!(reqs.len(), 1);
        }

        // Manually fulfil request 1
        let identity = cs.h1(req_1_block_plus_5.condition.as_ref());
        let signature = cs.decryption_key(identity); // the signature is the ibe decryption key
        decryption_sender
            .fulfillDecryptionRequest(
                U256::from(1),
                Bytes::from(b"decryption key"),
                signature.ser_uncompressed().unwrap().into(),
            )
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        // Save and restore state
        let saved_state = blocklock.save_state();
        let blocklock = BlocklockAgent::from_state(
            &blocklock.scheme_id,
            blocklock.sync_batch_size,
            blocklock.fulfiller_channel,
            blocklock.decryption_sender,
            saved_state,
        )
        .await
        .expect("failed to sync state");

        // It should now contain two requests
        let reqs: Vec<_> = blocklock.decryption_requests.values().cloned().collect();
        assert_eq!(reqs.len(), 2);
        assert!(
            vec![req_2_block_500, req_3_block_500]
                .into_iter()
                .all(|r| reqs.contains(&r.into()))
        );
    }
}
