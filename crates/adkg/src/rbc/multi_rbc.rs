//! Module to manage multiple parallel RBC for the ADKG.

use crate::helpers::{PartyId, SessionId};
use crate::rbc::{RbcPredicate, ReliableBroadcast, ReliableBroadcastConfig};
use dcipher_network::topic::TopicBasedTransport;
use itertools::izip;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::task::{JoinError, JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

pub struct MultiRbc<RBCConfig>
where
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
{
    node_id: PartyId,
    n_instances: usize,
    rbc_config: Arc<RBCConfig>,

    #[allow(clippy::type_complexity)]
    tasks: JoinSet<(SessionId, Result<Vec<u8>, RBCConfig::Error>)>,
    leader_input: Option<oneshot::Sender<Vec<u8>>>,
    cancels: Vec<CancellationToken>,
}

impl<RBCConfig> MultiRbc<RBCConfig>
where
    RBCConfig: ReliableBroadcastConfig<'static, PartyId>,
{
    pub fn new(node_id: PartyId, n_instances: usize, rbc_config: Arc<RBCConfig>) -> Self {
        let cancels = (0..n_instances).map(|_| CancellationToken::new()).collect();

        Self {
            node_id,
            n_instances,
            rbc_config,
            tasks: JoinSet::new(),
            leader_input: None,
            cancels,
        }
    }

    /// Start n parallel RBC instances in the background.
    pub fn start<T, I, P>(&mut self, predicates: I, transport: T)
    where
        T: TopicBasedTransport<Identity = PartyId>,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator, // for into_iter().len()
        P: RbcPredicate + 'static,
    {
        // Oneshot channel used to set the leader's output (i.e., the rbc where sid == Node ID)
        let (leader_input_tx, leader_input_rx) = oneshot::channel();
        self.leader_input = Some(leader_input_tx);
        let mut rx_own_rbc = Some(leader_input_rx);

        for (sid, cancel, predicate) in izip!(
            SessionId::iter_all(self.n_instances),
            self.cancels.iter().cloned(),
            predicates.into_iter()
        ) {
            self.tasks.spawn({
                let node_id = self.node_id;
                let rbc = self
                    .rbc_config
                    .new_instance_with_prefix(sid.to_string(), transport.clone())
                    .expect("failed to initialize RBC instance");
                let rx_own_rbc = if sid == node_id {
                    rx_own_rbc.take()
                } else {
                    None
                };

                async move {
                    // Start everything but our own rbc
                    if sid == node_id {
                        let Some(rx_own_rbc) = rx_own_rbc else {
                            unreachable!("cannot have more than one sid matching node_id");
                        };
                        let m: Vec<u8> = rx_own_rbc.await.unwrap();
                        (
                            sid,
                            rbc.start(&m, cancel)
                                .instrument(tracing::warn_span!("RBC::start", ?sid))
                                .await,
                        )
                    } else {
                        (
                            sid,
                            rbc.listen(&predicate, sid.into(), cancel)
                                .instrument(tracing::warn_span!("RBC::listen", ?sid))
                                .await,
                        )
                    }
                }
            });
        }
    }

    /// Get the oneshot sender used to set the leader output of the RBC where self.node_id == sid
    pub fn get_leader_sender(&mut self) -> Option<oneshot::Sender<Vec<u8>>> {
        self.leader_input.take()
    }

    /// Join the next available RBC task to get its output.
    pub async fn join_next(
        &mut self,
    ) -> Option<Result<(SessionId, Result<Vec<u8>, RBCConfig::Error>), JoinError>> {
        self.tasks.join_next().await
    }

    /// Stops each of the RBC instances, and return Ok(()) if no errors were output, otherwise,
    /// return the identifier of failed instances and their errors.
    pub async fn stop(self) -> Result<(), Vec<(SessionId, RBCConfig::Error)>> {
        self.cancels.iter().for_each(|c| c.cancel());
        let errors: Vec<(SessionId, RBCConfig::Error)> = self
            .tasks
            .join_all()
            .await
            .into_iter()
            .filter_map(|(sid, res)| res.err().map(|e| (sid, e)))
            .collect();

        if !errors.is_empty() {
            Err(errors)?
        } else {
            Ok(())
        }
    }
}
