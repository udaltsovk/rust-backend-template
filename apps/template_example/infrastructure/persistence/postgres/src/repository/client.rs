use application::repository::client::ClientRepository;
use domain::client::{Client, UpsertClient};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::{Conv as _, Pipe as _},
};
use sqlx::{Acquire as _, query_file_as};

use crate::{
    entity::client::{StoredClient, gender::StoredClientGender},
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl ClientRepository for PostgresRepositoryImpl<Client> {
    type AdapterError = PostgresAdapterError;

    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, Self::AdapterError> {
        if source.is_empty() {
            return Ok(vec![]);
        }

        let mut connection = self.pool.get().await?;
        let mut transaction = connection.begin().await?;

        let mut clients = vec![];

        for client in source {
            query_file_as!(
                StoredClient,
                "./sql/client/upsert.sql",
                client.id.value,
                client.login.cloned_inner(),
                client.age.cloned_inner().conv::<i32>(),
                client.gender.clone().conv::<StoredClientGender>()
                    as StoredClientGender,
                client.location.cloned_inner()
            )
            .fetch_one(&mut *transaction)
            .await
            .map(Client::from)?
            .pipe(|c| clients.push(c));
        }

        transaction.commit().await?;

        Ok(clients)
    }

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(StoredClient, "./sql/client/find_by_id.sql", id.value)
            .fetch_optional(&mut *connection)
            .await?
            .map(Client::from)
            .pipe(Ok)
    }
}
