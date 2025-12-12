#[macro_export]
macro_rules! repository_impl_struct {
    ($name: ident, $manager: ty) => {
        $crate::pastey::paste! {
            #[$crate::derive_where(Clone)]
            #[expect(dead_code, reason = "generated struct fields might not be used in all contexts")]
            pub struct [< $name RepositoryImpl >]<T>
            where
                T: Send + Sync,
            {
                pool: mobc_sqlx::mobc::Pool<$manager>,
                _entity: std::marker::PhantomData<T>,
            }

            impl<T> [< $name RepositoryImpl >]<T>
            where
                T: Send + Sync,
            {
                pub fn new(pool: &mobc_sqlx::mobc::Pool<$manager>) -> Self {
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
    use mobc_sqlx::{SqlxConnectionManager, sqlx::Postgres};

    struct TestEntity;

    repository_impl_struct!(Test, SqlxConnectionManager<Postgres>);

    #[tokio::test]
    async fn repository_impl() {
        let manager = SqlxConnectionManager::<Postgres>::new(
            "postgres://user:pass@localhost:5432/db",
        );
        let pool = mobc_sqlx::mobc::Pool::builder().build(manager);
        let _repo = TestRepositoryImpl::<TestEntity>::new(&pool);
    }

    #[tokio::test]
    #[expect(clippy::redundant_clone, reason = "Testing Clone implementation")]
    async fn repository_impl_clone() {
        let manager = SqlxConnectionManager::<Postgres>::new(
            "postgres://user:pass@localhost:5432/db",
        );
        let pool = mobc_sqlx::mobc::Pool::builder().build(manager);
        let repo = TestRepositoryImpl::<TestEntity>::new(&pool);
        let _cloned = repo.clone();
    }
}
