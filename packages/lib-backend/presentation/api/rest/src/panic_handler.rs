use std::any::Any;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse as _,
};
use tower_http::catch_panic::{CatchPanicLayer, ResponseForPanic};

use crate::errors::JsonError;

#[derive(Clone)]
pub struct PanicHandler;

impl PanicHandler {
    pub fn layer() -> CatchPanicLayer<Self> {
        CatchPanicLayer::custom(Self)
    }
}

impl ResponseForPanic for PanicHandler {
    type ResponseBody = Body;

    fn response_for_panic(
        &mut self,
        _err: Box<dyn Any + Send + 'static>,
    ) -> Response<Self::ResponseBody> {
        tracing::error!("Service panicked");

        JsonError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR",
            "Service panicked",
        )
        .into_response()
    }
}
