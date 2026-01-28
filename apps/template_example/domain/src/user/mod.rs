use lib::domain::{
    Id,
    validation::{Nullable, Optional},
};

use crate::{
    email::Email,
    password::{Password, PasswordHash},
    user::{
        avatar_url::UserAvatarUrl, name::UserName, surname::UserSurname,
        target_settings::UserTargetSettings,
    },
};

pub mod avatar_url;
mod constraints;
pub mod name;
pub mod surname;
pub mod target_settings;

#[derive(Debug)]
pub struct User {
    pub id: Id<Self>,
    pub name: UserName,
    pub surname: UserSurname,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub avatar_url: Option<UserAvatarUrl>,
    pub target_settings: UserTargetSettings,
}

#[derive(Debug)]
pub struct CreateUser {
    pub name: UserName,
    pub surname: UserSurname,
    pub email: Email,
    pub password: Password,
    pub avatar_url: Optional<Nullable<UserAvatarUrl>>,
    pub target_settings: UserTargetSettings,
}
