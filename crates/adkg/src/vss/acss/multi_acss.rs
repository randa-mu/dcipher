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
    acss_tasks: JoinSet<(SessionId, Result<(), ACSSConfig::Error>)>, // set of acss tasks
    acss_receivers: Vec<Option<oneshot::Receiver<ACSSConfig::Output>>>,
    cancels: Vec<CancellationToken>,
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
            cancels,
        }
    }

    /// Start the n parallel ACSS instances in the background.
    pub fn start<T>(&mut self, s: ACSSConfig::Input, rng: &mut impl AdkgRng, transport: T)
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        // First, we create n oneshot channel to receive / send the Acss output.
        let (senders, receivers): (Vec<_>, Vec<_>) = (1..=self.n_instances)
            .map(|_| oneshot::channel())
            .map(|(sender, receiver)| (sender, Some(receiver)))
            .collect();
        self.acss_receivers = receivers;
        let mut s = Some(s); // need an option for interior mutability...

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
                let s = if sid == node_id { s.take() } else { None };
                let mut rng = rng
                    .get(AdkgRngType::Acss(sid))
                    .expect("failed to obtain acss rng");
                async move {
                    // Start the acss tasks
                    let res = if sid == node_id {
                        acss.deal(
                            s.expect("can only enter once"),
                            cancellation_token,
                            sender,
                            &mut rng,
                        )
                        .instrument(tracing::info_span!("ACSS::deal", ?sid))
                        .await
                    } else {
                        acss.get_share(sid.into(), cancellation_token, sender, &mut rng)
                            .instrument(tracing::info_span!("ACSS::get_share", ?sid))
                            .await
                    };

                    (sid, res)
                }
            });
        }
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
    pub async fn stop(self) -> Result<(), Vec<(SessionId, ACSSConfig::Error)>> {
        // Signal cancellation through each of the cancellation tokens
        self.cancels.iter().for_each(|cancel| cancel.cancel());

        let errors: Vec<(SessionId, ACSSConfig::Error)> = self
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
