use std::{any::type_name, marker::PhantomData};

use crate::{
    DomainType,
    validation::{ValidationConfirmation, error::ValidationErrors},
};

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
        let res: Result<T, ValidationErrors> = value.try_into();

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

pub trait IntoValidator<T, I>
where
    Self: Sized,
    I: From<T> + Clone,
    T: DomainType<I> + TryFrom<Self, Error = ValidationErrors>,
{
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<T, I>;
}

impl<F, T, I> IntoValidator<T, I> for F
where
    I: From<T> + Clone,
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
{
    #[inline]
    fn into_validator(self, errors: &mut ValidationErrors) -> Validator<T, I> {
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
