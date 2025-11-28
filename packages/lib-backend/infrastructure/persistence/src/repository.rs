#[macro_export]
macro_rules! repository_impl_struct {
    ($name: ident, $manager: ty) => {
        $crate::pastey::paste! {
            #[$crate::derive_where(Clone)]
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
