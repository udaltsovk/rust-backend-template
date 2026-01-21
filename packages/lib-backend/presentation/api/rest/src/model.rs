use std::{any::type_name, marker::PhantomData};

use domain::validation::ValidationConfirmation;
pub use domain::validation::error::ValidationErrors;

pub trait Parseable<T> {
    fn parse(self) -> Result<T, ValidationErrors>;
}

impl<J, T> Parseable<Vec<T>> for Vec<J>
where
    J: Parseable<T>,
{
    fn parse(self) -> Result<Vec<T>, ValidationErrors> {
        let (errors, converted): (Vec<_>, Vec<_>) = self
            .into_iter()
            .map(|v| match v.parse() {
                Ok(ok) => (None, Some(ok)),
                Err(err) => (Some(err), None),
            })
            .unzip();
        errors
            .into_iter()
            .flatten()
            .collect::<ValidationErrors>()
            .into_result(|_| converted.into_iter().flatten().collect())
    }
}

pub struct NestedValidator<J, T>
where
    J: Parseable<T>,
{
    inner: Result<T, ValidationErrors>,
    _phantom: PhantomData<J>,
}

impl<J, T> NestedValidator<J, T>
where
    J: Parseable<T>,
{
    pub fn new(value: J, errors: &mut ValidationErrors) -> Self {
        let res: Result<T, ValidationErrors> = value.parse();

        if let Err(ref err) = res {
            errors.extend(err.clone());
        }

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

pub trait IntoNestedValidator<T>
where
    Self: Parseable<T> + Sized,
{
    fn into_nested_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> NestedValidator<Self, T>;
}

impl<J, T> IntoNestedValidator<T> for J
where
    J: Parseable<T> + Sized,
{
    #[inline]
    fn into_nested_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> NestedValidator<J, T> {
        NestedValidator::new(self, errors)
    }
}

#[macro_export]
macro_rules! into_nested_validators {
    ($($field: expr),*) => {
        {
            #[allow(unused_imports)]
            use $crate::model::{IntoNestedValidator as _, ValidationErrors};

            #[allow(unused_mut)]
            let mut errors = ValidationErrors::new();

            let validators = ($(
              $field.into_nested_validator(&mut errors)
            ),*);

            (errors, validators)
        }
    };
}
