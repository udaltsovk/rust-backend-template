use application::repository::user::UserRepository;
use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User},
};
use lib::{
    async_trait,
    domain::{DomainType, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::{Conv as _, Pipe as _},
};
use sqlx::query_file_as;

use crate::{
    entity::user::{StoredUser, target_settings::StoredUserTargetSettings},
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl UserRepository for PostgresRepositoryImpl<User> {
    type AdapterError = PostgresAdapterError;

    async fn create(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: String,
    ) -> Result<User, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let id = id.value;
        let name = source.name.into_inner();
        let surname = source.surname.into_inner();
        let email = source.email.into_inner();
        let avatar_url =
            source.avatar_url.flatten().map(DomainType::into_inner);
        let target_settings: StoredUserTargetSettings =
            source.target_settings.into();

        let user = query_file_as!(
            StoredUser,
            "sql/user/create.sql",
            id,
            name,
            surname,
            email,
            password_hash,
            avatar_url,
            target_settings as _
        )
        .fetch_one(&mut *connection)
        .await?
        .conv::<User>();

        Ok(user)
    }

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<(User, PasswordHash)>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(StoredUser, "sql/user/find_by_id.sql", id.value)
            .fetch_optional(&mut *connection)
            .await?
            .map(StoredUser::into_domain_tuple)
            .pipe(Ok)
    }

    async fn find_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<(User, PasswordHash)>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(StoredUser, "sql/user/find_by_email.sql", email.as_ref())
            .fetch_optional(&mut *connection)
            .await?
            .map(StoredUser::into_domain_tuple)
            .pipe(Ok)
    }
}
