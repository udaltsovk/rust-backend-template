use domain::user::target_settings::UserTargetSettings;
use lib::{
    domain::DomainType as _,
    infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper,
};
use sqlx::{FromRow, Type};

#[derive(Mapper, FromRow, Type)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserTargetSettings, from, into)]
#[sqlx(type_name = "user_target_settings")]
pub struct StoredUserTargetSettings {
    #[mapper(
        from_with = age.into_inner().into(),
        into_with = DomainTypeFromDb::into_domain
    )]
    pub age: i16,
    #[mapper(into_with = DomainTypeFromDb::into_domain)]
    pub country: String,
}
