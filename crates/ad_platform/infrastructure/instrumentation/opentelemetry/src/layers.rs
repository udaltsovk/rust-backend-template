use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    EnvFilter,
    filter::Directive,
    fmt::{
        self, Layer,
        format::{Compact, DefaultFields, Format},
    },
};

use crate::LGTM;

#[inline]
fn parse_directive(directive: &'static str) -> Directive {
    directive.parse().expect("Failed to parse directive")
}

impl LGTM {
    #[inline]
    pub(super) fn filter_layer() -> EnvFilter {
        EnvFilter::builder()
            .with_default_directive(
                if cfg!(debug_assertions) {
                    LevelFilter::DEBUG
                } else {
                    LevelFilter::INFO
                }
                .into(),
            )
            .from_env_lossy()
            .add_directive(parse_directive("tokio=off"))
            .add_directive(parse_directive("runtime=off"))
            .add_directive(parse_directive("hyper=off"))
            .add_directive(parse_directive("opentelemetry=off"))
            .add_directive(parse_directive("tonic=off"))
            .add_directive(parse_directive("h2=off"))
            .add_directive(parse_directive("tower=off"))
            .add_directive(parse_directive("reqwest=off"))
            .add_directive(parse_directive("aws=off"))
            .add_directive(parse_directive("rustls=off"))
            .add_directive(parse_directive("tungstenite=off"))
    }

    #[inline]
    pub(super) fn fmt_layer<S>() -> Layer<S, DefaultFields, Format<Compact>> {
        fmt::layer().compact()
    }
}
