use alloy::primitives::BlockNumber;
use alloy::providers::Provider;
use alloy::transports::TransportResult;
use futures_util::Stream;
use futures_util::StreamExt;

pub async fn block_poller<P>(
    provider: P,
    interval: std::time::Duration,
) -> TransportResult<impl Stream<Item = BlockNumber> + Unpin>
where
    P: Provider,
{
    let mut block_watcher = provider.watch_full_blocks().await?;
    block_watcher.set_poll_interval(interval);

    let block_stream = block_watcher
        .into_stream()
        .filter_map(|res| std::future::ready(res.ok().map(|b| b.header.number)));

    Ok(block_stream)
}
