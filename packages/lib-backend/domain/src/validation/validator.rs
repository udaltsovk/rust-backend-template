use std::{any::type_name, fmt::Debug};

use crate::validation::{
    ExternalInput, Nullable, Optional, OptionalNullable,
    ValidationConfirmation,
    error::{ValidationErrors, ValidationResult},
};

pub struct Validator<T> {
    inner: ValidationResult<T>,
}

impl<T> Validator<T> {
    pub fn from_result(
        result: ValidationResult<T>,
        errors: &mut ValidationErrors,
    ) -> Self {
        if let Err(errrors) = &result {
            errors.extend(errrors.clone());
        }

        Self {
            inner: result,
        }
    }

    pub fn map<U, F>(self, f: F) -> Validator<U>
    where
        F: FnOnce(T) -> U,
    {
        Validator {
            inner: self.inner.map(f),
        }
    }

    pub fn required<F>(
        input: ExternalInput<F>,
        errors: &mut ValidationErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
    {
        Self::from_result(input.try_into(), errors)
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

impl<T> Validator<Nullable<T>> {
    pub fn required_nullable<F>(
        input: ExternalInput<F>,
        errors: &mut ValidationErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
        F: Debug,
    {
        match input {
            ExternalInput::None => Self {
                inner: Ok(Nullable::Null),
            },
            input => {
                let Validator {
                    inner: res,
                } = Validator::required(input, errors);

                Self {
                    inner: res.map(Nullable::NonNull),
                }
            },
        }
    }
}

impl<T> Validator<Optional<T>> {
    pub fn optional<F>(
        input: ExternalInput<F>,
        errors: &mut ValidationErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
        F: Debug,
    {
        match input {
            ExternalInput::Missing => Self {
                inner: Ok(Optional::Missing),
            },
            input => {
                let Validator {
                    inner: res,
                } = Validator::required(input, errors);

                Self {
                    inner: res.map(Optional::Present),
                }
            },
        }
    }
}

impl<T> Validator<OptionalNullable<T>> {
    pub fn optional_nullable<F>(
        input: ExternalInput<F>,
        errors: &mut ValidationErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
        F: Debug,
    {
        match input {
            ExternalInput::Missing => Self {
                inner: Ok(OptionalNullable::Missing),
            },
            ExternalInput::None => Self {
                inner: Ok(OptionalNullable::Null),
            },
            input => {
                let Validator {
                    inner: res,
                } = Validator::required(input, errors);

                Self {
                    inner: res.map(OptionalNullable::Just),
                }
            },
        }
    }
}

pub trait IntoValidator<T, D, R>
where
    Self: Sized,
    D: TryFrom<ExternalInput<T>, Error = ValidationErrors>,
    ExternalInput<T>: From<Self>,
{
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<R>;
}

impl<F, T, D> IntoValidator<T, D, D> for F
where
    D: TryFrom<ExternalInput<T>, Error = ValidationErrors>,
    ExternalInput<T>: From<F>,
{
    #[inline]
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<D> {
        Validator::required(self.into(), errors)
    }
}

impl<F, T, D> IntoValidator<T, D, Nullable<D>> for F
where
    D: TryFrom<ExternalInput<T>, Error = ValidationErrors>,
    ExternalInput<T>: From<F>,
    T: Debug,
{
    #[inline]
    fn into_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> Validator<Nullable<D>> {
        Validator::required_nullable(self.into(), errors)
    }
}

impl<F, T, D> IntoValidator<T, D, Optional<D>> for F
where
    D: TryFrom<ExternalInput<T>, Error = ValidationErrors>,
    ExternalInput<T>: From<F>,
    T: Debug,
{
    #[inline]
    fn into_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> Validator<Optional<D>> {
        Validator::optional(self.into(), errors)
    }
}

impl<F, T, D> IntoValidator<T, D, OptionalNullable<D>> for F
where
    D: TryFrom<ExternalInput<T>, Error = ValidationErrors>,
    ExternalInput<T>: From<F>,
    T: Debug,
{
    #[inline]
    fn into_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> Validator<OptionalNullable<D>> {
        Validator::optional_nullable(self.into(), errors)
    }
}
