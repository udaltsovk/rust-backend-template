use lib::presentation::api::rest::{
    into_validators,
    validation::{UserInput, parseable::Parseable, validator::ValidatorResult},
};
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::features::user::domain::target_settings::UserTargetSettings;

#[derive(Mapper, Deserialize, Serialize, ToSchema, Default, Debug)]
#[mapper(ty = UserTargetSettings, from)]
pub struct UserTargetSettingsDto {
    #[schema(
        required,
        format = UInt8,
        minimum = 0,
        maximum = 100,
        examples(
            13
        ),
        value_type = i64,
    )]
    #[mapper(with = UserInput::from_domain)]
    #[serde(default)]
    pub age: UserInput<i64>,

    #[schema(
        required,
        format = "iso-3166-alpha-2",
        examples("ru"),
        value_type = String,
    )]
    #[mapper(with = UserInput::from_domain)]
    #[serde(default)]
    pub country: UserInput<String>,
}

impl Parseable<UserTargetSettings> for UserTargetSettingsDto {
    fn parse(self) -> ValidatorResult<UserTargetSettings> {
        let (errors, (age, country)) = into_validators!(
            field!(self.age, required, "age"),
            field!(self.country, required, "contry")
        );

        errors.into_result(|ok| UserTargetSettings {
            age: age.validated(ok),
            country: country.validated(ok),
        })
    }
}
