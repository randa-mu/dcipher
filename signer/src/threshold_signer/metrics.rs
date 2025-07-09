use prometheus::proto::MetricFamily;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntGauge, Registry};
use std::sync::LazyLock;

pub struct Metrics {
    registry: Registry,
    connected_peers: IntGauge,
    partials_sent: IntCounter,
    partials_received: IntCounter,
    invalid_partials: IntCounter,
    rtt_histogram: HistogramVec,
}

static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
    let registry = Registry::new();

    let connected_peers = IntGauge::new("libp2p_connected_peers", "Number of connected peers")
        .expect("metrics failed to initialise");

    let partials_sent = IntCounter::new(
        "partial_signatures_sent",
        "Number of partial signatures sent",
    )
    .expect("metrics failed to initialise");

    let partials_received = IntCounter::new(
        "partial_signatures_received",
        "Number of partial signatures received",
    )
    .expect("metrics failed to initialise");

    let invalid_partials = IntCounter::new(
        "invalid_partial_signature_received",
        "Number of invalid partial signatures received",
    )
    .expect("metrics failed to initialise");

    let rtt_histogram = HistogramVec::new(
        HistogramOpts::new(
            "libp2p_hosts_rtt_seconds",
            "Round-trip time to connected hosts in seconds",
        ),
        &["host_short_id"],
    )
    .expect("metrics failed to initialise");

    registry
        .register(Box::new(connected_peers.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(partials_sent.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(partials_received.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(invalid_partials.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(rtt_histogram.clone()))
        .expect("metrics failed to initialise");

    Metrics {
        registry,
        connected_peers,
        partials_received,
        partials_sent,
        invalid_partials,
        rtt_histogram,
    }
});

impl Metrics {
    pub(super) fn report_peer_connected() {
        METRICS.connected_peers.inc();
    }

    pub(super) fn report_peer_disconnected() {
        METRICS.connected_peers.dec();
    }

    pub(super) fn report_partials_received(count: u64) {
        METRICS.partials_received.inc_by(count)
    }

    pub(super) fn report_partials_sent(count: u64) {
        METRICS.partials_sent.inc_by(count)
    }

    pub(super) fn report_invalid_partials(count: u64) {
        METRICS.invalid_partials.inc_by(count)
    }

    pub(super) fn report_host_rtt(rtt_seconds: f64, host: impl AsRef<str> + std::fmt::Debug) {
        METRICS
            .rtt_histogram
            .with_label_values(&[host])
            .observe(rtt_seconds)
    }

    pub fn gather() -> Vec<MetricFamily> {
        METRICS.registry.gather()
    }
}
