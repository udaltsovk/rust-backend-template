use lib::kernel::domain::Id;

use crate::domain::session::entity::SessionEntity;

pub mod entity;

pub struct Session {
    pub id: Id<Self>,
    pub entity: SessionEntity,
}
