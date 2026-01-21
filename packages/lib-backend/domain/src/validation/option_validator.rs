use crate::validation::{
    ValidationConfirmation, Validator, error::ValidationErrors,
};

pub struct OptionValidator<T> {
    inner: Option<Validator<T>>,
}

impl<T> OptionValidator<T> {
    pub fn new<F>(value: Option<F>, errors: &mut ValidationErrors) -> Self
    where
        T: TryFrom<F, Error = ValidationErrors>,
    {
        Self {
            inner: value.map(|value| Validator::new(value, errors)),
        }
    }

    pub fn validated(self, confirmation: ValidationConfirmation) -> Option<T> {
        self.inner.map(|inner| inner.validated(confirmation))
    }
}

pub trait IntoOptionValidator<F, T>
where
    Self: Sized,
    T: TryFrom<F, Error = ValidationErrors>,
{
    fn into_option_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> OptionValidator<T>;
}

impl<F, T> IntoOptionValidator<F, T> for Option<F>
where
    T: TryFrom<F, Error = ValidationErrors>,
{
    #[inline]
    fn into_option_validator(
        self,
        errors: &mut ValidationErrors,
    ) -> OptionValidator<T> {
        OptionValidator::new(self, errors)
    }
}

#[macro_export]
macro_rules! into_option_validators {
    ($($field: expr),*) => {
        {
            #[allow(unused_imports)]
            use $crate::validation::{IntoOptionValidator as _, error::ValidationErrors};

            #[allow(unused_mut)]
            let mut errors = ValidationErrors::new();

            let option_validators = ($(
              $field.into_option_validator(&mut errors)
            ),*);

            (errors, option_validators)
        }
    };
}
