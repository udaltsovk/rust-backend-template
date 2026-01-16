use domain::user::target_settings::UserTargetSettings;
use lib::{
    domain::{into_validators, validation::error::ValidationErrors},
    model_mapper::Mapper,
    presentation::api::rest::model::ParseableJson,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Mapper, Deserialize, Serialize, ToSchema, Debug)]
#[mapper(ty = UserTargetSettings, from)]
pub struct JsonUserTargetSettings {
    ///
    pub age: i64,

    ///
    pub country: String,
}

impl ParseableJson<UserTargetSettings> for JsonUserTargetSettings {
    fn parse(self) -> Result<UserTargetSettings, ValidationErrors> {
        let (errors, (age, country)) = into_validators!(self.age, self.country);

        errors.into_result(|ok| UserTargetSettings {
            age: age.validated(ok),
            country: country.validated(ok),
        })
    }
}
