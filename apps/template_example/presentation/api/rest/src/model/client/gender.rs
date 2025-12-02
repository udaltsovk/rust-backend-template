use domain::client::gender::ClientGender;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Mapper, Deserialize, Serialize, ToSchema, Debug)]
#[mapper(ty = ClientGender, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum JsonClientGender {
    Male,
    Female,
}
