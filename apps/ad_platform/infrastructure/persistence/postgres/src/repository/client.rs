use application::repository::client::ClientRepository;
use async_trait::async_trait;
use domain::client::{Client, UpsertClient};
use lib::{
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
};
use sqlx::query_file_as;

use crate::{
    entity::client::{StoredClient, gender::StoredClientGender},
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all("PostgresClientRepository")]
impl ClientRepository for PostgresRepositoryImpl<Client> {
    type AdapterError = PostgresAdapterError;

    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, Self::AdapterError> {
        if source.is_empty() {
            return Ok(vec![]);
        }

        let mut transaction = self.db.begin().await?;

        let mut clients = vec![];

        for client in source {
            let gender: StoredClientGender = client.gender.clone().into();
            let age: i32 = client.age.cloned_inner().into();

            let result = query_file_as!(
                StoredClient,
                "./sql/client/upsert.sql",
                client.id.value,
                client.login.cloned_inner(),
                age,
                gender as StoredClientGender,
                client.location.cloned_inner()
            )
            .fetch_one(&mut *transaction)
            .await
            .map(Client::from)?;

            clients.push(result);
        }

        transaction.commit().await?;
        let result = clients;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, Self::AdapterError> {
        let result = query_file_as!(
            StoredClient,
            "./sql/client/find_by_id.sql",
            id.value
        )
        .fetch_optional(&*self.db)
        .await?
        .map(Client::from);

        Ok(result)
    }
}
