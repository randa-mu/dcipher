#[cfg(feature = "metrics")]
mod real_metrics {
    use prometheus::{HistogramOpts, HistogramVec, IntGauge, Registry};
    use std::sync::LazyLock;

    pub struct Metrics {
        pub(super) registry: Registry,
        pub(super) connected_peers: IntGauge,
        pub(super) rtt_histogram: HistogramVec,
    }

    pub(super) static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
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
}

#[cfg(feature = "metrics")]
pub use real_metrics::Metrics;

#[cfg(not(feature = "metrics"))]
pub struct Metrics;

impl Metrics {
    pub(super) fn report_peer_connected() {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS.connected_peers.inc();
    }

    pub(super) fn report_peer_disconnected() {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS.connected_peers.dec();
    }

    pub(super) fn report_host_rtt(rtt_seconds: f64, host: impl AsRef<str> + std::fmt::Debug) {
        #[cfg(feature = "metrics")]
        real_metrics::METRICS
            .rtt_histogram
            .with_label_values(&[host])
            .observe(rtt_seconds);
        #[cfg(not(feature = "metrics"))]
        {
            let _ = rtt_seconds;
            let _ = host;
        }
    }

    #[cfg(feature = "metrics")]
    pub fn gather() -> Vec<prometheus::proto::MetricFamily> {
        real_metrics::METRICS.registry.gather()
    }
}
