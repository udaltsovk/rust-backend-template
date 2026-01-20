use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    EnvFilter,
    filter::Directive,
    fmt::{
        self, Layer,
        format::{self, Format},
    },
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
};

#[inline]
fn parse_directive(directive: &'static str) -> Directive {
    directive.parse().expect("Failed to parse directive")
}

#[must_use]
#[inline]
pub fn filter_layer() -> EnvFilter {
    let default_level = if cfg!(debug_assertions) {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    EnvFilter::builder()
        .with_default_directive(default_level.into())
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

#[must_use]
#[inline]
pub fn fmt_layer<S>() -> Layer<S, format::DefaultFields, Format<format::Full>> {
    fmt::layer()
        .with_span_events(format::FmtSpan::CLOSE)
        .with_line_number(true)
        .with_thread_names(true)
        .log_internal_errors(true)
        .with_level(true)
        .with_target(true)
}

pub async fn wrap<F>(future: F)
where
    F: Future<Output = ()>,
{
    tracing_subscriber::registry()
        .with(filter_layer())
        .with(fmt_layer())
        .init();

    future.await;
}

#[cfg(test)]
mod tests {
    use tracing_subscriber::Registry;

    use super::*;

    #[test]
    fn parse_directive_valid() {
        let directive = parse_directive("info");
        assert_eq!(directive.to_string(), "info");
    }

    #[test]
    #[should_panic(expected = "Failed to parse directive")]
    fn parse_directive_invalid() {
        parse_directive("invalid=directive=format");
    }

    #[test]
    fn filter_layer_creation() {
        let filter = filter_layer();
        let filter_str = filter.to_string();
        assert!(filter_str.contains("tokio=off"));
        assert!(filter_str.contains("hyper=off"));
    }

    #[test]
    fn fmt_layer_creation() {
        let _layer = fmt_layer::<Registry>();
    }
}
