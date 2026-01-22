#[macro_export]
macro_rules! into_validators {
    ($($field: expr),*) => {
        {
            #[allow(unused_imports)]
            use $crate::validation::IntoValidator as _;

            #[allow(unused_mut)]
            let mut errors = $crate::validation::error::ValidationErrors::new();

            let validators = ($(
              $crate::validation::ExternalInput::from($field)
                  .into_validator(&mut errors)
            ),*);

            (errors, validators)
        }
    };
}
