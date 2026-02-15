use bon::Builder;

use crate::validation::constraints::Constraint;

#[macro_export]
macro_rules! has {
    ($name: ident, $matcher: expr $(,)?) => {
        $crate::pastey::paste! {
            #[derive(Builder)]
            #[builder(derive(Clone), start_fn = with_err)]
            pub struct [<$name:camel>] {
                #[builder(start_fn)]
                err_fn: fn(&str) -> String,
            }


            impl Constraint<String> for [<$name:camel>] {
                fn check(&self, value: &String) -> bool {
                    value.chars().any($matcher)
                }

                fn error_msg(&self, rejected_value: &String) -> String {
                    (self.err_fn)(rejected_value)
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
