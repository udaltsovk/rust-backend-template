use std::{any::type_name, marker::PhantomData};

use crate::{DomainType, validation::error::ValidationErrors};

pub mod constraints;
pub mod error;

pub use constraints::Constraints;

pub struct Validator<T, I>
where
    I: Clone,
    T: DomainType<I>,
{
    inner: Result<T, ValidationErrors>,
    _phantom: PhantomData<I>,
}

impl<T, I> Validator<T, I>
where
    I: Clone,
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
    I: Clone,
    T: DomainType<I> + TryFrom<Self, Error = ValidationErrors>,
{
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<T, I> {
        Validator::new(self, errors)
    }
}

impl<F, T, I> IntoValidator<T, I> for F
where
    I: Clone,
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
{
}
