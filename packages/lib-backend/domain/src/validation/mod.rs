use std::{any::type_name, marker::PhantomData};

use crate::{DomainType, validation::error::ValidationErrors};

pub mod constraints;
pub mod error;

pub use constraints::Constraints;

pub struct Validator<T, I>
where
    I: From<T> + Clone,
    T: DomainType<I>,
{
    inner: Result<T, ValidationErrors>,
    _phantom: PhantomData<I>,
}

impl<T, I> Validator<T, I>
where
    I: From<T> + Clone,
    T: DomainType<I>,
{
    pub fn new<F>(value: F, errors: &mut ValidationErrors) -> Self
    where
        T: TryFrom<F, Error = ValidationErrors>,
    {
        let res = value
            .try_into()
            .inspect_err(|err: &ValidationErrors| errors.extend(err.clone()));
        Self {
            inner: res,
            _phantom: PhantomData,
        }
    }

    pub fn validated(self, _confirmation: ValidationConfirmation) -> T {
        self.inner.unwrap_or_else(|_| {
            panic!(
                "`{}` should be Ok because error vec is empty",
                type_name::<Self>()
            )
        })
    }
}

#[derive(Clone, Copy)]
pub struct ValidationConfirmation {
    _phantom: PhantomData<()>,
}

pub trait IntoValidator<T, I>
where
    Self: Sized,
    I: From<T> + Clone,
    T: DomainType<I> + TryFrom<Self, Error = ValidationErrors>,
{
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<T, I> {
        Validator::new(self, errors)
    }
}

impl<F, T, I> IntoValidator<T, I> for F
where
    I: From<T> + Clone,
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
{
}

#[macro_export]
macro_rules! into_validators {
    ($($field: expr),*) => {
        {
            use $crate::validation::{IntoValidator as _, error::ValidationErrors};

            let mut errors = ValidationErrors::new();

            let validators = ($(
              $field.into_validator(&mut errors)
            ),*);

            (errors, validators)
        }
    };
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use rstest::rstest;

    use super::{IntoValidator, ValidationConfirmation, Validator};
    use crate::validation::error::ValidationErrors;

    // Type aliases to simplify complex type annotations
    type TestValidator = Validator<TestValue, String>;
    type TwoValidators = (TestValidator, TestValidator);
    type ThreeValidators = (TestValidator, TestValidator, TestValidator);

    // Test domain type implementation
    #[derive(Debug, Clone, PartialEq)]
    struct TestValue {
        inner: String,
    }

    impl AsRef<String> for TestValue {
        fn as_ref(&self) -> &String {
            &self.inner
        }
    }

    impl AsMut<String> for TestValue {
        fn as_mut(&mut self) -> &mut String {
            &mut self.inner
        }
    }

    impl From<TestValue> for String {
        fn from(value: TestValue) -> Self {
            value.inner
        }
    }

    impl crate::DomainType<String> for TestValue {}

    impl TryFrom<String> for TestValue {
        type Error = ValidationErrors;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.is_empty() {
                let mut errors = ValidationErrors::new();
                errors.push("value", "cannot be empty");
                Err(errors)
            } else {
                Ok(TestValue {
                    inner: value,
                })
            }
        }
    }

    #[rstest]
    fn test_validator_new_success() {
        let mut errors = ValidationErrors::new();
        let validator = Validator::<TestValue, String>::new(
            "valid".to_string(),
            &mut errors,
        );

        assert!(errors.into_inner().is_empty());
        assert!(validator.inner.is_ok());
    }

    #[rstest]
    fn test_validator_new_failure() {
        let mut errors = ValidationErrors::new();
        let validator =
            Validator::<TestValue, String>::new("".to_string(), &mut errors);

        assert!(!errors.into_inner().is_empty());
        assert!(validator.inner.is_err());
    }

    #[rstest]
    fn test_validator_validated_success() {
        let mut errors = ValidationErrors::new();
        let validator = Validator::<TestValue, String>::new(
            "valid".to_string(),
            &mut errors,
        );

        let confirmation = ValidationConfirmation {
            _phantom: PhantomData,
        };

        // This should not panic since validation passed
        let result = validator.validated(confirmation);
        assert_eq!(result.inner, "valid");
    }

    #[rstest]
    #[should_panic(
        expected = "`lib_domain::validation::Validator<lib_domain::validation::tests::TestValue, alloc::string::String>` should be Ok because error vec is empty"
    )]
    fn test_validator_validated_panic_on_error() {
        let mut errors = ValidationErrors::new();
        let validator: Validator<TestValue, String> =
            Validator::<TestValue, String>::new("".to_string(), &mut errors);

        let confirmation = ValidationConfirmation {
            _phantom: PhantomData,
        };

        // This should panic since validation failed
        validator.validated(confirmation);
    }

    #[rstest]
    fn test_into_validator_trait() {
        let mut errors = ValidationErrors::new();
        let validator: Validator<TestValue, String> =
            "valid".to_string().into_validator(&mut errors);

        assert!(errors.into_inner().is_empty());
        assert!(validator.inner.is_ok());
    }

    #[rstest]
    fn test_into_validator_trait_with_error() {
        let mut errors = ValidationErrors::new();
        let validator: Validator<TestValue, String> =
            "".to_string().into_validator(&mut errors);

        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 1);
        assert_eq!(error_list[0].0, "value");
        assert_eq!(error_list[0].1, "cannot be empty");
        assert!(validator.inner.is_err());
    }

    #[rstest]
    fn test_into_validators_macro_single_field() {
        let (errors, validator): (
            ValidationErrors,
            Validator<TestValue, String>,
        ) = crate::into_validators!("valid".to_string());

        assert!(errors.into_inner().is_empty());
        assert!(validator.inner.is_ok());
    }

    #[rstest]
    fn test_into_validators_macro_multiple_fields() {
        let (errors, validators): (ValidationErrors, ThreeValidators) = crate::into_validators!(
            "valid1".to_string(),
            "valid2".to_string(),
            "valid3".to_string()
        );

        assert!(errors.into_inner().is_empty());
        let (validator1, validator2, validator3) = validators;
        assert!(validator1.inner.is_ok());
        assert!(validator2.inner.is_ok());
        assert!(validator3.inner.is_ok());
    }

    #[rstest]
    fn test_into_validators_macro_with_errors() {
        let (errors, validators): (ValidationErrors, ThreeValidators) = crate::into_validators!(
            "valid".to_string(),
            "".to_string(),
            "also_valid".to_string()
        );

        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 1);
        assert_eq!(error_list[0].0, "value");
        assert_eq!(error_list[0].1, "cannot be empty");

        let (validator1, validator2, validator3) = validators;
        assert!(validator1.inner.is_ok());
        assert!(validator2.inner.is_err());
        assert!(validator3.inner.is_ok());
    }

    #[rstest]
    fn test_into_validators_macro_multiple_errors() {
        let (errors, validators): (ValidationErrors, TwoValidators) =
            crate::into_validators!("".to_string(), "".to_string());

        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 2);
        assert_eq!(error_list[0].0, "value");
        assert_eq!(error_list[0].1, "cannot be empty");
        assert_eq!(error_list[1].0, "value");
        assert_eq!(error_list[1].1, "cannot be empty");

        let (validator1, validator2) = validators;
        assert!(validator1.inner.is_err());
        assert!(validator2.inner.is_err());
    }

    #[rstest]
    fn test_validation_confirmation_copy() {
        let confirmation = ValidationConfirmation {
            _phantom: PhantomData,
        };

        // Should be Copy
        let copied_confirmation = confirmation;
        let _another_copy = confirmation;
        let _cloned = copied_confirmation;

        // Just verify they exist and can be used
        assert_eq!(
            std::mem::size_of_val(&confirmation),
            std::mem::size_of_val(&copied_confirmation)
        );
    }
}
