use std::net::SocketAddr;

use axum::{Json, Router, routing::get};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use utoipa::openapi::OpenApi;
use utoipa_scalar::{Scalar, Servable as _};

use crate::routes::fallback;

pub struct RestApi {
    pub router: Router,
}

impl RestApi {
    pub fn new<S>(openapi: OpenApi, router: Router<S>, modules: S) -> Self
    where
        S: Send + Sync + Clone + 'static,
    {
        let middlewares = ServiceBuilder::new().layer(CatchPanicLayer::new());

        let router = Router::new()
            .merge(router.with_state(modules))
            .merge(Scalar::with_url("/openapi", openapi.clone()))
            .route("/openapi.json", get(async move || Json(openapi.clone())))
            .fallback(fallback)
            .layer(middlewares);

        Self {
            router,
        }
    }

    pub async fn run(self, addr: SocketAddr) {
        let app = self.router.into_make_service();
        let listener = TcpListener::bind(addr)
            .await
            .expect("TcpListener cannot bind.");
        tracing::info!("Server is listening on {}", addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .expect("Server cannot launch.");
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    }
}
