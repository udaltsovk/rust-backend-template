use macros::{constraint, constraint_check};

use super::{Constraint, Validation};

#[macro_export]
macro_rules! has {
    ($name:ident, $matcher:expr $(,)?) => {
        $crate::pastey::paste! {
            #[constraint]
            pub struct [<$name:camel>] {
                err_fn: fn(&str) -> String,
            }

            #[constraint_check(String)]
            impl [<$name:camel>] {
                fn is_valid(value: &str) -> bool {
                    value.chars().any($matcher)
                }
            }
        }
    };
}

has!(letter, |c| c.is_ascii_alphabetic());

has!(lowercase, char::is_lowercase);

has!(uppercase, char::is_uppercase);

has!(digit, |c| c.is_ascii_digit());

has!(special_char, |c| matches!(
    c,
    '@' | '$' | '!' | '%' | '*' | '?' | '&'
));
