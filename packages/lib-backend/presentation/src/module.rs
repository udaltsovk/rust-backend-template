#[macro_export]
macro_rules! usecase_impl_type {
    () => {
        pub type UseCaseImpl<M, T> = application::usecase::UseCase<
            <M as ModulesExt>::RepositoriesModule,
            <M as ModulesExt>::ServicesModule,
            T,
        >;
    };
}
