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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    mod application {
        pub mod usecase {
            pub struct UseCase<R, S, T>(std::marker::PhantomData<(R, S, T)>);
        }
    }

    pub trait ModulesExt {
        type RepositoriesModule;
        type ServicesModule;
    }

    pub struct TestModules;
    pub struct TestRepoModule;
    pub struct TestServiceModule;

    impl ModulesExt for TestModules {
        type RepositoriesModule = TestRepoModule;
        type ServicesModule = TestServiceModule;
    }

    pub struct TestInput;

    usecase_impl_type!();

    #[rstest]
    fn macro_expansion_works() {
        let _: UseCaseImpl<TestModules, TestInput>;
    }
}
