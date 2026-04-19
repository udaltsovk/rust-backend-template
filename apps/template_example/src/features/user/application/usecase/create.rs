use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use super::{UserUseCaseError, UserUseCaseResult};
use crate::features::{
    user::{
        application::repository::UserRepository,
        domain::{CreateUser, User},
    },
    user_auth::application::service::secret_hasher::SecretHasherService,
};

#[entrait(pub CreateUserUsecase)]
#[instrument(skip(deps))]
async fn create_user<Deps>(
    deps: &Deps,
    source: CreateUser,
) -> UserUseCaseResult<User>
where
    Deps: UserRepository + SecretHasherService,
{
    if UserRepository::find_user_by_email(
        deps,
        &source.email,
    )
    .await?
    .is_some()
    {
        return UserUseCaseError::EmailAlreadyUsed(
            source.email,
        )
        .pipe(Err);
    }

    let password_hash = SecretHasherService::hash_secret(
        deps,
        &source.password,
    )?;

    UserRepository::create_user(
        deps,
        Id::generate(),
        source,
        password_hash,
    )
    .await?
    .pipe(Ok)
}
