use derive_more::From;
use domain::user::{CreateUser, User};
use lib::{
    domain::{
        into_option_validators, into_validators,
        validation::error::ValidationErrors,
    },
    model_mapper::Mapper,
    presentation::api::rest::{into_nested_validators, model::ParseableJson},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::user::target_settings::JsonUserTargetSettings;

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

#[derive(From, Serialize, ToSchema, Debug)]
pub struct JsonUserToken {
    ///
    token: String,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateJsonUser {
    ///
    name: String,
    ///
    surname: String,
    ///
    email: String,
    ///
    password: String,
    ///
    avatar_url: Option<String>,
    ///
    other: JsonUserTargetSettings,
}

impl ParseableJson<CreateUser> for CreateJsonUser {
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
