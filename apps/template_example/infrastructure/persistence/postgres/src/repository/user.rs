use application::repository::user::UserRepository;
use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User},
};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::{Conv as _, Pipe as _},
};
use sqlx::{Acquire as _, query_file_as};

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
        let mut transaction = connection.begin().await?;

        let user = query_file_as!(
            StoredUser,
            "sql/user/create.sql",
            id.value,
            source.name.cloned_inner(),
            source.surname.cloned_inner(),
            source.email.cloned_inner(),
            password_hash,
            source.avatar_url.map(|v| v.cloned_inner()),
            source.target_settings.conv::<StoredUserTargetSettings>()
                as StoredUserTargetSettings,
        )
        .fetch_one(&mut *transaction)
        .await?
        .conv::<User>();

        transaction.commit().await?;

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
