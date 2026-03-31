use domain::{session::CreateSession, user::User};
use entrait::entrait;
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    service::secret_hasher::SecretHasherService,
    usecase::user::error::{UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub AuthorizeUserUsecase)]
#[instrument(skip(app))]
async fn authorize_user<App>(
    app: &App,
    source: CreateSession,
) -> UserUseCaseResult<User>
where
    App: UserRepository + SecretHasherService,
{
    let user = app
        .find_user_by_email(&source.email)
        .await
        .map_err(UserUseCaseError::Infrastructure)?;

    app.verify_secret(
        &source.password,
        user.as_ref().map(|u| &u.password_hash),
    )
    .map_err(|_| UserUseCaseError::InvalidPassword)?;

    let user = user.expect("we can't match nonexistent user password successfully so user should be Some at this point");

    Ok(user)
}
