//! Handles message received by other nodes.

use crate::aba::crain20::coin::CoinToss;
use crate::aba::crain20::messages::{AbaMessage, EstimateMessage};
use crate::aba::crain20::{AbaCrain20, AbaCrain20Config, AbaState, PerPartyStorage};
use crate::helpers::{PartyId, SessionId};
use crate::network::broadcast_with_self;
use dcipher_network::{ReceivedMessage, Transport};
use futures::StreamExt;
use std::collections::HashSet;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug, error, info, trace, warn};

impl<CT, CK, T> AbaCrain20<CT, CK, T>
where
    CT: CoinToss,
    T: Transport<Identity = PartyId>,
{
    /// Thread responsible for receiving all types of ABA messages and transmitting notifications.
    pub(super) async fn recv_thread(
        sid: SessionId,
        config: Arc<AbaCrain20Config<CT, CK>>,
        receiver: T::ReceiveMessageStream,
        sender: T::Sender,
        cancel: CancellationToken,
        state: Arc<AbaState<CT>>,
    ) {
        let id = config.id;
        // Stop the thread upon receiving a signal from the cancellation token
        tokio::select! {
            _ = cancel.cancelled() => {
                info!("Node `{id}` in ABA with sid `{sid}` stopping recv_thread");
            }

            _ = Self::recv_loop(config, receiver, sender, state).instrument(tracing::info_span!("recv_loop", ?sid)) => {}
        }
    }

    /// Infinite loop listening for ABA messages and sending notifications.
    async fn recv_loop(
        config: Arc<AbaCrain20Config<CT, CK>>,
        mut receiver: T::ReceiveMessageStream,
        sender: T::Sender,
        state: Arc<AbaState<CT>>,
    ) {
        // Local variables
        let mut count_est = PerPartyStorage::new();
        let mut sent_estimate: HashSet<EstimateMessage> = HashSet::new();

        loop {
            let ReceivedMessage {
                sender: sender_id,
                content,
                ..
            } = match receiver.next().await {
                Some(Ok(m)) => m,
                Some(Err(e)) => {
                    warn!("Node `{}` failed to recv: {e:?}", config.id);
                    continue;
                }
                None => {
                    error!(
                        "Node `{}` failed to recv: no more items in stream",
                        config.id
                    );
                    return;
                }
            };

            let m: AbaMessage = match bson::from_slice(&content) {
                Ok(m) => m,
                Err(e) => {
                    error!(error = ?e, "Node `{}` failed to deserialize message", config.id);
                    continue;
                }
            };
            trace!(
                "Node `{}` received message {m:?} from {sender_id}",
                config.id,
            );

            match m {
                // 4: upon receiving BVAL(v) do
                AbaMessage::Estimate(est) => {
                    count_est.insert_once(est, sender_id, true);
                    let count = count_est.get_count(&est);

                    #[allow(clippy::int_plus_one)]
                    if count >= 2 * config.t + 1 {
                        // 7: if BVAL(V) received from 2t + 1 different nodes then
                        // 8: bin_values := bin_values \cup {v}
                        // add the estimate to the binary values
                        let mut r_bin_values = state.bin_values.lock().await;
                        let bin_values = &mut r_bin_values.entry(est.round).or_default()[est.stage];
                        if bin_values.contains(&est.estimate) {
                            drop(r_bin_values);
                        } else {
                            bin_values.push(est.estimate);
                            drop(r_bin_values);

                            // notify of update to bin_values
                            debug!(
                                "Node {} notifying bin values for round {}",
                                config.id, est.round
                            );
                            state.notify_bin_values.notify_one((est.round, est.stage));
                        };
                    } else if count >= config.t + 1 && !sent_estimate.contains(&est) {
                        // 5: if BVAL(v) received from t + 1 different nodes AND BVAL(v) was not sent, then
                        // 6: Send BVAL(v) to all nodes
                        let msg_est = AbaMessage::Estimate(est);
                        if let Err(e) =
                            broadcast_with_self(&msg_est, &config.retry_strategy, &sender).await
                        {
                            error!(
                                "Node `{}` failed to broadcast estimate message: {e:?}",
                                config.id
                            )
                        }
                        sent_estimate.insert(est);
                    }
                }

                AbaMessage::Auxiliary(aux) => {
                    let mut aux_views = state.aux_views.lock().await;
                    // Add the new estimate to the current view
                    aux_views
                        .entry((aux.round, aux.stage), sender_id)
                        .or_default()
                        .insert(aux.estimate);

                    // notify once we got at least n - t aux messages
                    if aux_views.get_count(&(aux.round, aux.stage)) >= config.n - config.t {
                        state.notify_count_aux.notify_one((aux.round, aux.stage));
                    }
                }

                AbaMessage::AuxiliarySet(aux_set) => {
                    // Insert auxset view, at most once per sender_id
                    let mut auxset_views = state.auxset_views.lock().await;
                    auxset_views.insert_once(aux_set.round, sender_id, aux_set.view);

                    // notify once we got at least n - t auxset messages
                    if auxset_views.get_count(&aux_set.round) >= config.n - config.t {
                        state.notify_count_auxset.notify_one(aux_set.round);
                    }
                }

                AbaMessage::CoinEval(msg_eval) => {
                    // Deserialize eval
                    let Ok(eval) = msg_eval.deser::<CT::Eval>() else {
                        warn!("Failed to deserialize CoinEvalMessage");
                        continue;
                    };

                    // Store one eval per party, per round. We cannot verify it here
                    // since the node may not be ready to check evaluations yet.
                    let mut coin_evals = state.coin_evals.lock().await;
                    coin_evals.insert_once(msg_eval.round, sender_id, eval);
                    let count = coin_evals.get_count(&msg_eval.round);
                    drop(coin_evals); // drop lock

                    // Notify if we have t + 1 evals
                    if count > config.t {
                        state.notify_enough_coin_evals.notify_one(msg_eval.round);
                    }
                }
            };
        }
    }
}
