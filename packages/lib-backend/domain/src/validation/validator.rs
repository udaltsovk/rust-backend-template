use std::any::type_name;

use crate::validation::{ValidationConfirmation, error::ValidationErrors};

pub struct Validator<T> {
    inner: Result<T, ValidationErrors>,
}

impl<T> Validator<T> {
    pub fn new<F>(value: F, errors: &mut ValidationErrors) -> Self
    where
        T: TryFrom<F, Error = ValidationErrors>,
    {
        let res: Result<T, ValidationErrors> = value.try_into();

        if let Err(ref err) = res {
            errors.extend(err.clone());
        }

        Self {
            inner: res,
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

pub trait IntoValidator<T>
where
    Self: Sized,
    T: TryFrom<Self, Error = ValidationErrors>,
{
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<T>;
}

impl<F, T> IntoValidator<T> for F
where
    T: TryFrom<F, Error = ValidationErrors>,
{
    #[inline]
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<T> {
        Validator::new(self, errors)
    }
}

#[macro_export]
macro_rules! into_validators {
    ($($field: expr),*) => {
        {
            #[allow(unused_imports)]
            use $crate::validation::{IntoValidator as _, error::ValidationErrors};

            #[allow(unused_mut)]
            let mut errors = ValidationErrors::new();

            let validators = ($(
              $field.into_validator(&mut errors)
            ),*);

            (errors, validators)
        }
    };
}
