use std::{any::type_name, marker::PhantomData};

use crate::domain::{DomainType, error::ValidationErrors};

pub(super) mod error;

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
    pub fn new<F>(value: F, errors: &mut Vec<ValidationErrors>) -> Self
    where
        T: TryFrom<F, Error = ValidationErrors>,
    {
        let res = value
            .try_into()
            .inspect_err(|err: &ValidationErrors| errors.push(err.clone()));
        Self {
            inner: res,
            _phantom: PhantomData,
        }
    }

    pub fn lazy(self) -> impl FnOnce() -> T {
        || {
            self.inner.unwrap_or_else(|_| {
                panic!(
                    "`{}` should be Ok because error vec is empty",
                    type_name::<Self>()
                )
            })
        }
    }
}
