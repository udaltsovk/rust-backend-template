pub mod age;
pub mod country;

use self::{
    age::UserTargetSettingsAge,
    country::UserTargetSettingsCountry,
};

#[derive(Debug)]
pub struct UserTargetSettings {
    pub age: UserTargetSettingsAge,
    pub country: UserTargetSettingsCountry,
}
