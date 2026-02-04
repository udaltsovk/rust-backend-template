use domain::validation::error::ValidationErrors;
use tap::Pipe as _;

use crate::{
    errors::validation::FieldErrors,
    validation::{UserInput, validator::ValidatorResult},
};

pub trait Parseable<T> {
    fn parse(self) -> ValidatorResult<T>;

    fn parse_field(self, field: impl Into<Option<String>>) -> ValidatorResult<T>
    where
        Self: Sized,
    {
        let field = field.into().unwrap_or_default();
        self.parse().map_err(|valid_errors| {
            valid_errors
                .into_inner()
                .into_iter()
                .map(|mut err| {
                    let sep = if err.field.ne(&"".into()) {
                        "."
                    } else {
                        Default::default()
                    };
                    err.field = format!("{field}{sep}{}", err.field).into();
                    err
                })
                .collect()
        })
    }
}

impl<I, T> Parseable<T> for UserInput<I>
where
    I: Parseable<T> + Default,
{
    fn parse(self) -> ValidatorResult<T> {
        match self {
            Self::Missing | Self::Null => I::default().parse(),
            Self::WrongType(value) => {
                let err = ValidationErrors::with_error("must be object", value);
                FieldErrors::from_validation_errors(&"".into(), err).pipe(Err)
            },
            Self::Ok(inp) => inp.parse(),
        }
    }
}

impl<I, T> Parseable<Vec<T>> for Vec<I>
where
    I: Parseable<T>,
{
    fn parse(self) -> ValidatorResult<Vec<T>> {
        let (oks, errs): (Vec<_>, Vec<_>) = self
            .into_iter()
            .enumerate()
            .map(|(i, v)| match v.parse_field(i.to_string()) {
                Ok(ok) => (Some(ok), None),
                Err(err) => (None, Some(err)),
            })
            .unzip();

        errs.into_iter()
            .flatten()
            .collect::<FieldErrors>()
            .into_result(|_| oks.into_iter().flatten().collect())
    }
}
