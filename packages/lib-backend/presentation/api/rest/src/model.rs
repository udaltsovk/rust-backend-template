use std::{any::type_name, marker::PhantomData};

pub use domain::validation::error::ValidationErrors;
use domain::validation::{ValidationConfirmation, error::ValidationResult};

pub trait Parseable<T> {
    const FIELD: &str;

    fn parse(self) -> ValidationResult<T>;
}

impl<I, T> Parseable<Vec<T>> for Vec<I>
where
    I: Parseable<T>,
{
    const FIELD: &str = I::FIELD;

    fn parse(self) -> ValidationResult<Vec<T>> {
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

impl<I, T> Parseable<T> for Option<I>
where
    I: Parseable<T>,
{
    const FIELD: &str = I::FIELD;

    fn parse(self) -> ValidationResult<T> {
        self.map(I::parse).transpose()?.ok_or_else(|| {
            ValidationErrors::with_error(
                Self::FIELD,
                "should be not null",
                None::<()>,
            )
        })
    }
}

pub struct NestedValidator<J, I>
where
    J: Parseable<I>,
{
    inner: ValidationResult<I>,
    _phantom: PhantomData<J>,
}

impl<J, I> NestedValidator<J, I>
where
    J: Parseable<I>,
{
    pub fn new(value: J, errors: &mut ValidationErrors) -> Self {
        let res: ValidationResult<I> = value.parse();

        if let Err(ref err) = res {
            errors.extend(err.clone());
        }

        Self {
            inner: res,
            _phantom: PhantomData,
        }
    }

    pub fn validated(self, _confirmation: ValidationConfirmation) -> I {
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

impl<I, T> IntoNestedValidator<T> for I
where
    I: Parseable<T> + Sized,
{
    #[inline]
    fn into_nested_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> NestedValidator<I, T> {
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
