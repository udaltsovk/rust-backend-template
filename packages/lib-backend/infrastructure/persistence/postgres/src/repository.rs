#[macro_export]
macro_rules! postgres_repository_impl_struct {
    () => {
        #[$crate::derive_where(Clone)]
        pub struct PostgresRepositoryImpl<T>
        where
            T: Send + Sync,
        {
            db: lib::infrastructure::persistence::postgres::Postgres,
            _entity: std::marker::PhantomData<T>,
        }

        impl<T> PostgresRepositoryImpl<T>
        where
            T: Send + Sync,
        {
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
