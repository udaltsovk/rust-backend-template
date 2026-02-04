use std::any::type_name;

use domain::validation::{
    ExternalInput, ValidationConfirmation, error::ValidationErrors,
};
use tap::Pipe as _;

use crate::{
    errors::validation::FieldErrors,
    validation::{UserInput, parseable::Parseable},
};

pub type ValidatorResult<T> = Result<T, FieldErrors>;

pub struct Validator<T> {
    pub(super) inner: ValidatorResult<T>,
}

impl<T> Validator<T> {
    pub fn from_result(
        result: ValidatorResult<T>,
        errors: &mut FieldErrors,
    ) -> Self {
        let inner = result.inspect_err(|err| {
            errors.extend(err.clone());
        });

        Self {
            inner,
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
        field: &'static str,
        input: UserInput<F>,
        errors: &mut FieldErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
    {
        Self::from_result(
            input.pipe(ExternalInput::from).try_into().map_err(|err| {
                FieldErrors::from_validation_errors(&field.into(), err)
            }),
            errors,
        )
    }

    pub fn nested<F, I>(field: I, input: F, errors: &mut FieldErrors) -> Self
    where
        F: Parseable<T>,
        I: Into<Option<&'static str>>,
    {
        Self::from_result(
            input.parse_field(field.into().map(str::to_string)),
            errors,
        )
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

impl<T> From<Vec<Validator<T>>> for Validator<Vec<T>> {
    fn from(validators: Vec<Validator<T>>) -> Self {
        let res = validators.into_iter().map(|v| v.inner).collect();
        Self {
            inner: res,
        }
    }
}

impl<T> Validator<Option<T>> {
    pub fn required_nullable<F>(
        field: &'static str,
        input: UserInput<F>,
        errors: &mut FieldErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
    {
        match input {
            UserInput::Null => Self {
                inner: Ok(None),
            },
            input => {
                let Validator {
                    inner: res,
                } = Validator::required(field, input, errors);

                Self {
                    inner: res.map(Some),
                }
            },
        }
    }

    pub fn optional<F>(
        field: &'static str,
        input: UserInput<F>,
        errors: &mut FieldErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
    {
        match input {
            UserInput::Missing => Self {
                inner: Ok(None),
            },
            input => {
                let Validator {
                    inner: res,
                } = Validator::required(field, input, errors);

                Self {
                    inner: res.map(Some),
                }
            },
        }
    }
}

impl<T> Validator<Option<Option<T>>> {
    pub fn optional_nullable<F>(
        field: &'static str,
        input: UserInput<F>,
        errors: &mut FieldErrors,
    ) -> Self
    where
        T: TryFrom<ExternalInput<F>, Error = ValidationErrors>,
    {
        match input {
            UserInput::Missing => Self {
                inner: Ok(None),
            },
            UserInput::Null => Self {
                inner: Ok(Some(None)),
            },
            input => {
                let Validator {
                    inner: res,
                } = Validator::required(field, input, errors);

                Self {
                    inner: res.map(Some).map(Some),
                }
            },
        }
    }
}
