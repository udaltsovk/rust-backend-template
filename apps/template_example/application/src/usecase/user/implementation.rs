use domain::{
    session::CreateSession,
    user::{CreateUser, User},
};
use lib::{async_trait, domain::Id, instrument_all, tap::Pipe as _};

use crate::usecase::{
    UseCase,
    user::{
        UserUseCase,
        error::{UserUseCaseError, UserUseCaseResult},
    },
};

#[async_trait]
#[instrument_all]
impl UserUseCase for UseCase<User> {
    async fn create(&self, source: CreateUser) -> UserUseCaseResult<User> {
        if self
            .repositories
            .user_repository()
            .find_by_email(&source.email)
            .await
            .map_err(UserUseCaseError::Infrastructure)?
            .is_some()
        {
            return UserUseCaseError::EmailAlreadyUsed(source.email).pipe(Err);
        }

        let password_hash = self
            .services
            .password_hasher_service()
            .hash(&source.password)
            .map_err(UserUseCaseError::Infrastructure)?;

        self.repositories
            .user_repository()
            .create(Id::generate(), source, password_hash)
            .await
            .map_err(UserUseCaseError::Infrastructure)
    }

    async fn authorize(
        &self,
        source: CreateSession,
    ) -> UserUseCaseResult<User> {
        let user = self
            .repositories
            .user_repository()
            .find_by_email(&source.email)
            .await
            .map_err(UserUseCaseError::Infrastructure)?;

        self.services
            .password_hasher_service()
            .verify(&source.password, user.as_ref().map(|u| &u.password_hash))
            .map_err(|_| UserUseCaseError::InvalidPassword)?;

        let user = user.expect("we can't match nonexistent user password successfully so user should be Some at this point");

        Ok(user)
    }

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> UserUseCaseResult<Option<User>> {
        self.repositories
            .user_repository()
            .find_by_id(id)
            .await
            .map_err(UserUseCaseError::Infrastructure)?
            .pipe(Ok)
    }

    async fn get_by_id(&self, id: Id<User>) -> UserUseCaseResult<User> {
        self.find_by_id(id)
            .await?
            .ok_or(UserUseCaseError::NotFoundById(id))
    }
}
