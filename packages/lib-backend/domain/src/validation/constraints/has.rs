#[macro_export]
macro_rules! has {
    ($name: ident, $matcher: expr, $msg: literal) => {
        pastey::paste! {
            pub struct [<$name:camel>];

            impl $crate::validation::constraints::Constraint<String> for [<$name:camel>] {
                fn check(&self, value: &String) -> bool {
                    value.chars().any($matcher)
                }

                fn error_msg(&self) -> String {
                    concat!("must contain at least one ", $msg).to_string()
                }
            }
        }
    };
}

has!(letter, |c| c.is_ascii_alphabetic(), "letter");

has!(lowercase, char::is_lowercase, "lowercase letter");

has!(uppercase, char::is_uppercase, "uppercase letter");

has!(digit, |c| c.is_ascii_digit(), "digit");

has!(
    special_char,
    |c| matches!(c, '@' | '$' | '!' | '%' | '*' | '?' | '&'),
    "special character (@$!%*?&)"
);
