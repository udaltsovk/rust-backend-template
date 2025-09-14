use lib::domain::Id;

use crate::session::entity::SessionEntity;

pub mod entity;

pub struct Session {
    pub id: Id<Self>,
    pub entity: SessionEntity,
}
