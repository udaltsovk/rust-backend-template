use domain::session::{Session, entity::SessionEntity};
use lib::{async_trait, redact::Secret};

use crate::usecase::session::error::SessionUseCaseResult;

pub mod error;
pub mod implementation;

#[async_trait]
pub trait SessionUseCase {
    async fn create(
        &self,
        entity: SessionEntity,
    ) -> SessionUseCaseResult<Secret<String>>;

    async fn get_from_token(
        &self,
        token: Secret<&str>,
    ) -> SessionUseCaseResult<Session>;
}
