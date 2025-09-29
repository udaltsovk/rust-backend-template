use domain::client::gender::ClientGender;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum JsonClientGender {
    Male,
    Female,
}

impl From<JsonClientGender> for ClientGender {
    fn from(g: JsonClientGender) -> Self {
        use JsonClientGender as G;
        match g {
            G::Male => Self::Male,
            G::Female => Self::Female,
        }
    }
}

impl From<ClientGender> for JsonClientGender {
    fn from(g: ClientGender) -> Self {
        use ClientGender as G;
        match g {
            G::Male => Self::Male,
            G::Female => Self::Female,
        }
    }
}
