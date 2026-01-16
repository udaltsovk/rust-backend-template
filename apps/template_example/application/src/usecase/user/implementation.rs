use domain::{
    session::CreateSession,
    user::{CreateUser, User},
};
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
    async fn create(
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

    async fn authorize(
        &self,
        source: CreateSession,
    ) -> UserUseCaseResult<R, S, User> {
        let (user, password_hash) = self
            .repositories
            .user_repository()
            .find_by_email(&source.email)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?
            .ok_or(UserUseCaseError::NotFoundByEmail {
                email: source.email,
                from_auth: true,
            })?;

        self.services
            .password_hasher_service()
            .verify(source.password.as_bytes(), &password_hash.0)
            .map_err(|_| UserUseCaseError::InvalidPassword)?;

        Ok(user)
    }

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> UserUseCaseResult<R, S, Option<User>> {
        self.repositories
            .user_repository()
            .find_by_id(id)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?
            .map(|(user, _)| user)
            .pipe(Ok)
    }

    async fn get_by_id(&self, id: Id<User>) -> UserUseCaseResult<R, S, User> {
        self.find_by_id(id)
            .await?
            .ok_or(UserUseCaseError::NotFoundById(id))
    }
}
