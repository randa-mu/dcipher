use alloy::primitives::U64;
use alloy::providers::Provider;
use alloy::rpc::client::{NoParams, RpcCall};
use futures_util::{FutureExt, Stream};
use std::pin::Pin;
use std::task::{Context, Poll};

pub async fn block_poller<P>(provider: P, interval: std::time::Duration) -> BlockPoller
where
    P: Provider,
{
    BlockPoller::new(provider, interval)
}

pub struct BlockPoller {
    /// The ticker
    ticker: tokio::time::Interval,

    /// The cloneable rpc call used to fetch the latest block
    block_number_ref_call: RpcCall<NoParams, U64>,

    /// A pending block number call
    block_number_pending_call: Option<RpcCall<NoParams, U64>>,
}

impl BlockPoller {
    fn new(provider: impl Provider, poll_interval: std::time::Duration) -> Self {
        let block_number_call: RpcCall<NoParams, U64> =
            provider.client().request_noparams("eth_blockNumber");

        let mut ticker = tokio::time::interval(poll_interval);
        // Reset ticker to ensure that it waits on the first poll since we set the fut block_number_fut
        // immediately
        ticker.reset();

        Self {
            ticker,
            block_number_pending_call: Some(block_number_call.clone()),
            block_number_ref_call: block_number_call,
        }
    }
}

impl Stream for BlockPoller {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(block_number_fut) = &mut self.block_number_pending_call {
            // Pending block number call, poll that
            let Poll::Ready(maybe_block_number) = block_number_fut.poll_unpin(cx) else {
                return Poll::Pending;
            };

            // Future completed
            self.block_number_pending_call = None;

            // Ready to poll again, we might need to get the next block immediately
            cx.waker().wake_by_ref();

            match maybe_block_number {
                Ok(block_number) => {
                    Poll::Ready(Some(block_number.try_into().expect("U64 to fit in u64")))
                }
                Err(e) => {
                    tracing::error!(error = ?e, "Poller failed to get latest block number");
                    Poll::Pending
                }
            }
        } else {
            // No pending call, poll ticker
            if self.ticker.poll_tick(cx).is_ready() {
                self.block_number_pending_call = Some(self.block_number_ref_call.clone());

                // Ready to poll the rpc call
                cx.waker().wake_by_ref();
            }
            Poll::Pending
        }
    }
}
