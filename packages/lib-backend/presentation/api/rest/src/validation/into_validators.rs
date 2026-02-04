#[macro_export]
macro_rules! into_validators {
    ($(field!($field_value: expr, $convertion_fn: ident, $field_name: expr)),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut errors = $crate::errors::validation::FieldErrors::new();

            let validators = ($(
              $crate::validation::validator::Validator::$convertion_fn(
                  $field_name,
                  $field_value,
                  &mut errors
              )
            ),*);

            (errors, validators)
        }
    };
}
