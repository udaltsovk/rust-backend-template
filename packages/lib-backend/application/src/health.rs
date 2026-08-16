use std::{
    future::Future,
    time::{Duration, Instant},
};

use futures_util::future::{BoxFuture, join_all};
use result_like::BoolLike;
use serde::Serialize;
use tap::Pipe as _;
use tokio::time::timeout;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(2);

pub type CheckResult = Result<(), String>;

#[derive(BoolLike, Serialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    Healthy,
    Unhealthy,
}

pub trait HealthCheck {
    fn health_check(
        &self,
    ) -> impl Future<Output = CheckResult> + Send + 'static;
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckOutcome {
    pub name: &'static str,

    pub status: Status,

    pub healthy: bool,

    pub duration_ms: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReadinessReport {
    pub status: Status,
    pub checks: Vec<CheckOutcome>,
}

struct Check {
    name: &'static str,
    future: BoxFuture<'static, CheckResult>,
}

impl Check {
    async fn run(self, limit: Duration) -> CheckOutcome {
        let started = Instant::now();

        let error = match timeout(limit, self.future).await {
            Ok(Ok(())) => None,
            Ok(Err(error)) => Some(error),
            Err(_) => Some(format!("timed out after {}ms", limit.as_millis())),
        };

        let status = error.is_none().into();

        CheckOutcome {
            name: self.name,
            status,
            healthy: status.eq(&Status::Healthy),
            duration_ms: u64::try_from(started.elapsed().as_millis())
                .unwrap_or(u64::MAX),
            error,
        }
    }
}

pub struct Readiness {
    timeout: Duration,
    checks: Vec<Check>,
}

impl Readiness {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            checks: Vec::new(),
        }
    }

    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    #[must_use]
    pub fn check<F>(mut self, name: &'static str, check: F) -> Self
    where
        F: Future<Output = CheckResult> + Send + 'static,
    {
        self.checks.push(Check {
            name,
            future: check.pipe(Box::pin),
        });
        self
    }

    #[must_use]
    pub fn probe<C>(self, name: &'static str, check: &C) -> Self
    where
        C: HealthCheck,
    {
        self.check(name, check.health_check())
    }

    pub async fn run(self) -> ReadinessReport {
        let limit = self.timeout;

        let checks =
            join_all(self.checks.into_iter().map(|check| check.run(limit)))
                .await;

        ReadinessReport {
            status: checks.iter().all(|check| check.status.to_bool()).into(),
            checks,
        }
    }
}

impl Default for Readiness {
    fn default() -> Self {
        Self::new()
    }
}
