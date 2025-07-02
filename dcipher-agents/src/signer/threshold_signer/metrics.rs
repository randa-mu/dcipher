use prometheus::proto::MetricFamily;
use prometheus::{IntCounter, IntGauge, Registry};
use std::sync::LazyLock;

pub struct Metrics {
    registry: Registry,
    connected_peers: IntGauge,
    partials_sent: IntCounter,
    partials_received: IntCounter,
    invalid_partials: IntCounter,
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

    Metrics {
        registry,
        connected_peers,
        partials_received,
        partials_sent,
        invalid_partials,
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

    pub fn gather() -> Vec<MetricFamily> {
        METRICS.registry.gather()
    }
}
