use std::marker::PhantomData;

use crate::Db;

mod client;

#[derive(Clone)]
pub struct DatabaseRepositoryImpl<T: Send + Sync> {
    db: Db,
    _entity: PhantomData<T>,
}
impl<T: Send + Sync> DatabaseRepositoryImpl<T> {
    pub(crate) fn new(db: &Db) -> Self {
        Self {
            db: db.clone(),
            _entity: PhantomData,
        }
    }
}
