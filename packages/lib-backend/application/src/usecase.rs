#[macro_export]
macro_rules! usecase_struct {
    ($repos_ext: ident, $services_ext: ident) => {
        #[$crate::derive_where(Clone)]
        pub struct UseCase<T> {
            repositories: std::sync::Arc<Box<dyn $repos_ext>>,
            services: std::sync::Arc<Box<dyn $services_ext>>,
            _entity: std::marker::PhantomData<T>,
        }

        impl<T> UseCase<T> {
            pub fn new<R, S>(repositories: &R, services: &S) -> Self
            where
                R: $repos_ext + Clone + 'static,
                S: $services_ext + Clone + 'static,
            {
                Self {
                    repositories: std::sync::Arc::new(Box::new(
                        repositories.clone(),
                    )),
                    services: std::sync::Arc::new(Box::new(services.clone())),
                    _entity: std::marker::PhantomData,
                }
            }
        }
    };
}
