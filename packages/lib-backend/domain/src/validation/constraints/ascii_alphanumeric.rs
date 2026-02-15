use bon::Builder;

use crate::validation::constraints::Constraint;

#[derive(Builder)]
#[builder(derive(Clone), start_fn = with_err)]
pub struct IsAsciiAlphanumeric<T>
where
    T: ToString,
{
    #[builder(start_fn)]
    err_fn: fn(&T) -> String,
}

impl<T> Constraint<T> for IsAsciiAlphanumeric<T>
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn error_msg(&self, rejected_value: &T) -> String {
        (self.err_fn)(rejected_value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::IsAsciiAlphanumeric;
    use crate::validation::constraints::Constraint as _;

    fn err(_: &String) -> String {
        "must contain only ascii alphanumeric characters".to_string()
    }

    #[fixture]
    fn constraint() -> IsAsciiAlphanumeric<String> {
        IsAsciiAlphanumeric::with_err(err).build()
    }

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
    #[case("abcdefghijklmnopqrstuvwxyz", true)] // lowercase letters
    #[case("ABCDEFGHIJKLMNOPQRSTUVWXYZ", true)] // uppercase letters
    #[case("AbCdEf", true)] // mixed case letters
    #[case("0123456789", true)] // all digits
    #[case("123456", true)] // some digits
    #[case("0", true)] // single digit
    #[case("999", true)] // repeated digits
    #[case("hello123", true)] // mixed letters and numbers
    #[case("ABC123def", true)] // mixed case with numbers
    #[case("test2023", true)] // letters with year
    #[case("user1password2", true)] // complex alphanumeric
    #[case("helloworld", true)] // pure ASCII alphanumeric
    #[case("test123", true)] // pure ASCII alphanumeric
    #[case("UserName123", true)] // pure ASCII alphanumeric mixed case
    #[case("", true)] // empty string is vacuously true
    #[case("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~", false)] // ASCII punctuation
    #[case(" ", false)] // space
    #[case("\t", false)] // tab
    #[case("\n", false)] // newline
    #[case("\r", false)] // carriage return
    #[case("hello world", false)] // space separator
    #[case("hello,world", false)] // comma separator
    #[case("hello;world", false)] // semicolon separator
    #[case("hello:world", false)] // colon separator
    #[case("hello/world", false)] // slash separator
    #[case("hello\\world", false)] // backslash separator
    #[case("hello|world", false)] // pipe separator
    #[case("hello&world", false)] // ampersand separator
    #[case("hello*world", false)] // asterisk separator
    #[case("hello+world", false)] // plus separator
    #[case("hello=world", false)] // equals separator
    #[case("hello(world)", false)] // parentheses
    #[case("hello[world]", false)] // brackets
    #[case("hello{world}", false)] // braces
    #[case("hello<world>", false)] // angle brackets
    #[case("ñoño", false)] // Spanish characters
    #[case("naïve", false)] // French characters
    #[case("ü", false)] // German umlaut
    #[case("ç", false)] // cedilla
    #[case("Москва", false)] // Russian
    #[case("東京", false)] // Japanese
    #[case("서울", false)] // Korean
    #[case("αβγ", false)] // Greek
    #[case("٠١٢٣", false)] // Arabic numerals
    #[case("零一二三", false)] // Chinese numerals
    #[case("🚀", false)] // emoji
    #[case("hello🎉", false)] // text with emoji
    #[case("hello café", false)] // mixed ASCII and non-ASCII
    #[case("test 🚀 rocket", false)] // text with emoji and space
    #[case("naïve123", false)] // non-ASCII with numbers
    #[case("user_name", false)] // underscore is not alphanumeric
    #[case("user@example.com", false)] // email format
    #[case("hello-world", false)] // hyphen separator
    #[case("test.file", false)] // dot separator
    #[case("password!123", false)] // exclamation mark
    fn is_ascii_alphanumeric_constraint(
        constraint: IsAsciiAlphanumeric<String>,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input.to_string()), expected);
    }

    #[rstest]
    fn is_ascii_alphanumeric_error_message(
        constraint: IsAsciiAlphanumeric<String>,
    ) {
        let value = "🦀".into();
        assert_eq!(constraint.error_msg(&value), err(&value));
    }

    #[rstest]
    fn is_ascii_alphanumeric_boundary_values(
        constraint: IsAsciiAlphanumeric<String>,
    ) {
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
    fn is_ascii_alphanumeric_empty_and_whitespace(
        constraint: IsAsciiAlphanumeric<String>,
    ) {
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
