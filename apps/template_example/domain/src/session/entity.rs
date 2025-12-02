use lib::{domain::Id, uuid::Uuid};

use crate::client::Client;

#[derive(Clone)]
pub enum SessionEntity {
    Client(Id<Client>),
}

impl From<SessionEntity> for Uuid {
    fn from(se: SessionEntity) -> Self {
        use SessionEntity as SE;
        match se {
            SE::Client(id) => id.value,
        }
    }
}
