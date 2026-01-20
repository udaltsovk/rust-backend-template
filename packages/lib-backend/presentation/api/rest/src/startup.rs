use std::net::SocketAddr;

use axum::Router;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};
#[cfg(feature = "openapi")]
use {
    axum::{Json, routing::get},
    utoipa::openapi::OpenApi,
    utoipa_scalar::{Scalar, Servable as _},
};

use crate::{
    panic_handler::PanicHandler,
    routes::{fallback_404, fallback_405},
    tracing::{
        AxumOtelOnFailure, AxumOtelOnResponse, AxumOtelSpanCreator, Level,
    },
};

pub struct RestApiBuilder<M>
where
    M: Send + Sync + Clone + 'static,
{
    pub router: Router<M>,
    pub modules: M,
    #[cfg(feature = "openapi")]
    pub openapi: Option<OpenApi>,
}

impl<M> RestApiBuilder<M>
where
    M: Send + Sync + Clone + 'static,
{
    pub const fn new(router: Router<M>, modules: M) -> Self {
        Self {
            router,
            modules,
            #[cfg(feature = "openapi")]
            openapi: None,
        }
    }

    #[cfg(feature = "openapi")]
    #[must_use]
    pub fn with_openapi(mut self, openapi: OpenApi) -> Self {
        self.openapi = Some(openapi);
        self
    }

    fn router(router: Router<M>, modules: M) -> Router<()> {
        Router::new().merge(router.with_state(modules))
    }

    pub fn build(self) -> RestApi {
        #[cfg(feature = "openapi")]
        let mut router = Self::router(self.router, self.modules);

        #[cfg(not(feature = "openapi"))]
        let router = Self::router(self.router, self.modules);

        #[cfg(feature = "openapi")]
        if let Some(openapi) = self.openapi {
            let openapi_json = Json(openapi.clone());
            router = router
                .merge(Scalar::with_url("/openapi", openapi))
                .route("/openapi.json", get(async move || openapi_json));
        }

        let middlewares = ServiceBuilder::new()
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        AxumOtelSpanCreator::new().level(Level::INFO),
                    )
                    .on_response(AxumOtelOnResponse::new().level(Level::INFO))
                    .on_failure(AxumOtelOnFailure::new()),
            )
            .layer(PanicHandler::layer());

        let router = router
            .fallback(fallback_404)
            .method_not_allowed_fallback(fallback_405)
            .layer(middlewares);

        RestApi {
            router,
        }
    }
}

pub struct RestApi {
    pub(crate) router: Router,
}

impl RestApi {
    pub const fn builder<M>(router: Router<M>, modules: M) -> RestApiBuilder<M>
    where
        M: Send + Sync + Clone + 'static,
    {
        RestApiBuilder::new(router, modules)
    }

    #[must_use]
    pub fn is_openapi_route(path: &str) -> bool {
        ["/openapi", "/openapi.json"].contains(&path)
    }

    pub async fn run(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr)
            .await
            .expect("TcpListener cannot bind.");
        self.serve(listener).await;
    }

    pub async fn serve(self, listener: TcpListener) {
        self.serve_with_shutdown(listener, Self::shutdown_signal())
            .await;
    }

    pub async fn serve_with_shutdown<F>(self, listener: TcpListener, signal: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let app = self.router.into_make_service();
        let addr = listener
            .local_addr()
            .expect("TcpListener must have a local address.");
        tracing::info!("Server is listening on {}", addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(signal)
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
            () = ctrl_c => {},
            () = terminate => {},
        }
    }
}
