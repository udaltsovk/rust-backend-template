use crate::validation::constraints::Constraint;

pub struct IsAlphanumeric;

impl<T> Constraint<T> for IsAlphanumeric
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().chars().all(char::is_alphanumeric)
    }

    fn error_msg(&self) -> String {
        "must contain only alphanumeric characters".to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::IsAlphanumeric;
    use crate::validation::constraints::Constraint;

    #[rstest]
    #[case("hello123", true)]
    #[case("Hello123", true)]
    #[case("123", true)]
    #[case("abc", true)]
    #[case("ABC", true)]
    #[case("hello world", false)] // space is not alphanumeric
    #[case("hello-world", false)] // hyphen is not alphanumeric
    #[case("hello_world", false)] // underscore is not alphanumeric
    #[case("hello.world", false)] // dot is not alphanumeric
    #[case("hello@world", false)] // @ is not alphanumeric
    #[case("", true)] // empty string is vacuously true
    #[case("café", true)] // accented characters are alphanumeric
    #[case("αβγ", true)] // Greek letters are alphanumeric
    #[case("中文", true)] // Chinese characters are alphanumeric
    #[case("🚀", false)] // emoji is not alphanumeric
    #[case("hello!", false)] // exclamation mark is not alphanumeric
    #[case("123.45", false)] // decimal point is not alphanumeric
    #[case("a1b2c3", true)] // mixed letters and numbers
    fn test_is_alphanumeric_constraint(
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let constraint = IsAlphanumeric;
        assert_eq!(constraint.check(&input.to_string()), expected);
    }

    #[rstest]
    fn test_is_alphanumeric_error_message() {
        let constraint = IsAlphanumeric;
        assert_eq!(
            <IsAlphanumeric as Constraint<String>>::error_msg(&constraint),
            "must contain only alphanumeric characters"
        );
    }

    #[rstest]
    fn test_is_alphanumeric_with_numbers() {
        let constraint = IsAlphanumeric;

        assert!(constraint.check(&"123456".to_string()));
        assert!(constraint.check(&"0".to_string()));
        assert!(constraint.check(&"999".to_string()));
    }

    #[rstest]
    fn test_is_alphanumeric_with_letters() {
        let constraint = IsAlphanumeric;

        assert!(constraint.check(&"abcdef".to_string()));
        assert!(constraint.check(&"ABCDEF".to_string()));
        assert!(constraint.check(&"AbCdEf".to_string()));
    }

    #[rstest]
    fn test_is_alphanumeric_with_special_chars() {
        let constraint = IsAlphanumeric;

        // Common special characters should fail
        assert!(!constraint.check(&"hello world".to_string())); // space
        assert!(!constraint.check(&"hello,world".to_string())); // comma
        assert!(!constraint.check(&"hello;world".to_string())); // semicolon
        assert!(!constraint.check(&"hello:world".to_string())); // colon
        assert!(!constraint.check(&"hello/world".to_string())); // slash
        assert!(!constraint.check(&"hello\\world".to_string())); // backslash
        assert!(!constraint.check(&"hello|world".to_string())); // pipe
        assert!(!constraint.check(&"hello&world".to_string())); // ampersand
        assert!(!constraint.check(&"hello*world".to_string())); // asterisk
        assert!(!constraint.check(&"hello+world".to_string())); // plus
        assert!(!constraint.check(&"hello=world".to_string())); // equals
        assert!(!constraint.check(&"hello(world)".to_string())); // parentheses
        assert!(!constraint.check(&"hello[world]".to_string())); // brackets
        assert!(!constraint.check(&"hello{world}".to_string())); // braces
        assert!(!constraint.check(&"hello<world>".to_string())); // angle brackets
    }

    #[rstest]
    fn test_is_alphanumeric_unicode() {
        let constraint = IsAlphanumeric;

        // Unicode letters should pass
        assert!(constraint.check(&"ñoño".to_string())); // Spanish
        assert!(constraint.check(&"naïve".to_string())); // French
        assert!(constraint.check(&"Москва".to_string())); // Russian
        assert!(constraint.check(&"東京".to_string())); // Japanese
        assert!(constraint.check(&"서울".to_string())); // Korean

        // Unicode numbers should pass
        assert!(constraint.check(&"٠١٢٣".to_string())); // Arabic numerals
        assert!(constraint.check(&"零一二三".to_string())); // Chinese numerals
    }
}
