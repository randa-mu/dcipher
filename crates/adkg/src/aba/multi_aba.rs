//! Module that allows multiplexing multiple ABAs through a single channel.

use crate::aba::Aba;
use crate::aba::{AbaConfig, Estimate};
use crate::helpers::{PartyId, SessionId};
use crate::rand::{AdkgRng, AdkgRngType};
use dcipher_network::topic::TopicBasedTransport;
use futures::future::join_all;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, StreamExt};
use itertools::{Either, izip};
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::task::{JoinError, JoinHandle};
use tokio_util::sync::CancellationToken;

pub struct MultiAba<ABAConfig>
where
    ABAConfig: AbaConfig<'static, PartyId>,
{
    #[allow(unused)]
    node_id: PartyId,
    n_instances: usize,
    aba_config: Arc<ABAConfig>,

    #[allow(clippy::type_complexity)]
    tasks: Vec<Option<JoinHandle<Result<(), ABAConfig::Error>>>>,
    send_proposals: Vec<Option<oneshot::Sender<ABAConfig::Input>>>,
    recv_outputs: Vec<Option<oneshot::Receiver<Estimate>>>,

    cancels: Vec<CancellationToken>,
}

impl<ABAConfig> MultiAba<ABAConfig>
where
    ABAConfig: AbaConfig<'static, PartyId>,
{
    pub fn new(node_id: PartyId, n_instances: usize, aba_config: Arc<ABAConfig>) -> Self {
        let cancels = (0..n_instances).map(|_| CancellationToken::new()).collect();

        Self {
            node_id,
            n_instances,
            aba_config,

            tasks: vec![],
            send_proposals: vec![],
            recv_outputs: vec![],
            cancels,
        }
    }

    /// Start n parallel ABA instances in the background.
    pub fn start<T>(&mut self, rng: &mut impl AdkgRng, transport: T)
    where
        T: TopicBasedTransport<Identity = PartyId>,
    {
        // Initializes the various channels
        let mut recv_proposals = vec![];
        let mut send_outputs = vec![];
        for _ in 1..=self.n_instances {
            let (estimate_send, estimate_recv) = oneshot::channel();
            let (output_send, output_recv) = oneshot::channel();

            self.send_proposals.push(Some(estimate_send));
            recv_proposals.push(estimate_recv);
            self.recv_outputs.push(Some(output_recv));
            send_outputs.push(output_send);
        }

        for (sid, estimate_recv, output_send, cancel) in izip!(
            SessionId::iter_all(self.n_instances),
            recv_proposals,
            send_outputs,
            self.cancels.iter().cloned(),
        ) {
            let aba = self
                .aba_config
                .new_instance_with_prefix(sid, sid.to_string(), transport.clone())
                .expect("failed to create aba instance");

            self.tasks.push(Some(tokio::task::spawn({
                let mut rng = rng.get(AdkgRngType::Aba(sid)).unwrap();
                async move {
                    aba.propose(estimate_recv, output_send, cancel, &mut rng)
                        .await
                }
            })));
        }
    }

    /// Returns the oneshot sender that can be used to set input of ABA with session id `sid`.
    pub fn get_sender(&mut self, sid: &SessionId) -> Option<oneshot::Sender<ABAConfig::Input>> {
        self.send_proposals[sid].take()
    }

    /// Returns an iterator over the remaining ABA senders and their sids.
    pub fn iter_remaining_senders(
        &mut self,
    ) -> impl Iterator<Item = (SessionId, oneshot::Sender<ABAConfig::Input>)> + use<ABAConfig> {
        let v: Vec<_> = self
            .send_proposals
            .iter_mut()
            .enumerate()
            .filter_map(move |(j, s)| s.take().map(|sender| (SessionId::from_index(j), sender)))
            .collect(); // collect to remove the bound to the lifetime of &self due to the opaque iterator type

        v.into_iter()
    }

    /// Returns a StreamExt used to receive the remaining estimates and their sids.
    pub fn iter_remaining_estimates(
        &mut self,
    ) -> impl StreamExt<Item = (SessionId, Result<Estimate, oneshot::error::RecvError>)> + use<ABAConfig>
    {
        let estimates_fut = self
            .recv_outputs
            .iter_mut()
            .zip(SessionId::iter_all(self.n_instances))
            .filter_map(|(v, sid)| v.take().map(|r| r.map(move |out| (sid, out))));

        FuturesUnordered::from_iter(estimates_fut)
    }

    /// Returns an iterator over the remaining ABA senders and their sids.
    pub async fn cancel(
        &mut self,
        i: SessionId,
    ) -> Option<Result<(), Either<JoinError, ABAConfig::Error>>> {
        let task = self.tasks[i].take()?;
        self.cancels[i].cancel();

        let res = match task.await {
            Ok(Ok(_)) => Ok(()),
            Err(join_error) => Err(Either::Left(join_error)),
            Ok(Err(aba_error)) => Err(Either::Right(aba_error)),
        };

        Some(res)
    }

    /// Stops each of the ABA instances, and return Ok(()) if no errors were output, otherwise,
    /// return the identifier of failed instances and their errors.
    pub async fn stop(self) -> Result<(), Vec<(PartyId, Either<JoinError, ABAConfig::Error>)>> {
        // Cancel each of the tasks
        self.cancels.iter().for_each(|c| c.cancel());

        let outputs = join_all(
            PartyId::iter_all(self.n_instances)
                .zip(self.tasks.into_iter())
                .filter_map(|(id, task)| task.map(|t| t.map(move |out| (id, out)))),
        )
        .await;

        let errors: Vec<_> = outputs
            .into_iter()
            .filter_map(|(sid, out)| {
                let err = match out {
                    Ok(Ok(_)) => None?,
                    Err(join_error) => Some(Either::Left(join_error)),
                    Ok(Err(aba_error)) => Some(Either::Right(aba_error)),
                };

                err.map(|e| (sid, e))
            })
            .collect();

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}
