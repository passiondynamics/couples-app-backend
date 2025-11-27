//! Author: irith
//! Date: 2025-11-08 @ 1:03pm
//! Description: TODO

use anyhow::{
    Context,
    Result,
};
use axum::{
    body::Body,
    routing::{
//        delete,
        get,
        post,
//        put,
    },
    Router,
};
use http::{
    Request,
    Response,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{
    info,
    info_span,
    Span,
};

use std::sync::Arc;
use std::time::Duration;

use crate::interfaces::http::handlers::{
    add_user,
    health,
};
use crate::models::config::Config;
use crate::services::AppService;

mod handlers;


/// Dynamic state to be shared across HTTP handlers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct AppState<A>
where
    A: AppService,
{
    app: Arc<A>,
}


/// An implementation of (incoming) HTTP request access to dictate intent.
pub struct HTTPInterface {
    router: Router,
    listener: TcpListener,
}

impl HTTPInterface {
    /// Create a new HTTP router/listener with attached logging and app
    /// state.
    pub async fn new(config: &Config, raw_app: impl AppService) -> Result<Self> {
        // Attach a trace span on each request with an automatic log for the
        // request info + response status.
        let trace_layer = TraceLayer::new_for_http()
                                     .make_span_with(|r: &Request<Body>| {
                                         info_span!(
                                             "request",
                                             method=%r.method(),
                                             uri=%r.uri(),
                                        )
                                     })
                                     .on_request(())
                                     .on_response(|r: &Response<Body>, l: Duration, _s: &Span| {
                                         info!(
                                             status=r.status().as_u16(),
                                             latency=?l,
                                         )
                                     });
        let middleware = ServiceBuilder::new()
                                        .layer(trace_layer);

        let app = Arc::new(raw_app);
        let app_state = AppState {app};
        let router = Router::new()
                            .route("/health", get(health))
                            .route("/add_user", post(add_user))
                            .layer(middleware)
                            .with_state(app_state);
        let listener = TcpListener::bind(("0.0.0.0", config.port))
                                   .await
                                   .with_context(|| "could not bind listener")?;

        Ok(Self {router, listener})
    }


    /// Start HTTP server.
    pub async fn serve(self) -> Result<()> {
        //let http_fut = axum::serve(listener, router)
        //                    .with_graceful_shutdown(async move {
        //                        http_token.cancelled().await;
        //                        debug!("HTTP server cancelled...");
        //                    }); // Type hints are too complicated to move into `http::init`. 
        axum::serve(self.listener, self.router)
             .await
             .with_context(|| "could not server on HTTP interface")?;

        Ok(())
    }
}
