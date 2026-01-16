use lib::domain::Id;

use crate::user::{
    avatar_url::UserAvatarUrl, email::UserEmail, name::UserName,
    password::UserPassword, surname::UserSurname,
    target_settings::UserTargetSettings,
};

pub mod avatar_url;
mod constraints;
pub mod email;
pub mod name;
pub mod password;
pub mod surname;
pub mod target_settings;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct User {
    pub id: Id<Self>,
    pub name: UserName,
    pub surname: UserSurname,
    pub email: UserEmail,
    pub avatar_url: Option<UserAvatarUrl>,
    pub target_settings: UserTargetSettings,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateUser {
    pub name: UserName,
    pub surname: UserSurname,
    pub email: UserEmail,
    pub password: UserPassword,
    pub avatar_url: Option<UserAvatarUrl>,
    pub target_settings: UserTargetSettings,
}
