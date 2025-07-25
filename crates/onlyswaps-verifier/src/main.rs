use axum::routing::get;
use axum::Router;
use clap::Parser;
use std::io::Error;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
struct CliConfig {
    #[arg(
        short = 'p',
        long = "port",
        env = "ONLYSWAPS_VERIFIER_PORT",
        default_value = "8080"
    )]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = CliConfig::parse();
    let app = Router::new().route("/health", get(healthcheck_handler));
    let listener = TcpListener::bind(("0.0.0.0", config.port)).await?;

    // Setup some signals
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    println!("Listening on port {}", config.port);
    tokio::select! {
        _ = sigterm.recv() => {
            println!("received SIGTERM, shutting down...");
            Ok(())
        },

        _ = sigint.recv() => {
            println!("received SIGINT, shutting down...");
            Ok(())
        },

        _ = tokio::signal::ctrl_c() => {
            println!("received ctrl+c, shutting down...");
            Ok(())
        },

        err = axum::serve(listener, app) => {
            eprintln!("axum stopped unexpectedly...");
            err
        }
    }
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}
