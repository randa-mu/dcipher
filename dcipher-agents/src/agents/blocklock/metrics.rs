use prometheus::proto::MetricFamily;
use prometheus::{IntCounter, IntCounterVec, Opts, Registry};
use std::sync::LazyLock;

pub struct Metrics {
    registry: Registry,
    missing_events: IntCounter,
    errors_total: IntCounterVec,
    sync_success: IntCounter,
    decryption_requests: IntCounter,
    decryption_success: IntCounter,
}

static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
    let registry = Registry::new();

    let missing_events = IntCounter::new("missing_events_total", "Missing events seen")
        .expect("metrics failed to initialise");

    let errors_total = IntCounterVec::new(
        Opts::new("errors_total", "Total number of errors by type"),
        &["type"],
    )
    .expect("metrics failed to initialise");

    let sync_success = IntCounter::new("sync_success_total", "Successful syncs")
        .expect("metrics failed to initialise");
    let decryption_requests = IntCounter::new("decryption_requested_total", "Decryptions requested that don't yet have their conditions met")
        .expect("metrics failed to initialise");
    let decryption_success = IntCounter::new("decryption_success_total", "Successful decryptions")
        .expect("metrics failed to initialise");

    registry
        .register(Box::new(missing_events.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(errors_total.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(decryption_requests.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(decryption_success.clone()))
        .expect("metrics failed to initialise");

    Metrics {
        registry,
        missing_events,
        errors_total,
        sync_success,
        decryption_requests,
        decryption_success,
    }
});

impl Metrics {
    pub fn report_missing_events(count: u64) {
        METRICS.missing_events.inc_by(count)
    }
    
    pub fn report_sync_success() {
        METRICS.sync_success.inc()
    }
    
    pub fn report_sync_error() {
        METRICS.errors_total.with_label_values(&["sync_error"]).inc();
    }

    pub fn report_scheme_error() {
        METRICS.errors_total.with_label_values(&["invalid_scheme"]).inc();
    }
    
    pub fn report_storage_error() {
        METRICS.errors_total.with_label_values(&["storage_error"]).inc();
    }

    pub fn report_fetch_requests_error() {
        METRICS.errors_total.with_label_values(&["fetch_requests_error"]).inc();
    }

    pub fn report_decryption_requested() {
        METRICS.decryption_requests.inc();
    }

    pub fn report_decryption_success() {
        METRICS.decryption_success.inc();
    }

    pub fn report_decryption_error(count: u64) {
        METRICS.errors_total.with_label_values(&["decryption_error"]).inc_by(count );
    }

    pub fn gather() -> Vec<MetricFamily> {
        METRICS.registry.gather()
    }
}
