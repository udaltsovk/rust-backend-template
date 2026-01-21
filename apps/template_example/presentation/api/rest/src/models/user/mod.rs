use domain::user::{CreateUser, User};
use lib::{
    domain::{
        into_option_validators, into_validators,
        validation::error::ValidationErrors,
    },
    model_mapper::Mapper,
    presentation::api::rest::{into_nested_validators, model::Parseable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::user::target_settings::JsonUserTargetSettings;

mod target_settings;

#[derive(Mapper, Serialize, ToSchema, Debug)]
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

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateJsonUser {
    ///
    #[schema(required, min_length = 1, max_length = 100, examples("Мария"))]
    name: Option<String>,

    ///
    #[schema(required, min_length = 1, max_length = 120, examples("Федотова"))]
    surname: Option<String>,

    ///
    #[schema(
        required,
        format = IdnEmail,
        min_length = 8,
        max_length = 120,
        examples(
            "cu_fan@edu.hse.ru"
        )
    )]
    email: Option<String>,

    ///
    #[schema(
        required,
        format = Password,
        min_length = 8,
        max_length = 60,
        pattern = r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[@$!%*?&])[A-Za-z\\d@$!%*?&]{8,}$",
        examples(
            "HardPa$$w0rd!iamthewinner"
        )
    )]
    password: Option<String>,

    /// Ссылка на фото пользователя
    #[schema(
        format = Uri,
        max_length = 350,
        examples(
            "https://cdn2.thecatapi.com/images/3lo.jpg"
        )
    )]
    avatar_url: Option<String>,

    ///
    #[schema(required)]
    other: Option<JsonUserTargetSettings>,
}

impl Parseable<CreateUser> for CreateJsonUser {
    const FIELD: &str = "user";

    fn parse(self) -> Result<CreateUser, ValidationErrors> {
        let (mut errors, (name, surname, email, password)) = into_validators!(
            self.name,
            self.surname,
            self.email,
            self.password
        );

        let (option_errors, avatar_url) =
            into_option_validators!(self.avatar_url);

        errors.extend(option_errors);

        let (nested_errors, target_settings) =
            into_nested_validators!(self.other);

        errors.extend(nested_errors);

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
