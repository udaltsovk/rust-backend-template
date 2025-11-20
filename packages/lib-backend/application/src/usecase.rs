#[macro_export]
macro_rules! usecase_struct {
    ($repos_ext: ident, $services_ext: ident) => {
        #[$crate::derive_where(Clone)]
        pub struct UseCase<R, S, T>
        where
            R: $repos_ext + Send + Sync,
            S: $services_ext + Send + Sync,
        {
            repositories: R,
            services: S,
            _entity: std::marker::PhantomData<T>,
        }

        impl<R, S, T> UseCase<R, S, T>
        where
            R: $repos_ext + Send + Sync,
            S: $services_ext + Send + Sync,
        {
            pub fn new(repositories: R, services: S) -> Self {
                Self {
                    repositories,
                    services,
                    _entity: std::marker::PhantomData,
                }
            }
        }
    };
}
