use kernel::domain::{
    DomainType as _,
    client::{
        Client, UpsertClient, age::ClientAge, location::ClientLocation,
        login::ClientLogin,
    },
    error::ValidationErrors,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::{ParseableJson, client::gender::JsonClientGender};

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
            login: c.login.into_inner(),
            age: c.age.into_inner(),
            gender: c.gender.into(),
            location: c.location.into_inner(),
        }
    }
}
#[derive(Deserialize, ToSchema, Debug)]
pub struct UpsertJsonClient {
    ///
    pub id: Uuid,

    ///
    pub login: String,

    ///
    pub age: i32,

    ///
    pub gender: JsonClientGender,

    ///
    pub location: String,
}
impl ParseableJson<UpsertClient> for UpsertJsonClient {
    fn parse(self) -> Result<UpsertClient, ValidationErrors> {
        let errors = vec![];
        let (errors, login_fn) = ClientLogin::parse(self.login, errors);
        let (errors, age_fn) = ClientAge::parse(self.age, errors);
        let (errors, location_fn) =
            ClientLocation::parse(self.location, errors);
        errors
            .is_empty()
            .then_some(UpsertClient {
                id: self.id.into(),
                login: login_fn(),
                age: age_fn(),
                gender: self.gender.into(),
                location: location_fn(),
            })
            .ok_or_else(|| errors.into())
    }
}
