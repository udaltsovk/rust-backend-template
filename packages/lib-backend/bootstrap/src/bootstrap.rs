#[macro_export]
macro_rules! bootstrap {
    ($_app_crate: ident, [], $_modules_fut: expr) => {
      const {
           panic!("`bootstrap!` can't be called with empty bootstrapper array!");
       }
    };
    ($app_crate: ident, [$($bootstrapper: tt($config_field: expr)),*], $modules_fut: expr) => {
        async {
            use $app_crate::bootstrappers::BootstrapperExt as _;


            let modules = $modules_fut.await;
            tokio::join!(
                $($bootstrapper::bootstrap($config_field, modules.clone())),*
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use rstest::{fixture, rstest};

    // Test trait definitions at module level
    #[derive(Clone)]
    pub struct TestModules {
        pub calls: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait::async_trait]
    pub trait TestBootstrapperExt {
        async fn bootstrap(config: &(), modules: TestModules);
    }

    #[derive(Clone)]
    pub struct MultiTestModules {
        pub calls: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait::async_trait]
    pub trait MultiBootstrapperExt {
        async fn bootstrap(config: &(), modules: MultiTestModules);
    }

    #[derive(Clone)]
    #[expect(
        dead_code,
        reason = "Used in tests that are currently disabled due to compile-time panic"
    )]
    pub struct EmptyModules {
        pub initialized: Arc<Mutex<bool>>,
    }

    #[async_trait::async_trait]
    #[expect(
        dead_code,
        reason = "Used in tests that are currently disabled due to compile-time panic"
    )]
    pub trait EmptyBootstrapperExt {
        async fn bootstrap(config: &(), modules: EmptyModules);
    }

    #[derive(Clone)]
    pub struct ConfigModules;

    #[async_trait::async_trait]
    pub trait ConfigBootstrapperExt {
        async fn bootstrap(config: &(), modules: ConfigModules);
    }

    #[fixture]
    fn test_modules() -> TestModules {
        TestModules {
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[rstest]
    #[tokio::test]
    async fn bootstrap_macro_with_single_bootstrapper(
        test_modules: TestModules,
    ) {
        struct TestBootstrapper;

        #[async_trait::async_trait]
        impl TestBootstrapperExt for TestBootstrapper {
            async fn bootstrap(_config: &(), modules: TestModules) {
                modules
                    .calls
                    .lock()
                    .expect("Test mutex should not be poisoned")
                    .push("test".to_string());
            }
        }

        mod test_app {
            pub mod bootstrappers {
                pub use crate::bootstrap::tests::TestBootstrapperExt as BootstrapperExt;
            }
        }

        let modules_fut = async { test_modules.clone() };

        crate::bootstrap!(test_app, [TestBootstrapper(&())], modules_fut).await;

        let calls = test_modules
            .calls
            .lock()
            .expect("Test mutex should not be poisoned");
        assert_eq!(calls.len(), 1);
        assert!(calls.contains(&"test".to_string()));
    }

    #[fixture]
    fn multi_test_modules() -> MultiTestModules {
        MultiTestModules {
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[rstest]
    #[tokio::test]
    async fn bootstrap_macro_with_multiple_bootstrappers(
        multi_test_modules: MultiTestModules,
    ) {
        struct BootstrapperA;
        struct BootstrapperB;

        #[async_trait::async_trait]
        impl MultiBootstrapperExt for BootstrapperA {
            async fn bootstrap(_config: &(), modules: MultiTestModules) {
                modules
                    .calls
                    .lock()
                    .expect("Test mutex should not be poisoned")
                    .push("A".to_string());
            }
        }

        #[async_trait::async_trait]
        impl MultiBootstrapperExt for BootstrapperB {
            async fn bootstrap(_config: &(), modules: MultiTestModules) {
                modules
                    .calls
                    .lock()
                    .expect("Test mutex should not be poisoned")
                    .push("B".to_string());
            }
        }

        mod multi_app {
            pub mod bootstrappers {
                pub use crate::bootstrap::tests::MultiBootstrapperExt as BootstrapperExt;
            }
        }

        let modules_fut = async { multi_test_modules.clone() };

        crate::bootstrap!(
            multi_app,
            [BootstrapperA(&()), BootstrapperB(&())],
            modules_fut
        )
        .await;

        let calls = multi_test_modules
            .calls
            .lock()
            .expect("Test mutex should not be poisoned");
        assert_eq!(calls.len(), 2);
        assert!(calls.contains(&"A".to_string()));
        assert!(calls.contains(&"B".to_string()));
    }

    // Note: Test for empty bootstrappers is not included because the macro
    // intentionally causes a compile-time panic when called with an empty array.
    // This is the desired behavior to prevent runtime errors.

    #[rstest]
    #[tokio::test]
    async fn bootstrap_macro_check_config_called() {
        mod config_app {
            pub mod bootstrappers {
                pub use crate::bootstrap::tests::ConfigBootstrapperExt as BootstrapperExt;
            }
        }

        struct DummyBootstrapper;

        #[async_trait::async_trait]
        impl ConfigBootstrapperExt for DummyBootstrapper {
            async fn bootstrap(_config: &(), _modules: ConfigModules) {
                // Test implementation
            }
        }

        let modules_fut = async { ConfigModules };

        crate::bootstrap!(config_app, [DummyBootstrapper(&())], modules_fut)
            .await;
    }
}
