use domain::user::target_settings::UserTargetSettings;
use lib::{
    domain::{into_validators, validation::error::ValidationErrors},
    model_mapper::Mapper,
    presentation::api::rest::model::Parseable,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Таргет настройки пользователя
#[derive(Mapper, Deserialize, Serialize, ToSchema, Debug)]
#[mapper(ty = UserTargetSettings, from)]
pub struct JsonUserTargetSettings {
    /// Возраст пользователя
    #[schema(
        format = UInt8,
        minimum = 0,
        maximum = 100,
        examples(
            13
        )
    )]
    pub age: i64,

    /// Страна пользователя в формате ISO 3166-1 alpha-2, регистр может быть разным. Страна с данным кодом должна обязательно существовать.
    #[schema(format = "iso-3166-alpha-2", examples("ru"))]
    pub country: String,
}

impl Parseable<UserTargetSettings> for JsonUserTargetSettings {
    fn parse(self) -> Result<UserTargetSettings, ValidationErrors> {
        let (errors, (age, country)) = into_validators!(self.age, self.country);

        errors.into_result(|ok| UserTargetSettings {
            age: age.validated(ok),
            country: country.validated(ok),
        })
    }
}
