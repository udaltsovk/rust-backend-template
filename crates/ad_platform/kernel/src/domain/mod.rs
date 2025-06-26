use std::marker::PhantomData;

use uuid::Uuid;

pub mod client;

#[derive(Debug)]
pub struct Id<T> {
    pub value: Uuid,
    _entity: PhantomData<T>,
}
impl<T> Id<T> {
    pub fn new(value: Uuid) -> Self {
        Self {
            value,
            _entity: PhantomData,
        }
    }

    pub fn generate() -> Self {
        Self::new(Uuid::now_v7())
    }
}
impl<T> From<Uuid> for Id<T> {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}
