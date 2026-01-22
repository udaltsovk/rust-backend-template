use derive_more::From;
use domain::session::CreateSession;
use lib::{
    domain::{into_validators, validation::error::ValidationResult},
    presentation::api::rest::{UserInput, model::Parseable},
};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(From, Serialize, ToSchema, ToResponse)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JsonUserSession {
    /// Токен доступа для авторизованных запросов. После успешной аутентификации СТАРЫЕ ТОКЕНЫ ПЕРЕСТАЮТ РАБОТАТЬ.
    #[schema(examples(
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Njg1ODE1MjMsImlhdCI6MTc2ODU3NzkyMywic3ViIjoiMDE5YmM2ZGEtM2Y0Yi03MTcwLTg4NDItMDMzY2MwZjA0ZTUwIiwicm9sZSI6IlVTRVIiLCJqdGkiOiIwMTliYzc3NS03ODI3LTc2NDEtODdmYy00YzlkYTc5ODlkZGEifQ.MCS_MLo8g4CciJ--qxXfBgPflVrhmmbdd4J9zqq69Sk"
    ))]
    token: String,
}

#[derive(Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateJsonSession {
    ///
    #[serde(default)]
    email: UserInput<String>,
    ///
    #[serde(default)]
    password: UserInput<String>,
}

impl Parseable<CreateSession> for CreateJsonSession {
    const FIELD: &str = "credentials";

    fn parse(self) -> ValidationResult<CreateSession> {
        let (errors, (email, password)) =
            into_validators!(self.email, self.password);

        errors.into_result(|ok| CreateSession {
            email: email.validated(ok),
            password: password.validated(ok),
        })
    }
}
