use domain::user::{CreateUser, User};
use lib::{async_trait, domain::Id, instrument_all, tap::Pipe as _};

use crate::{
    repository::{RepositoriesModuleExt, user::UserRepository as _},
    service::{ServicesModuleExt, hasher::HasherService as _},
    usecase::{
        UseCase,
        user::{
            UserUseCase,
            error::{UserUseCaseError, UserUseCaseResult},
        },
    },
};

#[async_trait]
#[instrument_all]
impl<R, S> UserUseCase<R, S> for UseCase<R, S, User>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn register(
        &self,
        source: CreateUser,
    ) -> UserUseCaseResult<R, S, User> {
        if self
            .repositories
            .user_repository()
            .find_by_email(&source.email)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?
            .is_some()
        {
            return UserUseCaseError::EmailAlreadyUsed(source.email).pipe(Err);
        }

        let password_hash = self
            .services
            .password_hasher_service()
            .hash(source.password.as_bytes())
            .map_err(S::Error::from)
            .map_err(UserUseCaseError::Service)?;

        self.repositories
            .user_repository()
            .create(Id::generate(), source, password_hash)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)
    }
}
