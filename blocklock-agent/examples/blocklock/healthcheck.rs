use anyhow::anyhow;
use dcipher_agents::agents::blocklock::metrics::Metrics;
use prometheus::{Encoder, TextEncoder};
use warp::http::StatusCode;
use warp::Filter;

pub async fn start_api(port: u16) -> anyhow::Result<()> {
    let health = warp::path!("health")
        .map(warp::reply)
        .map(|reply| warp::reply::with_status(reply, StatusCode::OK));

    let metrics = warp::path!("metrics").map(|| {
        let encoder = TextEncoder::new();
        let metrics = Metrics::gather();
        let mut buffer = Vec::new();

        match encoder.encode(&metrics, &mut buffer) {
            Err(_) => warp::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Vec::new()),

            Ok(()) => warp::http::Response::builder()
                .header("Content-Type", encoder.format_type())
                .body(buffer),
        }
    });

    let routes = health.or(metrics);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    Err(anyhow!("HTTP API server stopped"))
}
