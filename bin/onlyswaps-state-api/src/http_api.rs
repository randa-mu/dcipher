use crate::config::ApiConfig;
use crate::service::{StateService, SwapTransactionQueryFilter};
use crate::state::SwapTransaction;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use std::net::SocketAddrV4;
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct HttpApi<S: StateService> {
    listener: TcpListener,
    service: Arc<S>,
}

impl<S: StateService + 'static> HttpApi<S> {
    pub async fn new(config: &ApiConfig, service: S) -> anyhow::Result<Self> {
        let socket = SocketAddrV4::new(config.hostname, config.port);
        tracing::info!("Binding on {}", socket);
        let listener = TcpListener::bind(socket).await?;
        Ok(Self {
            listener,
            service: Arc::new(service),
        })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let router = Router::new()
            .route("/transactions", get(get_transactions::<S>))
            .layer(axum::Extension(Arc::clone(&self.service)));

        tracing::info!("API server starting");
        Ok(axum::serve(self.listener, router).await?)
    }
}

pub async fn get_transactions<S: StateService>(
    Query(filter): Query<SwapTransactionQueryFilter>,
    axum::extract::Extension(service): axum::extract::Extension<Arc<S>>,
) -> Result<Json<Vec<SwapTransaction>>, (axum::http::StatusCode, String)> {
    tracing::debug!("received get transaction request; filter={:?}", filter);
    service.get_transactions(filter).map(Json).map_err(|err| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )
    })
}
