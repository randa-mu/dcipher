use crate::config::SwapTest;
use crate::metrics::Metrics;
use alloy::primitives::Address;
use anyhow::Context;
use chrono::{DateTime, Utc};
use onlyswaps_client::FeeEstimator;
use onlyswaps_client::client::routing::SwapRouting;
use onlyswaps_client::client::{OnlySwapsClient, OnlySwapsReceipt, OnlySwapsRequestBuilder};
use std::sync::Arc;
use std::time::SystemTime;
use tracing::Instrument;

#[tracing::instrument(
    level = "info",
    skip_all,
    fields(
        label = swap.label,
        interval = %humantime::Duration::from(swap.interval),
        timeout = %humantime::Duration::from(swap.timeout),
        src_chain = swap.src_chain_id,
        dst_chain = swap.dst_chain_id,
        src_token = ?swap.src_token,
        dst_token = ?swap.dst_token,
    )
)]
pub fn smoketest_loop(
    self_recipient: Address,
    client: Arc<OnlySwapsClient>,
    swap: SwapTest,
) -> anyhow::Result<impl Future<Output = ()>> {
    let client = client.clone();
    let fee_estimator = FeeEstimator::default();
    let mut interval = tokio::time::interval(swap.interval);

    // Static configuration details
    let src_token_addr = client
        .config()
        .get_token_address(swap.src_chain_id, &swap.src_token)
        .with_context(|| {
            format!(
                "source token {:?} is not supported by chain {}",
                swap.src_token, swap.src_chain_id
            )
        })?;
    let dst_token_addr = client
        .config()
        .get_token_address(swap.dst_chain_id, &swap.dst_token)
        .with_context(|| {
            format!(
                "destination token {:?} is not supported by chain {}",
                swap.dst_token, swap.dst_chain_id
            )
        })?;
    let routing = SwapRouting {
        src_token: src_token_addr,
        dst_token: dst_token_addr,
        src_chain: swap.src_chain_id,
        dst_chain: swap.dst_chain_id,
    };
    let req_builder = OnlySwapsRequestBuilder::default()
        .recipient(swap.recipient.unwrap_or(self_recipient))
        .route(routing);

    Ok(async move {
        loop {
            let now: DateTime<Utc> = SystemTime::now().into();
            let span = tracing::info_span!("swap_request", timestamp = %now);
            do_swap(&client, &fee_estimator, &swap, req_builder.clone())
                .instrument(span)
                .await;

            interval.tick().await;
        }
    })
}

/// Fetch the latest solver / network fee, and execute a swap
async fn do_swap(
    client: &Arc<OnlySwapsClient>,
    fee_estimator: &FeeEstimator,
    swap: &SwapTest,
    req_builder: OnlySwapsRequestBuilder,
) {
    // Fetch the latest fees from the api
    let req_with_fees = match req_builder
        .clone()
        .exact_amount(swap.amount, fee_estimator)
        .await
    {
        Ok(req_with_fees) => req_with_fees,
        Err(e) => {
            tracing::error!(error = ?e, "failed to fetch fees");
            return;
        }
    };

    let req = req_with_fees.build().expect("valid arguments"); // only fallible if misused

    // Attempt to send a new swap request
    let swap_res = client.approve_and_swap(req).await;
    let receipt = match swap_res {
        Ok(receipt) => receipt,
        Err(e) => {
            tracing::error!(error = ?e, "failed to request swap through client");
            return;
        }
    };

    // We have successfully submitted a swap
    let span = tracing::warn_span!("swap_submitted", request_id = %receipt.request_id, tx_hash = %receipt.tx_hash).entered();
    tracing::info!("Swap request submitted");
    Metrics::report_swap_requested(swap.label.clone());

    // Monitor swap
    monitor_swap(client, swap, &receipt)
        .instrument(span.exit()) // exit the EnteredSpan to get a span that can be used to instrument fut
        .await
}

/// Monitor a swap receipt and log stages
async fn monitor_swap(client: &Arc<OnlySwapsClient>, swap: &SwapTest, receipt: &OnlySwapsReceipt) {
    // Checks that the swap is fulfilled & verified within the timeout
    let timeout_res = tokio::time::timeout(swap.timeout, async {
        // First, wait for completion
        tracing::debug!("Waiting for swap completion");
        match client.wait_until_complete(receipt).await {
            Ok(()) => {
                tracing::info!("Swap request fulfilled");
                Metrics::report_swap_fulfilled(swap.label.clone());
            }
            Err(e) => {
                tracing::error!(error = ?e, "Failed to wait until swap is fulfilled");
                return;
            }
        }

        // Then, for verification
        tracing::debug!("Waiting for swap verification");
        match client.wait_until_verified(receipt).await {
            Ok(()) => {
                tracing::info!("Swap request verified");
                Metrics::report_swap_verified(swap.label.clone());
            }
            Err(e) => {
                tracing::error!(error = ?e, "Failed to wait until swap is verified");
            }
        }
    })
    .await;

    if timeout_res.is_err() {
        tracing::error!("Swap was not fulfilled within timeout");
        Metrics::report_swap_failed(swap.label.clone(), "timeout".to_owned());
    }
}
