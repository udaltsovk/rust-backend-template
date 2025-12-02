use domain::client::{Client, UpsertClient};
use lib::{
    domain::{into_validators, validation::error::ValidationErrors},
    model_mapper::Mapper,
    presentation::api::rest::model::ParseableJson,
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::client::gender::JsonClientGender;

mod gender;

#[derive(Mapper, Serialize, ToSchema, Debug)]
#[mapper(ty = Client, from)]
pub struct JsonClient {
    pub id: Uuid,
    pub login: String,
    pub age: u16,
    pub gender: JsonClientGender,
    pub location: String,
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
