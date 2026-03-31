use domain::user::{CreateUser, User};
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    service::secret_hasher::SecretHasherService,
    usecase::user::error::{UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub CreateUserUsecase)]
#[instrument(skip(app))]
async fn create_user<App>(
    app: &App,
    source: CreateUser,
) -> UserUseCaseResult<User>
where
    App: UserRepository + SecretHasherService,
{
    if app.find_user_by_email(&source.email).await?.is_some() {
        return UserUseCaseError::EmailAlreadyUsed(source.email).pipe(Err);
    }

    let password_hash = app.hash_secret(&source.password)?;

    app.create_user(Id::generate(), source, password_hash)
        .await?
        .pipe(Ok)
}
