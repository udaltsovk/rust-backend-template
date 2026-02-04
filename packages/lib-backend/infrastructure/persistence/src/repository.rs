#[macro_export]
macro_rules! repository_impl_struct {
    ($name: ident, $manager: ty) => {
        $crate::pastey::paste! {
            #[$crate::derive_where(Clone)]
            #[expect(dead_code, reason = "generated struct fields may not be used in all contexts")]
            pub struct [< $name RepositoryImpl >]<T>
            where
                T: Send + Sync,
            {
                pool: mobc::Pool<$manager>,
                _entity: std::marker::PhantomData<T>,
            }

            impl<T> [< $name RepositoryImpl >]<T>
            where
                T: Send + Sync,
            {
                pub fn new(pool: &mobc::Pool<$manager>) -> Self {
                    Self {
                        pool: pool.clone(),
                        _entity: std::marker::PhantomData,
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use mobc_sqlx::{SqlxConnectionManager, mobc, sqlx::Postgres};
    use rstest::{fixture, rstest};
    use sqlx::postgres::PgConnectOptions;

    struct TestEntity;

    repository_impl_struct!(Test, SqlxConnectionManager<Postgres>);

    #[fixture]
    fn pool() -> mobc_sqlx::mobc::Pool<SqlxConnectionManager<Postgres>> {
        let options = PgConnectOptions::new()
            .username("user")
            .password("pass")
            .database("db");
        let manager = SqlxConnectionManager::new(options);
        mobc_sqlx::mobc::Pool::builder().build(manager)
    }

    #[rstest]
    #[tokio::test]
    async fn repository_impl(
        pool: mobc_sqlx::mobc::Pool<SqlxConnectionManager<Postgres>>,
    ) {
        let _repo = TestRepositoryImpl::<TestEntity>::new(&pool);
    }

    #[rstest]
    #[tokio::test]
    #[expect(clippy::redundant_clone, reason = "Testing Clone implementation")]
    async fn repository_impl_clone(
        pool: mobc_sqlx::mobc::Pool<SqlxConnectionManager<Postgres>>,
    ) {
        let repo = TestRepositoryImpl::<TestEntity>::new(&pool);
        let _cloned = repo.clone();
    }
}
