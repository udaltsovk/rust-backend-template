use domain::client::gender::ClientGender;
use lib::model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = ClientGender, from, into)]
#[sqlx(type_name = "client_gender", rename_all = "lowercase")]
pub enum StoredClientGender {
    Male,
    Female,
}
