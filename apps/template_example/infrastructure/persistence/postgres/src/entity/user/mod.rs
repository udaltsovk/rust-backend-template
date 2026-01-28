use domain::user::User;
use lib::{
    infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper, uuid::Uuid,
};
use sqlx::FromRow;

use crate::entity::user::target_settings::StoredUserTargetSettings;

pub mod target_settings;

#[derive(Mapper, FromRow)]
#[cfg_attr(debug_assertions, derive(Debug))]
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
