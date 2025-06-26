use garde::Validate;
use kernel::domain::client::{Client, UpsertClient};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::client::gender::JsonClientGender;

mod gender;

#[derive(Serialize, ToSchema, Debug)]
pub struct JsonClient {
    pub id: Uuid,
    pub login: String,
    pub age: u16,
    pub gender: JsonClientGender,
    pub location: String,
}
impl From<Client> for JsonClient {
    fn from(c: Client) -> Self {
        Self {
            id: c.id.value,
            login: c.login,
            age: c.age,
            gender: c.gender.into(),
            location: c.location,
        }
    }
}
#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct UpsertJsonClient {
    #[garde(skip)]
    ///
    pub id: Uuid,

    #[garde(ascii, length(min = 3, max = 32))]
    ///
    pub login: String,

    #[garde(range(min = 0, max = 255))]
    ///
    pub age: i64,

    #[garde(skip)]
    ///
    pub gender: JsonClientGender,

    #[garde(length(min = 10, max = 100))]
    ///
    pub location: String,
}
impl From<UpsertJsonClient> for UpsertClient {
    fn from(uc: UpsertJsonClient) -> UpsertClient {
        Self {
            id: uc.id.into(),
            login: uc.login,
            age: uc
                .age
                .try_into()
                .expect("this shouldn't happen due to validation"),
            gender: uc.gender.into(),
            location: uc.location,
        }
    }
}
