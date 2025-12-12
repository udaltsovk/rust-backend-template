#[macro_export]
macro_rules! usecase_struct {
    ($repos_ext: ident, $services_ext: ident) => {
        #[$crate::derive_where(Clone; R, S)]
        pub struct UseCase<R, S, T>
        where
            R: $repos_ext + Send + Sync + Clone,
            S: $services_ext + Send + Sync + Clone,
        {
            repositories: R,
            services: S,
            _entity: std::marker::PhantomData<T>,
        }

        impl<R, S, T> UseCase<R, S, T>
        where
            R: $repos_ext + Send + Sync + Clone,
            S: $services_ext + Send + Sync + Clone,
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

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use rstest::rstest;

    // Mock trait definitions for testing
    trait MockRepositoryExt: Send + Sync + Clone {
        fn get_data(&self) -> String;
    }

    trait MockServiceExt: Send + Sync + Clone {
        fn process_data(&self, data: &str) -> String;
    }

    // Mock implementations
    #[derive(Clone)]
    struct MockRepository {
        data: String,
    }

    impl MockRepositoryExt for MockRepository {
        fn get_data(&self) -> String {
            self.data.clone()
        }
    }

    #[derive(Clone)]
    struct MockService {
        prefix: String,
    }

    impl MockServiceExt for MockService {
        fn process_data(&self, data: &str) -> String {
            format!("{}: {}", self.prefix, data)
        }
    }

    // Test entity
    #[derive(Clone)]
    struct TestEntity {
        #[expect(
            dead_code,
            reason = "Test entity field used for type safety in tests"
        )]
        id: u32,
    }

    // Create test-specific version to avoid private bounds warnings
    #[derive(Clone)]
    struct TestUseCase<R, S, T>
    where
        R: MockRepositoryExt + Send + Sync + Clone,
        S: MockServiceExt + Send + Sync + Clone,
    {
        repositories: R,
        services: S,
        _entity: std::marker::PhantomData<T>,
    }

    impl<R, S, T> TestUseCase<R, S, T>
    where
        R: MockRepositoryExt + Send + Sync + Clone,
        S: MockServiceExt + Send + Sync + Clone,
    {
        pub fn new(repositories: R, services: S) -> Self {
            Self {
                repositories,
                services,
                _entity: std::marker::PhantomData,
            }
        }
    }

    #[rstest]
    fn usecase_creation() {
        let repository = MockRepository {
            data: "test data".to_string(),
        };
        let service = MockService {
            prefix: "processed".to_string(),
        };

        let usecase: TestUseCase<MockRepository, MockService, TestEntity> =
            TestUseCase::new(repository, service);

        // Verify the usecase was created successfully
        assert_eq!(usecase.repositories.get_data(), "test data");
        assert_eq!(usecase.services.process_data("input"), "processed: input");
    }

    #[rstest]
    fn usecase_clone() {
        let repository = MockRepository {
            data: "original data".to_string(),
        };
        let service = MockService {
            prefix: "clone_test".to_string(),
        };

        let usecase: TestUseCase<MockRepository, MockService, TestEntity> =
            TestUseCase::new(repository, service);
        let cloned_usecase = usecase.clone();

        // Verify cloned usecase works independently
        assert_eq!(cloned_usecase.repositories.get_data(), "original data");
        assert_eq!(
            cloned_usecase.services.process_data("test"),
            "clone_test: test"
        );

        // Verify original usecase still works (silences redundant clone warning)
        assert_eq!(usecase.repositories.get_data(), "original data");
    }

    #[rstest]
    #[case("repo_data_1", "service_1")]
    #[case("repo_data_2", "service_2")]
    #[case("", "empty_service")]
    #[case("special chars: !@#$%", "unicode: 🚀")]
    fn usecase_with_different_data(
        #[case] repo_data: &str,
        #[case] service_prefix: &str,
    ) {
        let repository = MockRepository {
            data: repo_data.to_string(),
        };
        let service = MockService {
            prefix: service_prefix.to_string(),
        };

        let usecase: TestUseCase<MockRepository, MockService, TestEntity> =
            TestUseCase::new(repository, service);

        assert_eq!(usecase.repositories.get_data(), repo_data);
        assert_eq!(
            usecase.services.process_data("input"),
            format!("{service_prefix}: input")
        );
    }

    #[rstest]
    fn usecase_phantom_data() {
        let repository = MockRepository {
            data: "phantom_test".to_string(),
        };
        let service = MockService {
            prefix: "phantom".to_string(),
        };

        let usecase: TestUseCase<MockRepository, MockService, TestEntity> =
            TestUseCase::new(repository, service);

        // Verify PhantomData is properly initialized
        assert_eq!(usecase._entity, PhantomData::<TestEntity>);
    }

    #[rstest]
    fn usecase_send_sync_bounds() {
        // This test verifies that the UseCase struct correctly implements Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TestUseCase<MockRepository, MockService, TestEntity>>(
        );
    }

    // Test with different entity types
    #[derive(Clone)]
    struct AnotherEntity {
        #[expect(
            dead_code,
            reason = "Test entity field used for type safety in tests"
        )]
        name: String,
    }

    #[rstest]
    fn usecase_with_different_entity_types() {
        let repository = MockRepository {
            data: "entity_test".to_string(),
        };
        let service = MockService {
            prefix: "entity".to_string(),
        };

        // Test with TestEntity
        let usecase1: TestUseCase<MockRepository, MockService, TestEntity> =
            TestUseCase::new(repository.clone(), service.clone());
        assert_eq!(usecase1.repositories.get_data(), "entity_test");

        // Test with AnotherEntity
        let usecase2: TestUseCase<MockRepository, MockService, AnotherEntity> =
            TestUseCase::new(repository, service);
        assert_eq!(usecase2.repositories.get_data(), "entity_test");

        // Verify they have different phantom data types
        assert_eq!(usecase1._entity, PhantomData::<TestEntity>);
        assert_eq!(usecase2._entity, PhantomData::<AnotherEntity>);
    }

    // Additional trait for comprehensive testing
    trait AnotherRepositoryExt: Send + Sync + Clone {
        fn fetch(&self) -> i32;
    }

    trait AnotherServiceExt: Send + Sync + Clone {
        fn transform(&self, value: i32) -> i32;
    }

    #[derive(Clone)]
    struct NumericRepository {
        value: i32,
    }

    impl AnotherRepositoryExt for NumericRepository {
        fn fetch(&self) -> i32 {
            self.value
        }
    }

    #[derive(Clone)]
    struct MathService {
        multiplier: i32,
    }

    impl AnotherServiceExt for MathService {
        fn transform(&self, value: i32) -> i32 {
            value.saturating_mul(self.multiplier)
        }
    }

    // Create another macro instance with different traits
    macro_rules! numeric_usecase_struct {
        ($repos_ext: ident, $services_ext: ident) => {
            #[crate::derive_where(Clone; R, S)]
            struct NumericUseCase<R, S, T>
            where
                R: $repos_ext + Send + Sync + Clone,
                S: $services_ext + Send + Sync + Clone,
            {
                repositories: R,
                services: S,
                _entity: std::marker::PhantomData<T>,
            }

            impl<R, S, T> NumericUseCase<R, S, T>
            where
                R: $repos_ext + Send + Sync + Clone,
                S: $services_ext + Send + Sync + Clone,
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

    numeric_usecase_struct!(AnotherRepositoryExt, AnotherServiceExt);

    #[rstest]
    #[case(10_i32, 2_i32, 20_i32)]
    #[case(5_i32, 3_i32, 15_i32)]
    #[case(0_i32, 10_i32, 0_i32)]
    #[case(-5_i32, 2_i32, -10_i32)]
    fn numeric_usecase(
        #[case] repo_value: i32,
        #[case] service_multiplier: i32,
        #[case] expected_result: i32,
    ) {
        let repository = NumericRepository {
            value: repo_value,
        };
        let service = MathService {
            multiplier: service_multiplier,
        };

        let usecase: NumericUseCase<
            NumericRepository,
            MathService,
            TestEntity,
        > = NumericUseCase::new(repository, service);

        let fetched_value = usecase.repositories.fetch();
        let transformed_value = usecase.services.transform(fetched_value);

        assert_eq!(transformed_value, expected_result);
    }

    #[rstest]
    fn macro_generates_independent_structs() {
        // Verify that different macro invocations create independent types
        let string_repo = MockRepository {
            data: "test".to_string(),
        };
        let string_service = MockService {
            prefix: "test".to_string(),
        };
        let numeric_repo = NumericRepository {
            value: 42_i32,
        };
        let numeric_service = MathService {
            multiplier: 2,
        };

        let string_usecase: TestUseCase<
            MockRepository,
            MockService,
            TestEntity,
        > = TestUseCase::new(string_repo, string_service);
        let numeric_usecase: NumericUseCase<
            NumericRepository,
            MathService,
            TestEntity,
        > = NumericUseCase::new(numeric_repo, numeric_service);

        // Both should work independently
        assert_eq!(string_usecase.repositories.get_data(), "test");
        assert_eq!(numeric_usecase.repositories.fetch(), 42_i32);
        assert_eq!(numeric_usecase.services.transform(10_i32), 20_i32);
    }

    // Test the main macro with public traits to ensure it compiles and works
    pub trait PublicRepositoryExt: Send + Sync + Clone {
        fn get_public_data(&self) -> String;
    }

    pub trait PublicServiceExt: Send + Sync + Clone {
        fn process_public_data(&self, data: &str) -> String;
    }

    #[derive(Clone)]
    pub struct PublicRepository {
        pub data: String,
    }

    impl PublicRepositoryExt for PublicRepository {
        fn get_public_data(&self) -> String {
            self.data.clone()
        }
    }

    #[derive(Clone)]
    pub struct PublicService {
        pub prefix: String,
    }

    impl PublicServiceExt for PublicService {
        fn process_public_data(&self, data: &str) -> String {
            format!("{}: {}", self.prefix, data)
        }
    }

    // Generate the UseCase struct using the main macro
    crate::usecase_struct!(PublicRepositoryExt, PublicServiceExt);

    #[rstest]
    fn main_macro_functionality() {
        let repository = PublicRepository {
            data: "main_macro_test".to_string(),
        };
        let service = PublicService {
            prefix: "main".to_string(),
        };

        let usecase: UseCase<PublicRepository, PublicService, TestEntity> =
            UseCase::new(repository, service);

        // Verify the main macro works correctly
        assert_eq!(usecase.repositories.get_public_data(), "main_macro_test");
        assert_eq!(
            usecase.services.process_public_data("input"),
            "main: input"
        );

        // Test cloning
        let cloned = usecase.clone();
        assert_eq!(cloned.repositories.get_public_data(), "main_macro_test");

        // Verify original usecase still works (silences redundant clone warning)
        assert_eq!(usecase.repositories.get_public_data(), "main_macro_test");
    }
}
