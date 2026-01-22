use tower_http::{classify::ServerErrorsFailureClass, trace::OnFailure};
use tracing::Level;
use tracing_otel_extra::dyn_event;

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct AxumOtelOnFailure {
    level: Level,
}

impl Default for AxumOtelOnFailure {
    fn default() -> Self {
        Self {
            level: Level::ERROR,
        }
    }
}

impl AxumOtelOnFailure {
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

impl OnFailure<ServerErrorsFailureClass> for AxumOtelOnFailure {
    #[expect(
        clippy::cognitive_complexity,
        reason = "I don't think it is really that complex"
    )]
    fn on_failure(
        &mut self,
        failure_classification: ServerErrorsFailureClass,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        dyn_event!(
            self.level,
            classification = %failure_classification,
            latency = %latency.as_millis(),
            "response failed"
        );
        match failure_classification {
            ServerErrorsFailureClass::StatusCode(status)
                if status.is_server_error() =>
            {
                span.record("otel.status_code", "ERROR");
            },
            _ => {},
        }
    }
}
