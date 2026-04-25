use lib::{
    infrastructure::persistence::entity::DomainTypeFromDb,
    uuid::Uuid,
};
use model_mapper::Mapper;
use sqlx::FromRow;

pub mod target_settings;

use self::target_settings::StoredUserTargetSettings;
use crate::features::user::domain::User;

#[derive(Mapper, FromRow, Debug)]
#[mapper(derive(ty = User, into))]
pub struct StoredUser {
    pub id: Uuid,
    #[mapper(
        when(ty = User, into_with = DomainTypeFromDb::into_domain),
    )]
    pub name: String,
    #[mapper(
        when(ty = User, into_with = DomainTypeFromDb::into_domain),
    )]
    pub surname: String,
    #[mapper(
        when(ty = User, into_with = DomainTypeFromDb::into_domain),
    )]
    pub email: String,
    pub password_hash: String,
    #[mapper(
        when(ty = User, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub avatar_url: Option<String>,
    pub target_settings: StoredUserTargetSettings,
}
