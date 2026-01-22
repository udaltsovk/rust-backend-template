use axum::http;
use tower_http::trace::OnResponse;
use tracing::Level;
use tracing_otel_extra::dyn_event;

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
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
        span.record("http.status_code", tracing::field::display(status));
        span.record("otel.status_code", "OK");

        dyn_event!(
            self.level,
            latency = %latency.as_millis(),
            status = %status,
            "finished processing request"
        );
    }
}
