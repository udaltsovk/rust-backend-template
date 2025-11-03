use domain::client::{Client, UpsertClient};
use lib::{
    domain::{
        DomainType as _, into_validators, validation::error::ValidationErrors,
    },
    presentation::api::rest::model::ParseableJson,
};
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
        let (errors, (login, age, location)) =
            into_validators!(self.login, self.age, self.location);

        errors.into_result(|ok| UpsertClient {
            id: self.id.into(),
            login: login.validated(ok),
            age: age.validated(ok),
            gender: self.gender.into(),
            location: location.validated(ok),
        })
    }
}
