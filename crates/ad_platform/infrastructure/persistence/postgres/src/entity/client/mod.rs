use kernel::domain::client::Client;
use sqlx::FromRow;
use uuid::Uuid;

use crate::entity::client::gender::StoredClientGender;

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
            login: c.login,
            age: c.age.try_into().unwrap_or(0),
            gender: c.gender.into(),
            location: c.location,
        }
    }
}
