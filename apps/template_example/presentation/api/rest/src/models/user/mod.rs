use domain::user::{CreateUser, User};
use lib::{
    model_mapper::Mapper,
    presentation::api::rest::{
        into_validators,
        validation::{
            UserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
};
use redact::Secret;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::user::target_settings::JsonUserTargetSettings;

pub mod target_settings;

#[derive(Mapper, Serialize, ToSchema, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = User, from, ignore_extra)]
pub struct JsonUser {
    ///
    name: String,

    ///
    surname: String,

    ///
    email: String,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[mapper(opt)]
    avatar_url: Option<String>,

    ///
    #[mapper(rename = target_settings)]
    other: JsonUserTargetSettings,
}

#[derive(Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateJsonUser {
    #[schema(
        required,
        min_length = 1,
        max_length = 100,
        examples("Мария"),
        value_type = String,
    )]
    #[serde(default)]
    name: UserInput<String>,

    ///
    #[schema(
        required,
        min_length = 1,
        max_length = 120,
        examples("Федотова"),
        value_type = String,
    )]
    #[serde(default)]
    surname: UserInput<String>,

    ///
    #[schema(
        required,
        format = IdnEmail,
        min_length = 8,
        max_length = 120,
        examples(
            "cu_fan@edu.hse.ru"
        ),
        value_type = String,
    )]
    #[serde(default)]
    email: UserInput<String>,

    ///
    #[schema(
        required,
        format = Password,
        min_length = 8,
        max_length = 60,
        pattern = r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[@$!%*?&])[A-Za-z\\d@$!%*?&]{8,}$",
        examples(
            "HardPa$$w0rd!iamthewinner"
        ),
        value_type = String,
    )]
    #[serde(default)]
    password: UserInput<Secret<String>>,

    /// Ссылка на фото пользователя
    #[schema(
        format = Uri,
        max_length = 350,
        examples(
            "https://cdn2.thecatapi.com/images/3lo.jpg"
        ),
        value_type = String,
    )]
    #[serde(default)]
    avatar_url: UserInput<String>,

    ///
    #[schema(required, value_type = JsonUserTargetSettings)]
    #[serde(default)]
    other: UserInput<JsonUserTargetSettings>,
}

impl Parseable<CreateUser> for CreateJsonUser {
    fn parse(self) -> ValidatorResult<CreateUser> {
        let (
            errors,
            (name, surname, email, password, avatar_url, target_settings),
        ) = into_validators!(
            field!(self.name, required, "name"),
            field!(self.surname, required, "surname"),
            field!(self.email, required, "email"),
            field!(self.password, required, "password"),
            field!(self.avatar_url, optional, "avatar_url"),
            field!(self.other, nested, "other")
        );

        errors.into_result(|ok| CreateUser {
            name: name.validated(ok),
            surname: surname.validated(ok),
            email: email.validated(ok),
            password: password.validated(ok),
            avatar_url: avatar_url.validated(ok),
            target_settings: target_settings.validated(ok),
        })
    }
}
