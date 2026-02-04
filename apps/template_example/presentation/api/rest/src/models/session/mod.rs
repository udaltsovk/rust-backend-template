use derive_more::From;
use domain::session::CreateSession;
use lib::presentation::api::rest::{
    into_validators,
    validation::{UserInput, parseable::Parseable, validator::ValidatorResult},
};
use redact::{Secret, expose_secret};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(From, Serialize, ToSchema, ToResponse)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JsonUserSession {
    /// Токен доступа для авторизованных запросов. После успешной аутентификации СТАРЫЕ ТОКЕНЫ ПЕРЕСТАЮТ РАБОТАТЬ.
    #[schema(
        examples(
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Njg1ODE1MjMsImlhdCI6MTc2ODU3NzkyMywic3ViIjoiMDE5YmM2ZGEtM2Y0Yi03MTcwLTg4NDItMDMzY2MwZjA0ZTUwIiwicm9sZSI6IlVTRVIiLCJqdGkiOiIwMTliYzc3NS03ODI3LTc2NDEtODdmYy00YzlkYTc5ODlkZGEifQ.MCS_MLo8g4CciJ--qxXfBgPflVrhmmbdd4J9zqq69Sk"
        ),
        value_type = String,
    )]
    #[serde(serialize_with = "expose_secret")]
    token: Secret<String>,
}

#[derive(Deserialize, ToSchema, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateJsonSession {
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
}

impl Parseable<CreateSession> for CreateJsonSession {
    fn parse(self) -> ValidatorResult<CreateSession> {
        let (errors, (email, password)) = into_validators!(
            field!(self.email, required, "email"),
            field!(self.password, required, "password")
        );

        errors.into_result(|ok| CreateSession {
            email: email.validated(ok),
            password: password.validated(ok),
        })
    }
}
