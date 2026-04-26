use axum::http;
use tower_http::trace::OnResponse;
use tracing::Level;
use tracing_otel_extra::dyn_event;

use super::http_route;

#[derive(Clone, Copy, Debug)]
pub struct AxumOtelOnResponse {
    level: Level,
}

impl Default for AxumOtelOnResponse {
    fn default() -> Self {
        Self {
            level: Level::DEBUG,
        }
    }
}

impl AxumOtelOnResponse {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl<B> OnResponse<B> for AxumOtelOnResponse {
    #[expect(
        clippy::cognitive_complexity,
        reason = "I don't think it is really that complex"
    )]
    fn on_response(
        self,
        response: &http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let status = response.status().as_u16();
        span.record(
            "http.status_code",
            tracing::field::display(status),
        );
        span.record("otel.status_code", "OK");

        let http_route = http_route(span)
            .unwrap_or_else(|| "unknown".into());

        dyn_event!(
            self.level,
            latency = %latency.as_millis(),
            status = %status,
            http.route = %http_route,
            "finished processing request"
        );
    }
}
