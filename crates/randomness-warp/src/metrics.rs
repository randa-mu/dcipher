use prometheus::proto::MetricFamily;
use prometheus::{IntCounter, IntCounterVec, IntGauge, Opts, Registry};
use std::sync::LazyLock;

pub struct Metrics {
    registry: Registry,
    chain_height: IntGauge,
    missing_events: IntCounter,
    errors_total: IntCounterVec,
    sync_success: IntCounter,
    randomness_requests: IntCounter,
    randomness_fulfilled: IntCounter,
}

static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
    let registry = Registry::new();

    let chain_height = IntGauge::new("chain_height_reached", "Observed chain height")
        .expect("metrics failed to initialise");

    let missing_events = IntCounter::new("missing_events_total", "Missing events seen")
        .expect("metrics failed to initialise");

    let errors_total = IntCounterVec::new(
        Opts::new("errors_total", "Total number of errors by type"),
        &["type"],
    )
    .expect("metrics failed to initialise");

    let sync_success = IntCounter::new("sync_success_total", "Successful syncs")
        .expect("metrics failed to initialise");
    let randomness_requests = IntCounter::new(
        "randomness_requested_total",
        "Total number of randomness request received by the agent",
    )
    .expect("metrics failed to initialise");
    let randomness_fulfilled = IntCounter::new(
        "randomness_fulfilled_total",
        "Number of randomness requests that were fulfilled",
    )
    .expect("metrics failed to initialise");

    registry
        .register(Box::new(chain_height.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(missing_events.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(errors_total.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(sync_success.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(randomness_requests.clone()))
        .expect("metrics failed to initialise");
    registry
        .register(Box::new(randomness_fulfilled.clone()))
        .expect("metrics failed to initialise");

    Metrics {
        registry,
        chain_height,
        missing_events,
        errors_total,
        sync_success,
        randomness_requests,
        randomness_fulfilled,
    }
});

impl Metrics {
    pub(super) fn report_chain_height(chain_height: u64) {
        let chain_height: i64 = chain_height.try_into().unwrap_or_default();
        METRICS.chain_height.set(chain_height)
    }

    pub(super) fn report_missing_events(count: u64) {
        METRICS.missing_events.inc_by(count)
    }

    pub(super) fn report_sync_success() {
        METRICS.sync_success.inc()
    }

    pub(super) fn report_sync_error() {
        METRICS
            .errors_total
            .with_label_values(&["sync_error"])
            .inc();
    }

    pub(super) fn report_scheme_error() {
        METRICS
            .errors_total
            .with_label_values(&["invalid_scheme"])
            .inc();
    }

    pub(super) fn report_fetch_requests_error() {
        METRICS
            .errors_total
            .with_label_values(&["fetch_requests_error"])
            .inc();
    }

    pub(super) fn report_fulfillment_error() {
        METRICS
            .errors_total
            .with_label_values(&["fulfillment_failed"])
            .inc();
    }

    pub(super) fn report_subscription_insufficient_funds() {
        METRICS
            .errors_total
            .with_label_values(&["fulfillment_failed_subscription_insufficient_funds"])
            .inc();
    }

    pub(super) fn report_fulfillment_cost_too_high() {
        METRICS
            .errors_total
            .with_label_values(&["fulfillment_failed_cost_too_high"])
            .inc();
    }

    pub(super) fn report_fulfillment_profit_too_low() {
        METRICS
            .errors_total
            .with_label_values(&["fulfillment_failed_profit_too_low"])
            .inc();
    }

    pub(super) fn report_randomness_requested() {
        METRICS.randomness_requests.inc();
    }

    pub(super) fn report_randomness_fulfilled() {
        METRICS.randomness_fulfilled.inc();
    }

    pub fn gather() -> Vec<MetricFamily> {
        METRICS.registry.gather()
    }
}
