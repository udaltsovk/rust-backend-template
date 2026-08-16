use std::{future::Future, time::Duration};

use tokio::time::error::Elapsed;

pub trait TimeoutExt: Future + Sized {
    fn timeout_traced(
        self,
        limit: Duration,
        operation: &'static str,
    ) -> impl Future<Output = Result<Self::Output, Elapsed>> + Send
    where
        Self: Send,
        Self::Output: Send;
}

impl<F> TimeoutExt for F
where
    F: Future + Sized,
{
    async fn timeout_traced(
        self,
        limit: Duration,
        operation: &'static str,
    ) -> Result<Self::Output, Elapsed>
    where
        Self: Send,
        Self::Output: Send,
    {
        let result = tokio::time::timeout(limit, self).await;

        if result.is_err() {
            tracing::error!(
                operation,
                timeout_ms =
                    u64::try_from(limit.as_millis()).unwrap_or(u64::MAX),
                "operation timed out"
            );
        }

        result
    }
}
