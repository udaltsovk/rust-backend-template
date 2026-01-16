use lib::domain::Id;

use crate::{
    email::Email, password::Password, session::entity::SessionEntity,
    user::User,
};

pub mod entity;

#[derive(PartialEq, Eq)]
pub struct Session {
    pub id: Id<Self>,
    pub entity: SessionEntity,
}

impl Session {
    // one hour
    pub const LIFETIME: usize = 60 * 60;

    #[must_use]
    pub fn new_for_user(id: Id<User>) -> Self {
        Self {
            id: Id::generate(),
            entity: SessionEntity::User(id),
        }
    }
}

pub struct CreateSession {
    pub email: Email,
    pub password: Password,
}
