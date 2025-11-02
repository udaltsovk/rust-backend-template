use axum::http::{Response, StatusCode};

pub trait ResponseExt {
    fn with_status(self, status: StatusCode) -> Self;
}

impl<T> ResponseExt for Response<T> {
    fn with_status(mut self, status: StatusCode) -> Self {
        *self.status_mut() = status;
        self
    }
}
