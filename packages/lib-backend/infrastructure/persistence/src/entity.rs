use std::any::type_name;

use domain::{DomainType, validation::error::ValidationErrors};

pub trait DomainTypeFromDb<T, I>
where
    Self: Sized,
    T: DomainType<I> + TryFrom<Self, Error = ValidationErrors>,
    I: From<T> + Clone,
{
    fn into_domain(self) -> T;
}

impl<F, T, I> DomainTypeFromDb<T, I> for F
where
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
    I: From<T> + Clone,
{
    fn into_domain(self) -> T {
        self.try_into().unwrap_or_else(|_| {
            panic!(
                "Expected `{}` from the db should be valid",
                type_name::<T>()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use domain::{DomainType, validation::error::ValidationErrors};
    use rstest::rstest;

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestDomain(String);

    impl AsRef<String> for TestDomain {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }

    impl AsMut<String> for TestDomain {
        fn as_mut(&mut self) -> &mut String {
            &mut self.0
        }
    }

    impl From<TestDomain> for String {
        fn from(value: TestDomain) -> Self {
            value.0
        }
    }

    impl DomainType<String> for TestDomain {}

    struct TestDbEntity(String);

    impl TryFrom<TestDbEntity> for TestDomain {
        type Error = ValidationErrors;

        fn try_from(value: TestDbEntity) -> Result<Self, Self::Error> {
            if value.0 == "invalid" {
                Err(ValidationErrors::default())
            } else {
                Ok(Self(value.0))
            }
        }
    }

    #[rstest]
    #[case("valid", "valid")]
    fn into_domain_success(#[case] input: &str, #[case] expected: &str) {
        let db_entity = TestDbEntity(input.to_string());
        let domain_entity: TestDomain = db_entity.into_domain();
        assert_eq!(domain_entity.0, expected);
    }

    #[rstest]
    #[should_panic(expected = "from the db should be valid")]
    fn into_domain_panic_on_failure() {
        let db_entity = TestDbEntity("invalid".to_string());
        let _: TestDomain = db_entity.into_domain();
    }
}
