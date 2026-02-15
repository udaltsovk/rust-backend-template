use bon::Builder;

use crate::validation::constraints::Constraint;

#[derive(Builder)]
#[builder(derive(Clone), start_fn = with_err)]
pub struct IsAlphanumeric<T>
where
    T: ToString,
{
    #[builder(start_fn)]
    err_fn: fn(&T) -> String,
}

impl<T> Constraint<T> for IsAlphanumeric<T>
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().chars().all(char::is_alphanumeric)
    }

    fn error_msg(&self, rejected_value: &T) -> String {
        (self.err_fn)(rejected_value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::IsAlphanumeric;
    use crate::validation::constraints::Constraint;

    fn err(_: &String) -> String {
        "must contain only alphanumeric characters".to_string()
    }

    #[fixture]
    #[default(impl Constraint<String>)]
    fn constraint() -> impl Constraint<String> {
        IsAlphanumeric::with_err(err).build()
    }

    #[rstest]
    #[case("hello123", true)] // mixed letters and numbers
    #[case("Hello123", true)] // mixed case with numbers
    #[case("123", true)] // numbers only
    #[case("abc", true)] // lowercase letters
    #[case("ABC", true)] // uppercase letters
    #[case("123456", true)] // numbers only
    #[case("0", true)] // single digit
    #[case("999", true)] // multiple same digits
    #[case("abcdef", true)] // lowercase letters
    #[case("ABCDEF", true)] // uppercase letters
    #[case("AbCdEf", true)] // mixed case letters
    #[case("a1b2c3", true)] // mixed letters and numbers
    #[case("", true)] // empty string is vacuously true
    #[case("café", true)] // accented characters are alphanumeric
    #[case("αβγ", true)] // Greek letters are alphanumeric
    #[case("中文", true)] // Chinese characters are alphanumeric
    #[case("ñoño", true)] // Spanish
    #[case("naïve", true)] // French
    #[case("Москва", true)] // Russian
    #[case("東京", true)] // Japanese
    #[case("서울", true)] // Korean
    #[case("٠١٢٣", true)] // Arabic numerals
    #[case("零一二三", true)] // Chinese numerals
    #[case("hello world", false)] // space is not alphanumeric
    #[case("hello-world", false)] // hyphen is not alphanumeric
    #[case("hello_world", false)] // underscore is not alphanumeric
    #[case("hello.world", false)] // dot is not alphanumeric
    #[case("hello@world", false)] // @ is not alphanumeric
    #[case("🚀", false)] // emoji is not alphanumeric
    #[case("hello!", false)] // exclamation mark is not alphanumeric
    #[case("123.45", false)] // decimal point is not alphanumeric
    #[case("hello,world", false)] // comma
    #[case("hello;world", false)] // semicolon
    #[case("hello:world", false)] // colon
    #[case("hello/world", false)] // slash
    #[case("hello\\world", false)] // backslash
    #[case("hello|world", false)] // pipe
    #[case("hello&world", false)] // ampersand
    #[case("hello*world", false)] // asterisk
    #[case("hello+world", false)] // plus
    #[case("hello=world", false)] // equals
    #[case("hello(world)", false)] // parentheses
    #[case("hello[world]", false)] // brackets
    #[case("hello{world}", false)] // braces
    #[case("hello<world>", false)] // angle brackets
    fn is_alphanumeric_constraint(
        constraint: impl Constraint<String>,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input.to_string()), expected);
    }

    #[rstest]
    fn is_alphanumeric_error_message(constraint: impl Constraint<String>) {
        let value = "**".into();
        assert_eq!(constraint.error_msg(&value), err(&value));
    }
}
