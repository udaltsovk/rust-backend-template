use kernel::domain::client::{
    Client, age::ClientAge, location::ClientLocation, login::ClientLogin,
};
use sqlx::FromRow;
use uuid::Uuid;

use crate::entity::{DomainTypeFromDb, client::gender::StoredClientGender};

pub(crate) mod gender;

#[derive(FromRow, Debug)]
pub struct StoredClient {
    pub id: Uuid,
    pub login: String,
    pub age: i32,
    pub gender: StoredClientGender,
    pub location: String,
}
impl From<StoredClient> for Client {
    fn from(c: StoredClient) -> Self {
        Self {
            id: c.id.into(),
            login: ClientLogin::safe_parse(c.login),
            age: ClientAge::safe_parse(c.age),
            gender: c.gender.into(),
            location: ClientLocation::safe_parse(c.location),
        }
    }
}
