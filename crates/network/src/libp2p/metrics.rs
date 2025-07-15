use prometheus::proto::MetricFamily;
use prometheus::{HistogramOpts, HistogramVec, IntGauge, Registry};
use std::sync::LazyLock;

pub struct Metrics {
    registry: Registry,
    connected_peers: IntGauge,
    rtt_histogram: HistogramVec,
}

static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
    let registry = Registry::new();

    let connected_peers = IntGauge::new("libp2p_connected_peers", "Number of connected peers")
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
        .register(Box::new(rtt_histogram.clone()))
        .expect("metrics failed to initialise");

    Metrics {
        registry,
        connected_peers,
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
