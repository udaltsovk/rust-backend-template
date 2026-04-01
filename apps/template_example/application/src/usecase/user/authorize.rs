use domain::{session::CreateSession, user::User};
use entrait::entrait;
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    service::secret_hasher::SecretHasherService,
    usecase::user::{UserUseCaseError, UserUseCaseResult},
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
    let user = UserRepository::find_user_by_email(deps, &source.email).await?;

    SecretHasherService::verify_secret(
        deps,
        &source.password,
        user.as_ref().map(|u| &u.password_hash),
    )
    .map_err(|_| UserUseCaseError::InvalidPassword)?;

    user.ok_or(UserUseCaseError::InvalidPassword)
}
