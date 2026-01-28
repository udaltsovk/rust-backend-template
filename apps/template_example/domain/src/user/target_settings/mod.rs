use crate::user::target_settings::{
    age::UserTargetSettingsAge, country::UserTargetSettingsCountry,
};

pub mod age;
pub mod country;

#[derive(Debug)]
pub struct UserTargetSettings {
    pub age: UserTargetSettingsAge,
    pub country: UserTargetSettingsCountry,
}
