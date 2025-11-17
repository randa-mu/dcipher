use async_stream::stream;
use chrono::Utc;
use futures::Stream;
use std::cmp::{Ordering, Reverse, max};
use std::collections::BinaryHeap;
use std::ops::Add;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct RetryScheduler<Item> {
    // we use a min-heap so that the soonest retry will be on top
    to_retry: BinaryHeap<Reverse<Retry<Item>>>,
    retry_duration: Duration,
    rx: Receiver<Reverse<Retry<Item>>>,
    tx: Sender<Reverse<Retry<Item>>>,
}

pub struct Retry<Item> {
    earliest_time: i64,
    item: Item,
}

impl<Item: Eq> PartialOrd for Retry<Item> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Item: Eq> Ord for Retry<Item> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.earliest_time.cmp(&other.earliest_time)
    }
}

impl<Item: PartialEq> PartialEq for Retry<Item> {
    fn eq(&self, other: &Self) -> bool {
        self.earliest_time == other.earliest_time && self.item == other.item
    }
}

impl<Item: Eq> Eq for Retry<Item> {}

impl<Item> RetryScheduler<Item>
where
    Item: Eq,
{
    pub fn new(retry_duration: Duration) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(256);
        Self {
            to_retry: BinaryHeap::new(),
            retry_duration,
            tx,
            rx,
        }
    }
}

impl<Item> RetryScheduler<Item> {
    pub fn tx(&self) -> RetrySender<Item> {
        RetrySender {
            tx: self.tx.clone(),
            retry_duration: self.retry_duration,
        }
    }
}

impl<Item> RetryScheduler<Item>
where
    Item: Send + Eq + 'static,
{
    pub fn into_stream(mut self) -> impl Stream<Item = Item> + Send + 'static {
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
                        yield retry.0.item;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct RetrySender<Item> {
    tx: Sender<Reverse<Retry<Item>>>,
    retry_duration: Duration,
}

impl<Item> RetrySender<Item> {
    pub async fn send(&self, item: Item) -> anyhow::Result<()> {
        let earliest_time = Utc::now().add(self.retry_duration).timestamp();
        self.tx
            .clone()
            .send(Reverse(Retry {
                earliest_time,
                item,
            }))
            .await
            // custom error instead of wrapping the sent item into an error
            .map_err(|_| anyhow::anyhow!("retry channel closed"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::pin_mut;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn retry_scheduler_should_wait() {
        let retry_scheduler = RetryScheduler::new(Duration::from_hours(1));
        let sender = retry_scheduler.tx();
        let retry_stream = retry_scheduler.into_stream();
        pin_mut!(retry_stream);

        sender.send(()).await.expect("to send item successfully");
        tokio::time::timeout(Duration::from_millis(500), retry_stream.next())
            .await
            .expect_err("should timeout");
    }
}
