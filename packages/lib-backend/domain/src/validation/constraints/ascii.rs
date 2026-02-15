use bon::Builder;

use crate::validation::constraints::Constraint;

#[derive(Builder)]
#[builder(derive(Clone), start_fn = with_err)]
pub struct IsAscii<T>
where
    T: ToString,
{
    #[builder(start_fn)]
    err_fn: fn(&T) -> String,
}

impl<T> Constraint<T> for IsAscii<T>
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().is_ascii()
    }

    fn error_msg(&self, rejected_value: &T) -> String {
        (self.err_fn)(rejected_value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::IsAscii;
    use crate::validation::constraints::Constraint as _;

    fn err(_: &String) -> String {
        "must contain only ascii characters".to_string()
    }

    #[fixture]
    fn constraint() -> IsAscii<String> {
        IsAscii::with_err(err).build()
    }

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
    #[case("abcdefghijklmnopqrstuvwxyz", true)] // lowercase letters
    #[case("ABCDEFGHIJKLMNOPQRSTUVWXYZ", true)] // uppercase letters
    #[case("0123456789", true)] // digits
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
    #[case("hello!@#$%^&*()", true)] // ASCII special characters
    #[case("123.45", true)] // ASCII numbers with decimal
    #[case("a1b2c3", true)] // mixed ASCII letters and numbers
    #[case("\n\t\r", true)] // ASCII control characters
    #[case("\\", true)] // backslash is ASCII
    #[case("\"'", true)] // quotes are ASCII
    #[case("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~", true)] // ASCII punctuation
    #[case(" ", true)] // space
    #[case("\t", true)] // tab
    #[case("\n", true)] // newline
    #[case("\r", true)] // carriage return
    #[case("hello world", true)] // pure ASCII
    #[case("test rocket", true)] // pure ASCII
    #[case("price: $10", true)] // ASCII currency
    #[case("naive approach", true)] // pure ASCII
    #[case("café", false)] // é is not ASCII
    #[case("naïve", false)] // ï is not ASCII
    #[case("αβγ", false)] // Greek letters are not ASCII
    #[case("中文", false)] // Chinese characters are not ASCII
    #[case("🚀", false)] // emoji is not ASCII
    #[case("Москва", false)] // Cyrillic is not ASCII
    #[case("東京", false)] // Japanese is not ASCII
    #[case("서울", false)] // Korean is not ASCII
    #[case("ñ", false)] // ñ (U+00F1)
    #[case("ü", false)] // ü (U+00FC)
    #[case("ç", false)] // ç (U+00E7)
    #[case("€", false)] // Euro symbol
    #[case("£", false)] // Pound symbol
    #[case("¥", false)] // Yen symbol
    #[case("∑", false)] // Summation symbol
    #[case("∞", false)] // Infinity symbol
    #[case("≠", false)] // Not equal symbol
    #[case("→", false)] // Right arrow
    #[case("←", false)] // Left arrow
    #[case("↑", false)] // Up arrow
    #[case("hello café", false)] // mixed ASCII and non-ASCII
    #[case("test 🚀 rocket", false)] // with emoji
    #[case("price: 10€", false)] // with currency symbol
    #[case("naïve approach", false)] // with accented character
    fn is_ascii_constraint(
        constraint: IsAscii<String>,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input.to_string()), expected);
    }

    #[rstest]
    fn is_ascii_error_message(constraint: IsAscii<String>) {
        let value = "🦀".into();
        assert_eq!(constraint.error_msg(&value), err(&value));
    }

    #[rstest]
    fn is_ascii_boundary_values(constraint: IsAscii<String>) {
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
