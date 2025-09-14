use std::marker::PhantomData;

use lib::infrastructure::persistence::postgres::Postgres;

mod client;

#[derive(Clone)]
pub struct PostgresRepositoryImpl<T: Send + Sync> {
    db: Postgres,
    _entity: PhantomData<T>,
}
impl<T: Send + Sync> PostgresRepositoryImpl<T> {
    pub fn new(db: &Postgres) -> Self {
        Self {
            db: db.clone(),
            _entity: PhantomData,
        }
    }
}
