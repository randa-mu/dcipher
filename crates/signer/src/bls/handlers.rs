//! Handle internal signing requests and partial signatures received from other nodes.

use crate::bls::metrics::Metrics;
use crate::bls::{
    BatchKnownPartials, BatchReplayPartials, BlsSignatureRequest, BlsSigner, BlsThresholdSigner,
    G1, G1Affine, G2, G2Affine, Group, NetworkMessage, PartialSignature,
    PartialSignatureWithRequest, ReplayPartial, StoredSignatureRequest,
    lagrange_points_interpolate_at,
};
use crate::dsigner::BlsSignatureAlgorithm;
use ark_ec::{AffineRepr, CurveGroup};
use dcipher_network::{ReceivedMessage, TransportSender};
use futures_util::{Stream, StreamExt};
use itertools::{Either, izip};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;
use utils::display::LogBytes;
use utils::dst::NamedCurveGroup;
use utils::serialize::point::{
    PointDeserializeCompressed, PointSerializeCompressed, PointSerializeUncompressed,
};

/// Map either with the same expression
macro_rules! map_either {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            Either::Left($pattern) => Either::Left($result),
            Either::Right($pattern) => Either::Right($result),
        }
    };
}

impl<BLS> BlsThresholdSigner<BLS>
where
    BLS: BlsSigner + Clone + Send + Sync + 'static,
    G1Affine<BLS>:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
    G2Affine<BLS>:
        PointSerializeCompressed + PointDeserializeCompressed + PointSerializeUncompressed,
{
    pub(super) async fn sign_requests_loop<T>(
        self: Arc<Self>,
        mut rx_reqs: UnboundedReceiver<BlsSignatureRequest>,
        tx_to_network: T,
        cancellation_token: CancellationToken,
    ) where
        T: TransportSender<Identity = u16>,
    {
        #[cfg(feature = "rayon")]
        use rayon::prelude::*;

        const MAX_BATCH_SIZE: usize = 256;

        let inner_fn = async move {
            let mut reqs = Vec::with_capacity(MAX_BATCH_SIZE);

            loop {
                let count = rx_reqs.recv_many(&mut reqs, MAX_BATCH_SIZE).await;
                if count == 0 {
                    tracing::warn!("Registry has dropped message sender, exiting recv loop");
                    break;
                };

                let (reqs, missing_partials) = self.filter_requests(reqs.drain(..));

                // Request partials on each of the missing messages
                let m =
                    serde_cbor::to_vec(&NetworkMessage::<BLS>::ReplayPartials(missing_partials))
                        .expect("serialization to be infallible");
                if let Err(e) = tx_to_network.broadcast(m).await {
                    tracing::error!(error = ?e, "Failed to request partials")
                }

                let span =
                    tracing::debug_span!("threshold_signer_batch", batch_size = count).entered();
                #[cfg(feature = "rayon")]
                tracing::debug!(requests_count = reqs.len(), "Signing messages in parallel");
                #[cfg(not(feature = "rayon"))]
                tracing::debug!(requests_count = reqs.len(), "Signing requests sequentially");
                let span = span.exit();

                // Create signatures in parallel if rayon is enabled, otherwise use a standard iter
                #[cfg(feature = "rayon")]
                let iter = reqs.par_iter();
                #[cfg(not(feature = "rayon"))]
                let iter = reqs.iter();
                let partials: Vec<_> = iter.filter_map(|(req, stored_req)| {
                    tracing::info!(msg = %LogBytes(&req.m), app = ?req.args.app(), alg = ?req.alg, "Received new message to sign");
                    match self.sign(&req.m, &stored_req.dst, &req.alg) {
                        Ok(sig) => Some(sig),
                        Err(e) => {
                            tracing::error!(error = ?e, msg = %LogBytes(&req.m), "Failed to sign message.");
                            None
                        }
                    }
                }).collect();

                // Collect points to aggregate
                let to_aggregate: Vec<Either<Vec<_>, Vec<_>>> = {
                    let mut partials_cache = self
                        .partials_cache
                        .lock()
                        .expect("a thread panicked with the mutex");

                    // We filter with a sequential iterator here due to side effects
                    izip!(partials.iter(), reqs.iter()).filter_map(|(partial_sig, (req, stored_req))| {
                        tracing::info!(msg = %LogBytes(&stored_req.m), party_id = self.id, "Storing partial signature on message");
                        let partials = partials_cache.get_or_insert_mut(stored_req.to_owned(), Default::default);
                        partials.insert(
                            self.id,
                            PartialSignature {
                                id: self.id,
                                sig: *partial_sig,
                            },
                        );

                        // Do we have exactly t partials?
                        if partials.len() == usize::from(self.t) {
                            Some(Self::collect_partials_into_points(&req.alg, partials))
                        } else {
                            None
                        }
                    }).collect()
                };

                let span = span.entered();
                #[cfg(feature = "rayon")]
                tracing::debug!(
                    requests_count = reqs.len(),
                    "Aggregating signatures in parallel"
                );
                #[cfg(not(feature = "rayon"))]
                tracing::debug!(
                    requests_count = reqs.len(),
                    "Aggregating signatures sequentially"
                );
                let _span = span.exit();

                // Do the aggregation with a parallel iterator if rayon is enabled
                #[cfg(feature = "rayon")]
                let iter = to_aggregate.into_par_iter();
                #[cfg(not(feature = "rayon"))]
                let iter = to_aggregate.into_iter();
                let signatures: Vec<_> = iter
                    .map(|points| {
                        map_either!(points, points => lagrange_points_interpolate_at(&points, 0).into_affine())
                    })
                    .collect();

                // We now have a bunch of signatures, store them
                {
                    let mut signatures_cache = self
                        .signatures_cache
                        .lock()
                        .expect("a thread panicked with the mutex");

                    // side effects, sequential iterator
                    for (sig, (_, stored_req)) in izip!(signatures.into_iter(), reqs.iter()) {
                        if let Some(Either::Right(tx_channel)) =
                            signatures_cache.put(stored_req.to_owned(), Either::Left(sig))
                        {
                            // If there previously was a channel stored at the entry, also send signature through it
                            tx_channel.send_replace(Some(sig));
                        }
                    }
                }

                // Send it to other nodes with libp2p if threshold greater than 1
                if self.t > 1 {
                    futures_util::future::join_all(partials.into_iter().zip(reqs).map(
                        async |(sig, (req, _))| {
                            let partial = PartialSignatureWithRequest { sig, req };

                            let m = serde_cbor::to_vec(&NetworkMessage::PartialSignature(partial))
                                .expect("serialization should always work");
                            Metrics::report_partials_sent(1);
                            if let Err(e) = tx_to_network.broadcast(m).await {
                                tracing::error!(error = ?e, "Failed to send message to signer");
                            }
                        },
                    ))
                    .await;
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping recv loop due to cancellation token");
            },

            _ = inner_fn => (),
        }
    }

    /// Filter a list of requests into a list of requests to sign, and a list of replay requests
    fn filter_requests(
        &self,
        reqs: impl IntoIterator<IntoIter: ExactSizeIterator<Item = BlsSignatureRequest>>,
    ) -> (
        Vec<(BlsSignatureRequest, StoredSignatureRequest)>,
        BatchReplayPartials,
    ) {
        let reqs = reqs.into_iter();

        let all_parties_excluding_self =
            BTreeSet::from_iter((1..=self.n).filter(|id| id != &self.id));

        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked holding the mutex");

        // We need to do isolate requests that have been signed by ourselves, and count which partials we're missing (if less than t - 1 sigs)
        let mut reqs_to_sign = Vec::with_capacity(reqs.len());
        let mut missing_partials = Vec::with_capacity(reqs.len());

        for req in reqs {
            // Filter unsupported requests
            let Some(stored_req) = self.try_parse_req(req.clone()) else {
                tracing::error!(app = ?req.args.app(), alg = ?req.alg, "Received a request to sign an unsupported request");
                continue;
            };

            let mut to_sign = true;

            // Has the request already been signed by any node?
            let required_partials = if let Some(partials_map) = partials_cache.get(&stored_req) {
                // Do not sign requests that were already signed by self
                if partials_map.contains_key(&self.id) {
                    tracing::debug!(msg = ?req.m, "Received message signing request, but message was already signed");
                    to_sign = false;
                }

                // Here, we decide to request partials iff we need some to aggregate
                if partials_map.len() <= self.t as usize {
                    // need partials from all but self, minus parties that have already signed
                    all_parties_excluding_self
                        .difference(&partials_map.keys().copied().collect())
                        .copied()
                        .collect()
                } else {
                    // we could avoid signing messages here, but might be good for other nodes to get our shares nonetheless
                    vec![] // we don't need anyone's shares
                }
            } else {
                // never signed by any node, need shares from everyone
                all_parties_excluding_self.clone().into_iter().collect()
            };

            // sign, unless already signed by self
            if to_sign {
                reqs_to_sign.push((req.clone(), stored_req));
            }

            // add missing partials
            if !required_partials.is_empty() {
                missing_partials.push(ReplayPartial {
                    req,
                    missing_partials: required_partials,
                });
            }
        }

        (reqs_to_sign, missing_partials)
    }

    #[allow(clippy::type_complexity)]
    fn collect_partials_into_points(
        alg: &BlsSignatureAlgorithm,
        partials: &mut BTreeMap<u16, PartialSignature<Group<BLS>>>,
    ) -> Either<Vec<(u64, G1<BLS>)>, Vec<(u64, G2<BLS>)>> {
        let points = partials.values();
        if <G1<BLS> as NamedCurveGroup>::CURVE_ID == alg.curve.into() {
            // Collect the g1 partials
            let points = points
                .map(|partial| {
                    (
                        u64::from(partial.id),
                        partial
                            .sig
                            .either()
                            .left()
                            .expect("g2 points stored in g1 request")
                            .into_group(),
                    )
                })
                .collect::<Vec<_>>();
            Either::Left(points)
        } else {
            // Collect the g2 partials
            let points = points
                .map(|partial| {
                    (
                        u64::from(partial.id),
                        partial
                            .sig
                            .either()
                            .right()
                            .expect("g1 points stored in g2 request")
                            .into_group(),
                    )
                })
                .collect::<Vec<_>>();
            Either::Right(points)
        }
    }

    pub(super) async fn network_recv_loop<T, E>(
        self: Arc<Self>,
        mut network_stream: impl Stream<Item = Result<ReceivedMessage<u16>, E>> + Unpin + Send,
        tx_new_message_to_sign: UnboundedSender<BlsSignatureRequest>,
        tx_to_network: T,
        cancellation_token: CancellationToken,
    ) where
        T: TransportSender<Identity = u16>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let inner_fn = async move {
            loop {
                let ReceivedMessage {
                    sender: sender_id,
                    content: partial,
                    ..
                } = match network_stream.next().await {
                    Some(Ok(m)) => m,
                    Some(Err(e)) => {
                        tracing::error!(error = ?e, "Failed to receive message");
                        continue; // receive next message
                    }
                    None => {
                        tracing::warn!("Libp2p node has dropped sender, exiting recv loop");
                        break; // stop the loop
                    }
                };

                let m: NetworkMessage<_> = match serde_cbor::from_slice(&partial) {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::error!(sender_id, error = ?e, "Failed to decode network message");
                        continue;
                    }
                };

                match m {
                    NetworkMessage::PartialSignature(partial) => self.handle_partial_from_network(
                        partial,
                        sender_id,
                        &tx_new_message_to_sign,
                    ),
                    NetworkMessage::ReplayPartials(reqs) => {
                        for req in reqs {
                            self.handle_replay_partials_from_network(
                                req,
                                sender_id,
                                &tx_to_network,
                            )
                            .await;
                        }
                    }
                    NetworkMessage::KnownPartials(partials) => {
                        self.handle_known_partials_from_network(
                            partials,
                            sender_id,
                            &tx_new_message_to_sign,
                        );
                    }
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                tracing::info!("Stopping recv loop due to cancellation token");
            },

            _ = inner_fn => (),
        }
    }

    fn handle_partial_from_network(
        &self,
        partial: PartialSignatureWithRequest<BLS>,
        sender: u16,
        new_message_to_sign: &UnboundedSender<BlsSignatureRequest>,
    ) {
        let PartialSignatureWithRequest { sig, req } = partial;

        Metrics::report_partials_received(1);

        // Get the dst, making sure the request is supported
        let Some(dst) = self
            .filter
            .get_rfc9380_dst_if_supported(&req.args, &req.alg)
        else {
            tracing::warn!(sender_id = sender, app = ?req.args.app(), alg = ?req.alg, "Received partial with unsupported app");
            return;
        };
        let stored_req = StoredSignatureRequest {
            dst,
            m: req.m.clone(),
        };

        // Verify the validity of the partial signature for the specified id
        match self.try_verify(&req.m, &stored_req.dst, sig, &sender, &req.alg) {
            Ok(true) => (),
            Ok(false) => {
                tracing::error!(sender_id = sender, "Received invalid partial signature");
                Metrics::report_invalid_partials(1);
                return;
            }
            Err(e) => {
                // Algorithm should be supported at this point
                tracing::warn!(sender_id = sender, error = ?e, "Failed to verify partial");
                return;
            }
        }

        // Valid signature, add it to our cache
        self.store_and_process_partial(
            stored_req.clone(),
            PartialSignature { id: sender, sig },
            &req,
        );

        if self.eager_signing {
            // If eager signing is enabled and the message has not been signed already,
            // request to broadcast a partial signature on that message
            if !self.partial_issued(&stored_req) {
                new_message_to_sign
                    .send(req)
                    .expect("failed to forward message to signer");
            }
        }
    }

    #[tracing::instrument(skip_all, fields(requester_id))]
    async fn handle_replay_partials_from_network<T>(
        &self,
        req: ReplayPartial,
        requester_id: u16,
        tx_to_network: &T,
    ) where
        T: TransportSender<Identity = u16>,
    {
        let ReplayPartial {
            missing_partials,
            req,
        } = req;
        tracing::info!(requester_id, "Received replay partials request from node");

        let stored_req = match self.try_parse_req(req.clone()) {
            Some(value) => value,
            None => {
                tracing::warn!(requester_id, app = ?req.args.app(), alg = ?req.alg, "Received partial request with unsupported app");
                return;
            }
        };

        let partials: Vec<_> = {
            let mut partials_cache = self
                .partials_cache
                .lock()
                .expect("a thread panicked with the mutex");
            partials_cache
                .get(&stored_req)
                .map(|p| {
                    missing_partials
                        .iter()
                        .filter_map(|i| p.get(i).cloned())
                        .collect()
                })
                .unwrap_or_default()
        };

        if partials.is_empty() {
            tracing::info!(requester_id, msg = %LogBytes(&stored_req.m), "No partials in cache");
        } else {
            tracing::info!(
                requester_id,
                partials_count = partials.len(),
                msg = %LogBytes(&stored_req.m),
                "Sending partials to requester"
            );
            let m = serde_cbor::to_vec(&NetworkMessage::KnownPartials(vec![(req, partials)]))
                .expect("serialization should always work");

            if let Err(e) = tx_to_network.send_single(m, requester_id).await {
                tracing::error!(error = ?e, "Failed to send partials back to sender");
            }
        }
    }

    /// Parse a [`BlsSignatureRequest`] into a [`StoredSignatureRequest`] if the specified algorithm is
    /// supported.
    fn try_parse_req(&self, req: BlsSignatureRequest) -> Option<StoredSignatureRequest> {
        // Get the dst, making sure the request is supported
        let dst = self
            .filter
            .get_rfc9380_dst_if_supported(&req.args, &req.alg)?;

        Some(StoredSignatureRequest { dst, m: req.m })
    }

    #[tracing::instrument(skip_all, fields(sender_id))]
    fn handle_known_partials_from_network(
        &self,
        known_partials_batch: BatchKnownPartials<BLS>,
        sender_id: u16,
        new_message_to_sign: &UnboundedSender<BlsSignatureRequest>,
    ) {
        let known_partials_batch = {
            let mut partials_cache = self.partials_cache.lock().expect("an unpoisoned mutex");
            let signatures_cache = self.signatures_cache.lock().expect("an unpoisoned mutex");

            known_partials_batch.into_iter().filter_map(|(req, partials)| {
                // Filter unsupported apps requests
                let stored_req = match self.try_parse_req(req.clone()) {
                    Some(value) => value,
                    None => {
                        tracing::warn!(sender_id, app = ?req.args.app(), alg = ?req.alg, "Received partial replay for unsupported app");
                        None?
                    },
                };

                // Filter already signed requests
                if let Some(Either::Left(_sig)) = signatures_cache.peek(&stored_req) {
                    tracing::debug!(sender_id, "Filtering known partials for request that was already signed");
                    None?
                }

                // Filter partials that were already received & verified
                let partials = match partials_cache.get(&stored_req) {
                    Some(partials_cache) => {
                        partials.into_iter().filter(|p| {
                            // keep partials that are not already in cache
                            !partials_cache.contains_key(&p.id)
                        }).collect()
                    }
                    None => {
                        // no partials in cache, keep all partials
                        partials
                    }
                };

                Some((req, partials))
            })
            .collect::<Vec<_>>() // collect to drop the mutexes
        };

        for (req, partials) in known_partials_batch {
            tracing::info!(sender_id, partials_count = partials.len(), msg = %LogBytes(&req.m), "Received partials from node");

            for partial in partials {
                let partial_w_req = PartialSignatureWithRequest {
                    sig: partial.sig,
                    req: req.clone(),
                };
                self.handle_partial_from_network(partial_w_req, partial.id, new_message_to_sign)
            }
        }
    }

    /// Verify whether a partial has already been issued or not.
    fn partial_issued(&self, req: &StoredSignatureRequest) -> bool {
        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked holding the mutex");
        let Some(partials_map) = partials_cache.get(req) else {
            return false;
        };

        partials_map.contains_key(&self.id)
    }

    /// Store a partial signature to the cache, and aggregate it if there are enough partials.
    fn store_and_process_partial(
        &self,
        stored_req: StoredSignatureRequest,
        partial: PartialSignature<Group<BLS>>,
        req: &BlsSignatureRequest,
    ) {
        tracing::info!(msg = %LogBytes(&stored_req.m), party_id = partial.id, "Storing partial signature on message");
        let mut partials_cache = self
            .partials_cache
            .lock()
            .expect("a thread panicked with the mutex");
        let partials = partials_cache.get_or_insert_mut(stored_req.clone(), Default::default);
        partials.insert(
            partial.id,
            PartialSignature {
                id: partial.id,
                sig: partial.sig,
            },
        );

        // Do we have exactly t partials?
        if partials.len() == usize::from(self.t) {
            // Aggregate the partials with Lagrange's interpolation
            let points = Self::collect_partials_into_points(&req.alg, partials);
            let sig = map_either!(points, points => lagrange_points_interpolate_at(&points, 0).into_affine());

            // We now have a signature, store it
            let mut signatures_cache = self
                .signatures_cache
                .lock()
                .expect("a thread panicked with the mutex");
            if let Some(Either::Right(tx_channel)) =
                signatures_cache.put(stored_req, Either::Left(sig))
            {
                // If there previously was a channel stored at the entry, also send signature through it
                tx_channel.send_replace(Some(sig));
            }
        }
    }
}
