use lib::{domain::Id, uuid::Uuid};

use crate::user::User;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SessionEntity {
    User(Id<User>),
}

impl From<SessionEntity> for Uuid {
    fn from(entity: SessionEntity) -> Self {
        use SessionEntity as SE;
        match entity {
            SE::User(id) => id.value,
        }
    }
}

impl From<&User> for SessionEntity {
    fn from(user: &User) -> Self {
        Self::User(user.id)
    }
}

impl SessionEntity {
    #[must_use]
    pub const fn as_tuple(&self) -> (&'static str, Uuid) {
        match self {
            Self::User(id) => ("user", id.value),
        }
    }
}
