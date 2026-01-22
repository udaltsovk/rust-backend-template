use std::{net::SocketAddr, str::FromStr as _};

use axum::{
    extract::ConnectInfo,
    http::{self, uri::PathAndQuery},
};
use opentelemetry::trace::SpanKind;
use tap::Pipe as _;
use tower_http::trace::MakeSpan;
use tracing::{Level, field::Empty};
use tracing_otel_extra::{
    dyn_span,
    extract::{context, fields},
};
use tracing_subscriber::{Registry, registry::LookupSpan as _};
use uuid::Uuid;

use crate::errors::RequestMeta;

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct AxumOtelSpanCreator {
    level: Level,
}

impl AxumOtelSpanCreator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            level: Level::TRACE,
        }
    }

    #[must_use]
    pub const fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl Default for AxumOtelSpanCreator {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> MakeSpan<B> for AxumOtelSpanCreator {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        let http_method = request.method().as_str();
        let http_route = request.uri().clone();

        let request_id = fields::extract_request_id(request)
            .pipe(Uuid::from_str)
            .expect("uuid from fields should be valid");

        let client_ip = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(ip)| tracing::field::debug(ip));

        let span_name = format!("{http_method} {http_route}");

        let span = dyn_span!(
            self.level,
            "request",
            http.client_ip = client_ip,
            http.versions = ?request.version(),
            http.host = ?fields::extract_host(request),
            http.method = ?fields::extract_http_method(request),
            http.route = http_route.to_string(),
            http.scheme = ?fields::extract_http_scheme(request),
            http.status_code = Empty,
            http.target = request.uri().path_and_query().map(PathAndQuery::as_str),
            http.user_agent = ?fields::extract_user_agent(request),
            otel.name = span_name,
            otel.kind = ?SpanKind::Server,
            otel.status_code = Empty,
            request_id = %request_id,
            trace_id = Empty
        );

        span.with_subscriber(|(id, subscriber)| {
            if let Some(registry) = subscriber.downcast_ref::<Registry>()
                && let Some(span_ref) = registry.span(id)
            {
                span_ref.extensions_mut().insert(RequestMeta {
                    http_route,
                    request_id: Some(request_id),
                });
            }
        });

        context::set_otel_parent(request.headers(), &span);
        span
    }
}
