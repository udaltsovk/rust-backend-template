use std::net::SocketAddr;

use axum::{Json, Router, http::HeaderName, routing::get};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
};
use utoipa::OpenApi as _;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as _};

use crate::{context::openapi::ApiDoc, module::ModulesExt, routes};

pub struct App {
    pub router: Router,
}
impl App {
    const REQUEST_ID_HEADER: HeaderName =
        HeaderName::from_static("x-request-id");

    pub fn new<M: ModulesExt>(modules: M) -> Self {
        let (routes, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .merge(routes::router())
            .with_state(modules)
            .split_for_parts();

        let middlewares = ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(
                Self::REQUEST_ID_HEADER,
                MakeRequestUuid,
            ))
            .layer(PropagateRequestIdLayer::new(Self::REQUEST_ID_HEADER))
            .layer(CatchPanicLayer::new());

        let router = Router::new()
            .nest("/{api_version}", routes)
            .merge(Scalar::with_url("/openapi", api.clone()))
            .route("/openapi.json", get(async move || Json(api.clone())))
            .fallback(routes::fallback)
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
        tracing::info!("Server listening on {}", addr);

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
