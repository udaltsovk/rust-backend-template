#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use std::net::SocketAddr;

use axum::{Router, middleware::from_fn_with_state};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};
#[cfg(feature = "openapi")]
use {
    axum::{Json, routing::get},
    utoipa::openapi::OpenApi,
    utoipa_scalar::{
        Scalar, ScalarConfig, ScalarMcp, ScalarShowDeveloperTools, ScalarTheme,
    },
};

use super::{
    errors::envelope::ErrorEnvelope,
    mask::{self, ServerErrorMasking},
    negotiate::{self, BodyEncoder, ResponseFormat},
    panic_handler::PanicHandler,
    request_id::{self, RequestIdPolicy},
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
    pub cors: CorsLayer,
    pub modules: M,
    pub response_format: ResponseFormat,
    pub request_id_policy: RequestIdPolicy,
    pub mask_server_errors: ServerErrorMasking,
    #[cfg(feature = "openapi")]
    pub openapi: Option<OpenApi>,
}

impl<M> RestApiBuilder<M>
where
    M: Send + Sync + Clone + 'static,
{
    pub fn new(router: Router<M>, modules: &M) -> Self {
        Self {
            router,
            cors: CorsLayer::new(),
            modules: modules.clone(),
            response_format: ResponseFormat::default(),
            request_id_policy: RequestIdPolicy::default(),
            mask_server_errors: ServerErrorMasking::Disabled,
            #[cfg(feature = "openapi")]
            openapi: None,
        }
    }

    #[must_use]
    pub fn with_cors(mut self, cors: CorsLayer) -> Self {
        self.cors = cors;
        self
    }

    #[must_use]
    pub const fn with_request_id_policy(
        mut self,
        policy: RequestIdPolicy,
    ) -> Self {
        self.request_id_policy = policy;
        self
    }

    #[must_use]
    pub const fn with_masked_server_errors(
        mut self,
        masking: ServerErrorMasking,
    ) -> Self {
        self.mask_server_errors = masking;
        self
    }

    #[must_use]
    pub fn with_envelope<E>(mut self, envelope: E) -> Self
    where
        E: ErrorEnvelope,
    {
        self.response_format = ResponseFormat::new(envelope);
        self
    }

    #[must_use]
    pub fn with_encoders(
        mut self,
        encoders: Vec<Box<dyn BodyEncoder>>,
    ) -> Self {
        self.response_format = self.response_format.encoders(encoders);
        self
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
                .merge(Scalar::with_url_and_config(
                    "/openapi",
                    openapi,
                    ScalarConfig::builder()
                        .dark_mode(true)
                        .theme(ScalarTheme::Mars)
                        .show_developer_tools(ScalarShowDeveloperTools::Never)
                        .mcp(ScalarMcp::Disabled)
                        .build(),
                ))
                .route("/openapi.json", get(async move || openapi_json));
        }

        let middlewares = ServiceBuilder::new()
            .layer(from_fn_with_state(self.response_format, negotiate::apply))
            .layer(from_fn_with_state(
                self.mask_server_errors,
                mask::server_errors_if,
            ))
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(from_fn_with_state(
                self.request_id_policy,
                request_id::enforce,
            ))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        AxumOtelSpanCreator::new().level(Level::INFO),
                    )
                    .on_response(AxumOtelOnResponse::new().level(Level::INFO))
                    .on_failure(AxumOtelOnFailure::new()),
            )
            .layer(self.cors)
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
    pub fn builder<M>(router: Router<M>, modules: &M) -> RestApiBuilder<M>
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
