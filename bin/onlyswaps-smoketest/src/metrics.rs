//! Prometheus metrics for monitoring swap test execution and results.
//!
//! This module defines and exposes metrics that track the lifecycle of swap tests,
//! from initial request through fulfillment and verification.
//!
//! # Metrics Overview
//!
//! - **swap_requested**: Incremented when a swap request is successfully submitted
//! - **swap_fulfilled**: Incremented when a swap is fulfilled by the solver on the destination chain
//! - **swap_verified**: Incremented when a swap is verified on the source chain
//! - **swap_failed**: Incremented when a swap fails, with reason label for debugging

use prometheus::{IntCounterVec, Opts, Registry};
use std::sync::LazyLock;

pub struct Metrics {
    pub(super) registry: Registry,
    pub(super) swap_failed: IntCounterVec,
    pub(super) swap_requested: IntCounterVec,
    pub(super) swap_fulfilled: IntCounterVec,
    pub(super) swap_verified: IntCounterVec,
}

pub(super) static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
    let registry = Registry::new();

    let swap_failed = IntCounterVec::new(
        Opts::new(
            "swap_failed",
            "Total number of swap that have failed per label and reason",
        ),
        &["label", "reason"],
    )
    .expect("failed to create IntCounterVec");

    let swap_requested = IntCounterVec::new(
        Opts::new(
            "swap_requested",
            "Total number of swaps requested per label",
        ),
        &["label"],
    )
    .expect("failed to create IntCounterVec");

    let swap_fulfilled = IntCounterVec::new(
        Opts::new(
            "swap_fulfilled",
            "Total number of swaps fulfilled per label",
        ),
        &["label"],
    )
    .expect("failed to create IntCounterVec");

    let swap_verified = IntCounterVec::new(
        Opts::new("swap_verified", "Total number of swaps verified per label"),
        &["label"],
    )
    .expect("failed to create IntCounterVec");

    registry
        .register(Box::new(swap_failed.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(swap_requested.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(swap_fulfilled.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(swap_verified.clone()))
        .expect("metrics failed to initialise");

    Metrics {
        registry,
        swap_failed,
        swap_requested,
        swap_fulfilled,
        swap_verified,
    }
});

impl Metrics {
    pub(super) fn report_swap_failed(label: String, reason: String) {
        METRICS
            .swap_failed
            .with_label_values(&[label, reason])
            .inc();
    }

    pub(super) fn report_swap_requested(label: String) {
        METRICS.swap_requested.with_label_values(&[label]).inc();
    }

    pub(super) fn report_swap_fulfilled(label: String) {
        METRICS.swap_fulfilled.with_label_values(&[label]).inc();
    }

    pub(super) fn report_swap_verified(label: String) {
        METRICS.swap_verified.with_label_values(&[label]).inc();
    }

    pub fn gather() -> Vec<prometheus::proto::MetricFamily> {
        METRICS.registry.gather()
    }
}
