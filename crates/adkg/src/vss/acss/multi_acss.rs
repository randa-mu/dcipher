//! Module that allows managing multiple parallel ACSS instances.

use crate::helpers::{PartyId, SessionId};
use crate::rand::{AdkgRng, AdkgRngType};
use crate::vss::acss::{Acss, AcssConfig};
use ark_ec::CurveGroup;
use dcipher_network::topic::TopicBasedTransport;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, StreamExt};
use itertools::izip;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;

pub struct MultiAcss<CG, ACSSConfig>
where
    CG: CurveGroup,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
{
    node_id: PartyId,
    n_instances: usize,
    acss_config: Arc<ACSSConfig>,

    // Attributes used to manage the subtasks
    acss_tasks: JoinSet<(SessionId, Result<(), MultiAcssError>)>, // set of acss tasks
    acss_receivers: Vec<Option<oneshot::Receiver<ACSSConfig::Output>>>,
    acss_leader_sender: Option<oneshot::Sender<ACSSConfig::Input>>, // set the leader input
    cancels: Vec<CancellationToken>,
}

#[derive(thiserror::Error, Debug)]
pub enum MultiAcssError {
    #[error(transparent)]
    Acss(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("failed to get ACSS input from channel: sender dropped")]
    AcssInputDropped,
}

impl<CG, ACSSConfig> MultiAcss<CG, ACSSConfig>
where
    CG: CurveGroup,
    ACSSConfig: AcssConfig<'static, CG, PartyId>,
{
    pub fn new(node_id: PartyId, n_instances: usize, acss_config: Arc<ACSSConfig>) -> Self {
        let cancels = (0..n_instances).map(|_| CancellationToken::new()).collect();
        Self {
            node_id,
            n_instances,
            acss_config,
            acss_tasks: JoinSet::new(),
            acss_receivers: vec![],
            acss_leader_sender: None,
            cancels,
        }
    }

    /// Start the n parallel ACSS instances in the background.
    /// Returns a channel used to transmit the ACSS secret.
    pub fn start<T>(&mut self, rng: &mut impl AdkgRng, transport: T)
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        // First, we create n oneshot channel to receive / send the Acss output.
        let (senders, receivers): (Vec<_>, Vec<_>) = (1..=self.n_instances)
            .map(|_| oneshot::channel())
            .map(|(sender, receiver)| (sender, Some(receiver)))
            .collect();
        self.acss_receivers = receivers;

        // Create one channel for the ACSS input
        let (input_tx, input_rx) = oneshot::channel();
        self.acss_leader_sender = Some(input_tx);
        let mut input_rx = Some(input_rx); // need an option for interior mutability...

        for (sid, cancel, sender) in izip!(
            SessionId::iter_all(self.n_instances),
            self.cancels.iter().cloned(),
            senders
        ) {
            let acss = self
                .acss_config
                .new_instance_with_prefix(sid.to_string(), transport.clone()) // topic is e.g. 1/hbacss0
                .expect("failed to initialize ACSS instance");

            // Spawn a new task for the acss
            self.acss_tasks.spawn({
                let node_id = self.node_id;
                let cancellation_token = cancel.clone();

                // s is not cloneable, and we only want to move it when sid == node_id
                // In order to not move s due to the async move below, we take() s only once
                // here, and use None when sid != node_id. This allows to move the value only once.
                let mut input_rx = if sid == node_id {
                    input_rx.take()
                } else {
                    None
                };

                let mut rng = rng
                    .get(AdkgRngType::Acss(sid))
                    .expect("failed to obtain acss rng");
                async move {
                    // Start the acss tasks
                    let res = if sid == node_id {
                        if let Ok(s) = input_rx.take().expect("to enter once").await {
                            acss.deal(s, cancellation_token, sender, &mut rng)
                                .instrument(tracing::warn_span!("ACSS::deal", ?sid))
                                .await
                                .map_err(|e| MultiAcssError::Acss(e.into()))
                        } else {
                            Err(MultiAcssError::AcssInputDropped)
                        }
                    } else {
                        acss.get_share(sid.into(), cancellation_token, sender, &mut rng)
                            .instrument(tracing::warn_span!("ACSS::get_share", ?sid))
                            .await
                            .map_err(|e| MultiAcssError::Acss(e.into()))
                    };

                    (sid, res)
                }
            });
        }
    }

    /// Get the oneshot sender used to set the leader output of the ACSS where self.node_id == sid
    pub fn get_leader_sender(&mut self) -> Option<oneshot::Sender<ACSSConfig::Input>> {
        self.acss_leader_sender.take()
    }

    /// Create an iterator over the remaining ACSS outputs.
    pub fn iter_remaining_outputs(
        &mut self,
    ) -> impl StreamExt<
        Item = (
            SessionId,
            Result<ACSSConfig::Output, oneshot::error::RecvError>,
        ),
    > + use<CG, ACSSConfig> {
        let outputs_fut = self
            .acss_receivers
            .iter_mut()
            .zip(SessionId::iter_all(self.n_instances))
            .filter_map(|(v, sid)| v.take().map(|r| r.map(move |out| (sid, out))));

        FuturesUnordered::from_iter(outputs_fut)
    }

    /// Stop the ACSS instances and return Ok(()) if no errors were output, otherwise, return the identifier of failed instances and their errors.
    pub async fn stop(self) -> Result<(), Vec<(SessionId, MultiAcssError)>> {
        // Signal cancellation through each of the cancellation tokens
        self.cancels.iter().for_each(|cancel| cancel.cancel());

        let errors: Vec<(SessionId, MultiAcssError)> = self
            .acss_tasks
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
