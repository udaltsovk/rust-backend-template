use entrait::entrait;
use tracing::instrument;

use super::{UserUseCaseError, UserUseCaseResult};
use crate::features::{
    user::{
        application::repository::UserRepository,
        domain::User,
    },
    user_auth::{
        application::service::secret_hasher::SecretHasherService,
        domain::session::CreateSession,
    },
};

#[entrait(pub AuthorizeUserUsecase)]
#[instrument(skip(deps))]
async fn authorize_user<Deps>(
    deps: &Deps,
    source: CreateSession,
) -> UserUseCaseResult<User>
where
    Deps: UserRepository + SecretHasherService,
{
    let user = UserRepository::find_user_by_email(
        deps,
        &source.email,
    )
    .await?;

    SecretHasherService::verify_secret(
        deps,
        &source.password,
        user.as_ref().map(|u| &u.password_hash),
    )
    .map_err(|_| UserUseCaseError::InvalidPassword)?;

    user.ok_or(UserUseCaseError::InvalidPassword)
}
