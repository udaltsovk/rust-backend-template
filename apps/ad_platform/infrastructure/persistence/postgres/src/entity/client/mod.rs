use domain::client::Client;
use lib::infrastructure::persistence::postgres::entity::DomainTypeFromDb as _;
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
            login: c.login.into_domain(),
            age: c.age.into_domain(),
            gender: c.gender.into(),
            location: c.location.into_domain(),
        }
    }
}
