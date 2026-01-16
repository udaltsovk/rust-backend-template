use domain::user::User;
use lib::{
    infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper, uuid::Uuid,
};
use sqlx::FromRow;

use crate::entity::user::target_settings::StoredUserTargetSettings;

pub mod target_settings;

#[derive(Mapper, FromRow, Debug)]
#[mapper(ty = User, into)]
pub struct StoredUser {
    pub id: Uuid,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub name: String,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub surname: String,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub email: String,
    #[mapper(skip)]
    pub password_hash: String,
    #[mapper(opt(into_with = DomainTypeFromDb::into_domain))]
    pub avatar_url: Option<String>,
    pub target_settings: StoredUserTargetSettings,
}
