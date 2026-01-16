use crate::user::target_settings::{
    age::UserTargetSettingsAge, country::UserTargetSettingsCountry,
};

pub mod age;
pub mod country;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserTargetSettings {
    pub age: UserTargetSettingsAge,
    pub country: UserTargetSettingsCountry,
}
