use std::future::Future;

use application::health::ReadinessReport;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{self, MethodRouter},
};

pub struct ReadinessResponse(pub ReadinessReport);

impl From<ReadinessReport> for ReadinessResponse {
    fn from(report: ReadinessReport) -> Self {
        Self(report)
    }
}

impl IntoResponse for ReadinessResponse {
    fn into_response(self) -> Response {
        let status = if self.0.status.to_bool() {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        (status, Json(self.0)).into_response()
    }
}

pub trait ReadinessCheck {
    fn readiness(&self) -> impl Future<Output = ReadinessReport> + Send;
}

async fn readiness<M>(
    State(modules): State<entrait::Impl<M>>,
) -> ReadinessResponse
where
    M: ReadinessCheck + Clone + Send + Sync + 'static,
{
    modules.readiness().await.into()
}

pub fn ready<M>() -> MethodRouter<entrait::Impl<M>>
where
    M: ReadinessCheck + Clone + Send + Sync + 'static,
{
    routing::get(readiness::<M>)
}

pub fn live<S>() -> MethodRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    routing::get(async || StatusCode::NO_CONTENT)
}
