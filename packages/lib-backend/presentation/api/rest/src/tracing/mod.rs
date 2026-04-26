mod make_span;
mod on_failure;
mod on_response;

pub use make_span::AxumOtelSpanCreator;
pub use on_failure::AxumOtelOnFailure;
pub use on_response::AxumOtelOnResponse;
pub use tracing::Level;
use tracing_subscriber::{
    Registry, registry::LookupSpan as _,
};

use crate::errors::RequestMeta;

fn http_route(span: &tracing::Span) -> Option<String> {
    let mut http_route = None;

    span.with_subscriber(|(id, subscriber)| {
        if let Some(registry) =
            subscriber.downcast_ref::<Registry>()
            && let Some(span_ref) = registry.span(id)
            && let Some(meta) =
                span_ref.extensions().get::<RequestMeta>()
        {
            http_route = Some(meta.http_route.to_string());
        }
    });

    http_route
}
