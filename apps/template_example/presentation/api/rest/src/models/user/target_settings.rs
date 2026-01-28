use domain::user::target_settings::UserTargetSettings;
use lib::{
    domain::{into_validators, validation::error::ValidationResult},
    model_mapper::Mapper,
    presentation::api::rest::{UserInput, model::Parseable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Таргет настройки пользователя
#[derive(Mapper, Deserialize, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserTargetSettings, from)]
pub struct JsonUserTargetSettings {
    /// Возраст пользователя
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

    /// Страна пользователя в формате ISO 3166-1 alpha-2, регистр может быть разным. Страна с данным кодом должна обязательно существовать.
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

impl Parseable<UserTargetSettings> for JsonUserTargetSettings {
    const FIELD: &str = "target";

    fn parse(self) -> ValidationResult<UserTargetSettings> {
        let (errors, (age, country)) = into_validators!(self.age, self.country);

        errors.into_result(|ok| UserTargetSettings {
            age: age.validated(ok),
            country: country.validated(ok),
        })
    }
}
