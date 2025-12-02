use domain::client::Client;
use lib::{
    infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper, uuid::Uuid,
};
use sqlx::FromRow;

use crate::entity::client::gender::StoredClientGender;

pub mod gender;

#[derive(Mapper, FromRow, Debug)]
#[mapper(ty = Client, into)]
pub struct StoredClient {
    pub id: Uuid,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub login: String,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub age: i32,
    pub gender: StoredClientGender,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub location: String,
}
