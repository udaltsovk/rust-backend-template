use crate::validation::constraints::Constraint;

pub struct IsAsciiAlphanumeric;

impl<T> Constraint<T> for IsAsciiAlphanumeric
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn error_msg(&self) -> String {
        "must contain only ascii alphanumeric characters".to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::IsAsciiAlphanumeric;
    use crate::validation::constraints::Constraint;

    #[rstest]
    #[case("hello123", true)]
    #[case("HELLO123", true)]
    #[case("123", true)]
    #[case("abc", true)]
    #[case("ABC", true)]
    #[case("hello world", false)] // space is not ASCII alphanumeric
    #[case("hello-world", false)] // hyphen is not ASCII alphanumeric
    #[case("hello_world", false)] // underscore is not ASCII alphanumeric
    #[case("hello.world", false)] // dot is not ASCII alphanumeric
    #[case("hello@world", false)] // @ is not ASCII alphanumeric
    #[case("", true)] // empty string is vacuously true
    #[case("café", false)] // é is not ASCII
    #[case("naïve", false)] // ï is not ASCII
    #[case("αβγ", false)] // Greek letters are not ASCII
    #[case("中文", false)] // Chinese characters are not ASCII
    #[case("🚀", false)] // emoji is not ASCII
    #[case("hello!", false)] // exclamation mark is not ASCII alphanumeric
    #[case("123.45", false)] // decimal point is not ASCII alphanumeric
    #[case("a1b2c3", true)] // mixed ASCII letters and numbers
    #[case("Москва", false)] // Cyrillic is not ASCII
    #[case("東京", false)] // Japanese is not ASCII
    #[case("서울", false)] // Korean is not ASCII
    #[case("hello!@#$%^&*()", false)] // ASCII special characters are not alphanumeric
    #[case("\n\t\r", false)] // ASCII control characters are not alphanumeric
    #[case("\\", false)] // backslash is not alphanumeric
    #[case("\"'", false)] // quotes are not alphanumeric
    fn test_is_ascii_alphanumeric_constraint(
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let constraint = IsAsciiAlphanumeric;
        assert_eq!(constraint.check(&input.to_string()), expected);
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_error_message() {
        let constraint = IsAsciiAlphanumeric;
        assert_eq!(
            <IsAsciiAlphanumeric as Constraint<String>>::error_msg(&constraint),
            "must contain only ascii alphanumeric characters"
        );
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_with_ascii_letters() {
        let constraint = IsAsciiAlphanumeric;

        assert!(constraint.check(&"abcdefghijklmnopqrstuvwxyz".to_string()));
        assert!(constraint.check(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()));
        assert!(constraint.check(&"AbCdEf".to_string()));
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_with_ascii_numbers() {
        let constraint = IsAsciiAlphanumeric;

        assert!(constraint.check(&"0123456789".to_string()));
        assert!(constraint.check(&"123456".to_string()));
        assert!(constraint.check(&"0".to_string()));
        assert!(constraint.check(&"999".to_string()));
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_with_mixed_ascii() {
        let constraint = IsAsciiAlphanumeric;

        assert!(constraint.check(&"hello123".to_string()));
        assert!(constraint.check(&"ABC123def".to_string()));
        assert!(constraint.check(&"test2023".to_string()));
        assert!(constraint.check(&"user1password2".to_string()));
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_with_special_chars() {
        let constraint = IsAsciiAlphanumeric;

        // ASCII punctuation and symbols should fail
        assert!(
            !constraint
                .check(&"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".to_string())
        );
        assert!(!constraint.check(&" ".to_string())); // space
        assert!(!constraint.check(&"\t".to_string())); // tab
        assert!(!constraint.check(&"\n".to_string())); // newline
        assert!(!constraint.check(&"\r".to_string())); // carriage return

        // Common separators should fail
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
    fn test_is_ascii_alphanumeric_with_non_ascii_unicode() {
        let constraint = IsAsciiAlphanumeric;

        // Extended Latin should fail (not ASCII)
        assert!(!constraint.check(&"ñoño".to_string())); // Spanish
        assert!(!constraint.check(&"naïve".to_string())); // French
        assert!(!constraint.check(&"ü".to_string())); // German umlaut
        assert!(!constraint.check(&"ç".to_string())); // cedilla

        // Non-Latin scripts should fail
        assert!(!constraint.check(&"Москва".to_string())); // Russian
        assert!(!constraint.check(&"東京".to_string())); // Japanese
        assert!(!constraint.check(&"서울".to_string())); // Korean
        assert!(!constraint.check(&"αβγ".to_string())); // Greek

        // Unicode numbers should fail (not ASCII)
        assert!(!constraint.check(&"٠١٢٣".to_string())); // Arabic numerals
        assert!(!constraint.check(&"零一二三".to_string())); // Chinese numerals

        // Emoji should fail
        assert!(!constraint.check(&"🚀".to_string()));
        assert!(!constraint.check(&"hello🎉".to_string()));
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_mixed_content() {
        let constraint = IsAsciiAlphanumeric;

        // Mixed ASCII alphanumeric with non-ASCII should fail
        assert!(!constraint.check(&"hello café".to_string()));
        assert!(!constraint.check(&"test 🚀 rocket".to_string()));
        assert!(!constraint.check(&"naïve123".to_string()));
        assert!(!constraint.check(&"user_name".to_string())); // underscore is not alphanumeric

        // Mixed ASCII alphanumeric with ASCII special chars should fail
        assert!(!constraint.check(&"user@example.com".to_string()));
        assert!(!constraint.check(&"hello-world".to_string()));
        assert!(!constraint.check(&"test.file".to_string()));
        assert!(!constraint.check(&"password!123".to_string()));

        // Pure ASCII alphanumeric should pass
        assert!(constraint.check(&"helloworld".to_string()));
        assert!(constraint.check(&"test123".to_string()));
        assert!(constraint.check(&"UserName123".to_string()));
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_boundary_values() {
        let constraint = IsAsciiAlphanumeric;

        // Test ASCII boundary characters
        // '0' (48) and '9' (57) - ASCII digits
        assert!(constraint.check(&"0".to_string()));
        assert!(constraint.check(&"9".to_string()));

        // 'A' (65) and 'Z' (90) - ASCII uppercase letters
        assert!(constraint.check(&"A".to_string()));
        assert!(constraint.check(&"Z".to_string()));

        // 'a' (97) and 'z' (122) - ASCII lowercase letters
        assert!(constraint.check(&"a".to_string()));
        assert!(constraint.check(&"z".to_string()));

        // Characters just outside alphanumeric ranges
        assert!(!constraint.check(&"/".to_string())); // '/' (47) - just before '0'
        assert!(!constraint.check(&":".to_string())); // ':' (58) - just after '9'
        assert!(!constraint.check(&"@".to_string())); // '@' (64) - just before 'A'
        assert!(!constraint.check(&"[".to_string())); // '[' (91) - just after 'Z'
        assert!(!constraint.check(&"`".to_string())); // '`' (96) - just before 'a'
        assert!(!constraint.check(&"{".to_string())); // '{' (123) - just after 'z'
    }

    #[rstest]
    fn test_is_ascii_alphanumeric_empty_and_whitespace() {
        let constraint = IsAsciiAlphanumeric;

        // Empty string should pass (vacuously true)
        assert!(constraint.check(&String::new()));

        // Various whitespace should fail
        assert!(!constraint.check(&" ".to_string())); // space
        assert!(!constraint.check(&"  ".to_string())); // multiple spaces
        assert!(!constraint.check(&"\t".to_string())); // tab
        assert!(!constraint.check(&"\n".to_string())); // newline
        assert!(!constraint.check(&"\r".to_string())); // carriage return
        assert!(!constraint.check(&"\r\n".to_string())); // CRLF
        assert!(!constraint.check(&" \t\n".to_string())); // mixed whitespace
    }
}
