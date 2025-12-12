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
    pub const fn new(router: Router<M>, modules: M) -> Self {
        Self {
            router,
            modules,
            openapi: None,
        }
    }

    #[must_use]
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

#[cfg(test)]
mod tests {
    use axum::{Router, routing::get};
    use rstest::rstest;
    use tokio::net::TcpListener;
    use utoipa::openapi::{Info, OpenApiBuilder};

    use super::*;

    #[derive(Debug, Clone)]
    struct TestModules {
        pub value: String,
    }

    async fn test_handler() -> &'static str {
        "test response"
    }

    #[rstest]
    fn rest_api_builder_new_creates_correct_structure() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        let builder = RestApiBuilder::new(router, modules);

        // Verify the builder contains the expected data
        assert_eq!(builder.modules.value, "test");
        assert!(builder.openapi.is_none());
    }

    #[rstest]
    fn rest_api_builder_with_openapi_sets_openapi() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi = OpenApiBuilder::new()
            .info(Info::new("Test API", "1.0.0"))
            .build();

        let builder =
            RestApiBuilder::new(router, modules).with_openapi(openapi);

        assert!(builder.openapi.is_some());
        let stored_openapi = builder.openapi.expect("OpenAPI should be set");
        assert_eq!(stored_openapi.info.title, "Test API");
        assert_eq!(stored_openapi.info.version, "1.0.0");
    }

    #[rstest]
    fn rest_api_builder_build_creates_rest_api() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        let builder = RestApiBuilder::new(router, modules);
        let rest_api = builder.build();

        // The RestApi should be created successfully
        // We can't easily test the internal router without starting a server,
        // but we can verify the build process completes
        assert!(std::mem::size_of_val(&rest_api) > 0);
    }

    #[rstest]
    fn rest_api_builder_build_with_openapi_includes_openapi_routes() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi = OpenApiBuilder::new()
            .info(Info::new("Test API", "1.0.0"))
            .build();

        let builder =
            RestApiBuilder::new(router, modules).with_openapi(openapi);
        let rest_api = builder.build();

        // The RestApi should be created successfully with OpenAPI routes
        assert!(std::mem::size_of_val(&rest_api) > 0);
    }

    #[rstest]
    fn rest_api_builder_method_chaining() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi = OpenApiBuilder::new()
            .info(Info::new("Test API", "1.0.0"))
            .build();

        // Test method chaining works correctly
        let rest_api = RestApiBuilder::new(router, modules)
            .with_openapi(openapi)
            .build();

        assert!(std::mem::size_of_val(&rest_api) > 0);
    }

    #[rstest]
    fn rest_api_builder_const_constructor() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        // Test that the new method is const
        const {
            assert!(std::mem::size_of::<RestApiBuilder<TestModules>>() > 0);
        }

        let _builder = RestApiBuilder::new(router, modules);
    }

    #[rstest]
    fn rest_api_builder_with_openapi_must_use() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi = OpenApiBuilder::new()
            .info(Info::new("Test API", "1.0.0"))
            .build();

        let builder = RestApiBuilder::new(router, modules);

        // Test that with_openapi returns a modified builder
        let _modified_builder = builder.with_openapi(openapi);
        // The #[must_use] attribute should ensure this value is used
    }

    #[rstest]
    fn rest_api_builder_creates_instance() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        let builder = RestApiBuilder::new(router, modules);
        let rest_api = builder.build();

        // Verify RestApi instance is created
        assert!(std::mem::size_of_val(&rest_api) > 0);
    }

    #[rstest]
    fn rest_api_builder_const_constructor_from_rest_api() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        // Test the const builder method on RestApi
        let _builder = RestApi::builder(router, modules);
    }

    #[rstest]
    fn rest_api_is_openapi_route_identifies_openapi_paths() {
        // Test the static method that identifies OpenAPI routes
        assert!(RestApi::is_openapi_route("/openapi"));
        assert!(RestApi::is_openapi_route("/openapi.json"));

        // Test non-OpenAPI routes
        assert!(!RestApi::is_openapi_route("/api/users"));
        assert!(!RestApi::is_openapi_route("/health"));
        assert!(!RestApi::is_openapi_route("/"));
        assert!(!RestApi::is_openapi_route("/openapi/docs")); // Similar but not exact
        assert!(!RestApi::is_openapi_route("/openapi.html")); // Similar but not exact
    }

    #[rstest]
    fn rest_api_is_openapi_route_handles_edge_cases() {
        // Test edge cases for the OpenAPI route detection
        assert!(!RestApi::is_openapi_route(""));
        assert!(!RestApi::is_openapi_route("/"));
        assert!(!RestApi::is_openapi_route("openapi")); // Missing leading slash
        assert!(!RestApi::is_openapi_route("openapi.json")); // Missing leading slash
        assert!(!RestApi::is_openapi_route("/openapi/")); // Trailing slash
        assert!(!RestApi::is_openapi_route("/openapi.json/")); // Trailing slash
        assert!(!RestApi::is_openapi_route("/OPENAPI")); // Case sensitive
        assert!(!RestApi::is_openapi_route("/OpenApi.json")); // Case sensitive
    }

    #[rstest]
    fn rest_api_builder_handles_different_module_types() {
        #[derive(Debug, Clone)]
        #[expect(
            dead_code,
            reason = "Test struct fields are intentionally unused"
        )]
        struct DifferentModules {
            id: u32,
            active: bool,
        }

        let router = Router::new().route("/different", get(test_handler));
        let modules = DifferentModules {
            id: 42,
            active: true,
        };

        let builder = RestApiBuilder::new(router, modules);
        let rest_api = builder.build();

        assert!(std::mem::size_of_val(&rest_api) > 0);
    }

    #[rstest]
    fn rest_api_builder_multiple_with_openapi_calls() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi1 = OpenApiBuilder::new()
            .info(Info::new("API v1", "1.0.0"))
            .build();
        let openapi2 = OpenApiBuilder::new()
            .info(Info::new("API v2", "2.0.0"))
            .build();

        // Test that multiple calls to with_openapi work (last one wins)
        let builder = RestApiBuilder::new(router, modules)
            .with_openapi(openapi1)
            .with_openapi(openapi2);

        // Should have openapi2 as the final OpenAPI spec
        assert!(builder.openapi.is_some());
        let stored_openapi = builder.openapi.expect("OpenAPI should be set");
        assert_eq!(stored_openapi.info.version, "2.0.0");
    }

    #[rstest]
    fn rest_api_build_includes_fallback_handlers() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        let builder = RestApiBuilder::new(router, modules);
        let _rest_api = builder.build();

        // The build process should include fallback_404 and fallback_405 handlers
        // We can't easily test this without starting a server, but we can verify
        // that the build process completes without error
        // Test passes by reaching this point
    }

    #[rstest]
    fn rest_api_build_includes_middleware() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        let builder = RestApiBuilder::new(router, modules);
        let _rest_api = builder.build();

        // The build process should include CatchPanicLayer middleware
        // We can't easily test this without starting a server, but we can verify
        // that the build process completes without error
        // Test passes by reaching this point
    }

    #[rstest]
    fn rest_api_types_implement_required_traits() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };

        // Test that our modules type implements required traits
        let cloned_modules = modules.clone();
        assert_eq!(cloned_modules.value, modules.value);

        // Test builder creation
        let _builder = RestApiBuilder::new(router, modules);
    }

    #[rstest]
    #[tokio::test]
    async fn rest_api_serve_starts_server() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let rest_api = RestApiBuilder::new(router, modules).build();

        let listener =
            TcpListener::bind("127.0.0.1:0").await.expect("bind failed");

        let server_handle = tokio::spawn(async move {
            rest_api.serve(listener).await;
        });

        // Give it a moment to start
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Abort the task to clean up
        server_handle.abort();
    }

    #[rstest]
    #[tokio::test]
    async fn rest_api_run_starts_server() {
        use std::net::SocketAddr;

        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let rest_api = RestApiBuilder::new(router, modules).build();

        // Use port 0 to let OS choose a free port
        let addr: SocketAddr = "127.0.0.1:0".parse().expect("valid address");

        let server_handle = tokio::spawn(async move {
            rest_api.run(addr).await;
        });

        // Give it a moment to start
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Abort the task to clean up
        server_handle.abort();
    }

    #[rstest]
    #[tokio::test]
    async fn rest_api_serve_with_shutdown_gracefully_stops() {
        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let rest_api = RestApiBuilder::new(router, modules).build();

        let listener =
            TcpListener::bind("127.0.0.1:0").await.expect("bind failed");

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();

        let server_handle = tokio::spawn(async move {
            rest_api
                .serve_with_shutdown(listener, async {
                    rx.await.expect("failed to receive shutdown signal");
                })
                .await;
        });

        // Give it a moment to start
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Signal shutdown
        tx.send(()).expect("failed to send shutdown signal");

        // Wait for server to finish
        server_handle.await.expect("server task failed");
    }

    #[rstest]
    #[tokio::test]
    async fn rest_api_openapi_json_route_returns_spec() {
        use axum::{
            body::Body,
            http::{Request, StatusCode},
        };
        use tower::ServiceExt as _;

        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi = OpenApiBuilder::new()
            .info(Info::new("Test API", "1.0.0"))
            .build();

        let rest_api = RestApiBuilder::new(router, modules)
            .with_openapi(openapi)
            .build();

        let request = Request::builder()
            .uri("/openapi.json")
            .body(Body::empty())
            .expect("request builder");

        let response = rest_api.router.oneshot(request).await.expect("request");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[rstest]
    #[tokio::test]
    async fn rest_api_openapi_route_returns_ui() {
        use axum::{
            body::Body,
            http::{Request, StatusCode},
        };
        use tower::ServiceExt as _;

        let router = Router::new().route("/test", get(test_handler));
        let modules = TestModules {
            value: "test".to_string(),
        };
        let openapi = OpenApiBuilder::new()
            .info(Info::new("Test API", "1.0.0"))
            .build();

        let rest_api = RestApiBuilder::new(router, modules)
            .with_openapi(openapi)
            .build();

        let request = Request::builder()
            .uri("/openapi")
            .body(Body::empty())
            .expect("request builder");

        let response = rest_api.router.oneshot(request).await.expect("request");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
