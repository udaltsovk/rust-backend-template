use kernel::domain::{
    DomainType as _,
    client::{Client, UpsertClient},
    error::ValidationErrors,
    validation::Validator,
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
        let mut errors = ValidationErrors::new();
        let login_fn = Validator::new(self.login, &mut errors).lazy();
        let age_fn = Validator::new(self.age, &mut errors).lazy();
        let location_fn = Validator::new(self.location, &mut errors).lazy();
        errors.into_result(|| UpsertClient {
            id: self.id.into(),
            login: login_fn(),
            age: age_fn(),
            gender: self.gender.into(),
            location: location_fn(),
        })
    }
}
