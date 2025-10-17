#[cfg(feature = "metrics")]
mod real_metrics {
    use prometheus::{IntCounterVec, Opts, Registry};
    use std::sync::LazyLock;

    pub struct Metrics {
        pub(super) registry: Registry,
        pub(super) swap_requested: IntCounterVec,
        pub(super) swap_fee_updated: IntCounterVec,
        pub(super) swap_fulfilled: IntCounterVec,
        pub(super) swap_verified: IntCounterVec,
    }

    pub(super) static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
        let registry = Registry::new();

        let swap_requested = IntCounterVec::new(
            Opts::new("swap_requested", "Total number of swaps requested per (source chain, destination chain, source token, dest token) tuple"),
            &["src_chain_id", "dst_chain_id", "src_token", "dst_token"],
        ).expect("failed to create IntCounterVec");

        let swap_fee_updated = IntCounterVec::new(
            Opts::new("swap_fee_updated", "Total number of swap fees updated per (source chain, destination chain, source token, dest token) tuple"),
            &["src_chain_id", "dst_chain_id", "src_token", "dst_token"],
        ).expect("failed to create IntCounterVec");

        let swap_fulfilled = IntCounterVec::new(
            Opts::new("swap_fulfilled", "Total number of swaps fulfilled per (source chain, destination chain, source token, dest token) tuple"),
            &["src_chain_id", "dst_chain_id", "src_token", "dst_token"],
        ).expect("failed to create IntCounterVec");

        let swap_verified = IntCounterVec::new(
            Opts::new("swap_verified", "Total number of swaps verified per (source chain, destination chain, source token, dest token) tuple"),
            &["src_chain_id", "dst_chain_id", "src_token", "dst_token"],
        ).expect("failed to create IntCounterVec");

        registry
            .register(Box::new(swap_requested.clone()))
            .expect("metrics failed to initialise");
        registry
            .register(Box::new(swap_fee_updated.clone()))
            .expect("metrics failed to initialise");
        registry
            .register(Box::new(swap_fulfilled.clone()))
            .expect("metrics failed to initialise");
        registry
            .register(Box::new(swap_verified.clone()))
            .expect("metrics failed to initialise");

        Metrics {
            registry,
            swap_requested,
            swap_fee_updated,
            swap_fulfilled,
            swap_verified,
        }
    });
}

#[cfg(feature = "metrics")]
pub use real_metrics::Metrics;

#[cfg(not(feature = "metrics"))]
pub struct Metrics;

#[allow(unused)]
impl Metrics {
    pub(super) fn report_swap_requested(
        src_chain_id: alloy::primitives::U256,
        dst_chain_id: alloy::primitives::U256,
        token: alloy::primitives::Address,
        dst_token: alloy::primitives::Address,
    ) {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS
            .swap_requested
            .with_label_values(&[
                src_chain_id.to_string(),
                dst_chain_id.to_string(),
                token.to_string(),
                dst_token.to_string(),
            ])
            .inc();
    }

    pub(super) fn report_fee_updated(
        src_chain_id: alloy::primitives::U256,
        dst_chain_id: alloy::primitives::U256,
        token: alloy::primitives::Address,
        dst_token: alloy::primitives::Address,
    ) {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS
            .swap_fee_updated
            .with_label_values(&[
                src_chain_id.to_string(),
                dst_chain_id.to_string(),
                token.to_string(),
                dst_token.to_string(),
            ])
            .inc();
    }

    pub(super) fn report_swap_fulfilled(
        src_chain_id: alloy::primitives::U256,
        dst_chain_id: alloy::primitives::U256,
        token: alloy::primitives::Address,
        dst_token: alloy::primitives::Address,
    ) {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS
            .swap_fulfilled
            .with_label_values(&[
                src_chain_id.to_string(),
                dst_chain_id.to_string(),
                token.to_string(),
                dst_token.to_string(),
            ])
            .inc();
    }

    pub(super) fn report_swap_verified(
        src_chain_id: alloy::primitives::U256,
        dst_chain_id: alloy::primitives::U256,
        token: alloy::primitives::Address,
        dst_token: alloy::primitives::Address,
    ) {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS
            .swap_verified
            .with_label_values(&[
                src_chain_id.to_string(),
                dst_chain_id.to_string(),
                token.to_string(),
                dst_token.to_string(),
            ])
            .inc();
    }

    #[cfg(feature = "metrics")]
    pub fn gather() -> Vec<prometheus::proto::MetricFamily> {
        real_metrics::METRICS.registry.gather()
    }
}
