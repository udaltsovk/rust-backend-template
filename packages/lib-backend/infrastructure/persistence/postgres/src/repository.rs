#[macro_export]
macro_rules! postgres_repository_impl_struct {
    () => {
        #[derive(Clone)]
        pub struct PostgresRepositoryImpl<T: Send + Sync> {
            db: lib::infrastructure::persistence::postgres::Postgres,
            _entity: std::marker::PhantomData<T>,
        }

        impl<T: Send + Sync> PostgresRepositoryImpl<T> {
            pub fn new(
                db: &lib::infrastructure::persistence::postgres::Postgres,
            ) -> Self {
                Self {
                    db: db.clone(),
                    _entity: std::marker::PhantomData,
                }
            }
        }
    };
}
