use std::net::SocketAddr;

use axum::{Json, Router, routing::get};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use utoipa::openapi::OpenApi;
use utoipa_scalar::{Scalar, Servable as _};

use crate::routes::{fallback_404, fallback_405};

pub struct RestApiBuilder<M>
where
    M: Send + Sync + Clone + 'static,
{
    pub router: Router<M>,
    pub modules: M,
    pub openapi: Option<OpenApi>,
}

impl<M> RestApiBuilder<M>
where
    M: Send + Sync + Clone + 'static,
{
    pub fn new(router: Router<M>, modules: M) -> Self {
        Self {
            router,
            modules,
            openapi: None,
        }
    }

    pub fn with_openapi(mut self, openapi: OpenApi) -> Self {
        self.openapi = Some(openapi);
        self
    }

    pub fn build(self) -> RestApi {
        let mut router =
            Router::new().merge(self.router.with_state(self.modules));

        if let Some(openapi) = self.openapi {
            router = router
                .merge(Scalar::with_url("/openapi", openapi.clone()))
                .route(
                    "/openapi.json",
                    get(async move || Json(openapi.clone())),
                );
        }

        let middlewares = ServiceBuilder::new().layer(CatchPanicLayer::new());

        let router = router
            .layer(middlewares)
            .fallback(fallback_404)
            .method_not_allowed_fallback(fallback_405);

        RestApi {
            router,
        }
    }
}

pub struct RestApi {
    router: Router,
}

impl RestApi {
    pub fn builder<M>(router: Router<M>, modules: M) -> RestApiBuilder<M>
    where
        M: Send + Sync + Clone + 'static,
    {
        RestApiBuilder::new(router, modules)
    }

    pub fn is_openapi_route(path: &str) -> bool {
        ["/openapi", "/openapi.json"].contains(&path)
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
