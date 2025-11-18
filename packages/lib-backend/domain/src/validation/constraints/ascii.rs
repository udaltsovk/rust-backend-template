use crate::validation::constraints::Constraint;

pub struct IsAscii;

impl<T> Constraint<T> for IsAscii
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().is_ascii()
    }

    fn error_msg(&self) -> String {
        "must contain only ascii characters".to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::IsAscii;
    use crate::validation::constraints::Constraint;

    #[rstest]
    #[case("hello", true)]
    #[case("HELLO", true)]
    #[case("hello123", true)]
    #[case("Hello World", true)]
    #[case("123", true)]
    #[case("abc", true)]
    #[case("ABC", true)]
    #[case("hello world", true)]
    #[case("hello-world", true)]
    #[case("hello_world", true)]
    #[case("hello.world", true)]
    #[case("hello@example.com", true)]
    #[case("", true)] // empty string is ASCII
    #[case("café", false)] // é is not ASCII
    #[case("naïve", false)] // ï is not ASCII
    #[case("αβγ", false)] // Greek letters are not ASCII
    #[case("中文", false)] // Chinese characters are not ASCII
    #[case("🚀", false)] // emoji is not ASCII
    #[case("Москва", false)] // Cyrillic is not ASCII
    #[case("東京", false)] // Japanese is not ASCII
    #[case("서울", false)] // Korean is not ASCII
    #[case("hello!@#$%^&*()", true)] // ASCII special characters
    #[case("123.45", true)] // ASCII numbers with decimal
    #[case("a1b2c3", true)] // mixed ASCII letters and numbers
    #[case("\n\t\r", true)] // ASCII control characters
    #[case("\\", true)] // backslash is ASCII
    #[case("\"'", true)] // quotes are ASCII
    fn test_is_ascii_constraint(#[case] input: &str, #[case] expected: bool) {
        let constraint = IsAscii;
        assert_eq!(constraint.check(&input.to_string()), expected);
    }

    #[rstest]
    fn test_is_ascii_error_message() {
        let constraint = IsAscii;
        assert_eq!(
            <IsAscii as Constraint<String>>::error_msg(&constraint),
            "must contain only ascii characters"
        );
    }

    #[rstest]
    fn test_is_ascii_with_basic_latin() {
        let constraint = IsAscii;

        // Basic Latin characters (0-127)
        assert!(constraint.check(&"abcdefghijklmnopqrstuvwxyz".to_string()));
        assert!(constraint.check(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()));
        assert!(constraint.check(&"0123456789".to_string()));
    }

    #[rstest]
    fn test_is_ascii_with_special_characters() {
        let constraint = IsAscii;

        // ASCII punctuation and symbols
        assert!(
            constraint.check(&"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".to_string())
        );
        assert!(constraint.check(&" ".to_string())); // space
        assert!(constraint.check(&"\t".to_string())); // tab
        assert!(constraint.check(&"\n".to_string())); // newline
        assert!(constraint.check(&"\r".to_string())); // carriage return
    }

    #[rstest]
    fn test_is_ascii_with_non_ascii_unicode() {
        let constraint = IsAscii;

        // Extended Latin
        assert!(!constraint.check(&"ñ".to_string())); // ñ (U+00F1)
        assert!(!constraint.check(&"ü".to_string())); // ü (U+00FC)
        assert!(!constraint.check(&"ç".to_string())); // ç (U+00E7)

        // Currency symbols
        assert!(!constraint.check(&"€".to_string())); // Euro
        assert!(!constraint.check(&"£".to_string())); // Pound
        assert!(!constraint.check(&"¥".to_string())); // Yen

        // Mathematical symbols
        assert!(!constraint.check(&"∑".to_string())); // Summation
        assert!(!constraint.check(&"∞".to_string())); // Infinity
        assert!(!constraint.check(&"≠".to_string())); // Not equal

        // Arrows and symbols
        assert!(!constraint.check(&"→".to_string())); // Right arrow
        assert!(!constraint.check(&"←".to_string())); // Left arrow
        assert!(!constraint.check(&"↑".to_string())); // Up arrow
    }

    #[rstest]
    fn test_is_ascii_mixed_content() {
        let constraint = IsAscii;

        // Mixed ASCII and non-ASCII should fail
        assert!(!constraint.check(&"hello café".to_string()));
        assert!(!constraint.check(&"test 🚀 rocket".to_string()));
        assert!(!constraint.check(&"price: 10€".to_string()));
        assert!(!constraint.check(&"naïve approach".to_string()));

        // Pure ASCII should pass
        assert!(constraint.check(&"hello world".to_string()));
        assert!(constraint.check(&"test rocket".to_string()));
        assert!(constraint.check(&"price: $10".to_string()));
        assert!(constraint.check(&"naive approach".to_string()));
    }

    #[rstest]
    fn test_is_ascii_boundary_values() {
        let constraint = IsAscii;

        // ASCII range is 0-127 (0x00-0x7F)
        let ascii_boundary = String::from("\u{007F}"); // DEL character (127)
        let non_ascii_start = String::from("\u{0080}"); // First non-ASCII character (128)

        assert!(constraint.check(&ascii_boundary));
        assert!(!constraint.check(&non_ascii_start));

        // Test null character (0)
        let null_char = String::from("\u{0000}");
        assert!(constraint.check(&null_char));
    }
}
