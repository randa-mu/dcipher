use crate::chain_state_pending::Verification;
use alloy::primitives::FixedBytes;
use async_stream::stream;
use chrono::Utc;
use config::network::NetworkConfig;
use futures::Stream;
use std::cmp::{Ordering, Reverse, max};
use std::collections::{BinaryHeap, HashMap};
use std::ops::Add;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct RetryScheduler {
    // we use a min-heap so that the soonest retry will be on top
    to_retry: BinaryHeap<Reverse<Retry>>,
    finality_durations: HashMap<u64, Duration>,
    rx: Receiver<Reverse<Retry>>,
    tx: Sender<Reverse<Retry>>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Retry {
    earliest_time: i64,
    verification: Verification<FixedBytes<32>>,
}

impl PartialOrd for Retry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Retry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.earliest_time.cmp(&other.earliest_time)
    }
}

impl RetryScheduler {
    pub fn new(networks: &[NetworkConfig]) -> Self {
        let finality_durations: HashMap<u64, Duration> = networks
            .iter()
            .map(|it| (it.chain_id, it.finality_duration_secs))
            .collect();
        let (tx, rx) = tokio::sync::mpsc::channel(256);
        Self {
            to_retry: BinaryHeap::new(),
            finality_durations,
            tx,
            rx,
        }
    }

    pub fn tx(&self) -> Enqueuer {
        Enqueuer {
            tx: self.tx.clone(),
            finality_durations: self.finality_durations.clone(),
        }
    }

    pub fn stream(&mut self) -> impl Stream<Item = Verification<FixedBytes<32>>> {
        stream! {
            loop {
                let duration_until_retry = self.to_retry.peek()
                    .map(|it| it.0.earliest_time)
                    .map(|secs| max(0, Utc::now().timestamp() - secs))
                    .map(|secs| Duration::from_secs(secs as u64))
                    .unwrap_or(Duration::MAX);

                select! {
                    task = self.rx.recv() => {
                        if let Some(t) = task {
                           self.to_retry.push(t);
                        }
                    },
                    _ = tokio::time::sleep(duration_until_retry) => {
                        let retry = self.to_retry.pop()
                            .expect("we checked this in the preconditions, unless we've reached Duration::MAX in which case we should fear the heat death of the universe more than a panic");
                        yield retry.0.verification;
                    }
                }
            }
        }
    }
}

pub struct Enqueuer {
    tx: Sender<Reverse<Retry>>,
    finality_durations: HashMap<u64, Duration>,
}
impl Enqueuer {
    pub async fn send(&self, verification: Verification<FixedBytes<32>>) -> anyhow::Result<()> {
        let wait_duration = self
            .finality_durations
            .get(&verification.chain_id)
            .expect("it shouldn't be possible to get an invalid chain here");
        let earliest_time = Utc::now().add(*wait_duration).timestamp();
        self.tx
            .clone()
            .send(Reverse(Retry {
                earliest_time,
                verification,
            }))
            .await?;
        Ok(())
    }
}
