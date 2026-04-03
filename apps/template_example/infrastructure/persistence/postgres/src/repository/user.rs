use application::repository::user::UserRepositoryImpl;
use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User},
};
use entrait::entrait;
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    domain::{DomainType, Id},
    infrastructure::persistence::{HasPoolExt as _, sqlx::SqlxPool},
    instrument_all,
    tap::{Conv as _, Pipe as _},
};
use sqlx::{Postgres, query_file_as};

use crate::{
    entity::user::{StoredUser, target_settings::StoredUserTargetSettings},
    repository::PostgresRepositoryImpl,
};

#[entrait]
#[async_trait]
#[instrument_all]
impl UserRepositoryImpl for PostgresRepositoryImpl {
    async fn create_user<App>(
        app: &App,
        id: Id<User>,
        source: CreateUser,
        password_hash: PasswordHash,
    ) -> Result<User>
    where
        App: Has<SqlxPool<Postgres>>,
    {
        let mut connection = app.get_connection().await?;

        let id = id.value;
        let name = source.name.into_inner();
        let surname = source.surname.into_inner();
        let email = source.email.into_inner();
        let password_hash = password_hash.0.expose_secret();
        let avatar_url = source.avatar_url.map(DomainType::into_inner);
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

    async fn find_user_by_id<App>(
        app: &App,
        id: Id<User>,
    ) -> Result<Option<User>>
    where
        App: Has<SqlxPool<Postgres>>,
    {
        let mut connection = app.get_connection().await?;

        query_file_as!(StoredUser, "sql/user/find_by_id.sql", id.value)
            .fetch_optional(&mut *connection)
            .await?
            .map(User::from)
            .pipe(Ok)
    }

    async fn find_user_by_email<App>(
        app: &App,
        email: &Email,
    ) -> Result<Option<User>>
    where
        App: Has<SqlxPool<Postgres>>,
    {
        let mut connection = app.get_connection().await?;

        query_file_as!(StoredUser, "sql/user/find_by_email.sql", email.as_ref())
            .fetch_optional(&mut *connection)
            .await?
            .map(User::from)
            .pipe(Ok)
    }
}
