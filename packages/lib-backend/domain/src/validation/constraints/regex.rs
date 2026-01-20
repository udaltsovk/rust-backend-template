use regex::Regex;

use crate::validation::constraints::Constraint;

pub struct Matches(pub Regex);

impl TryFrom<&str> for Matches {
    type Error = regex::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Regex::new(value).map(Self)
    }
}

impl<T> Constraint<T> for Matches
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        self.0.is_match(&value.to_string())
    }

    fn error_msg(&self) -> String {
        format!("must match pattern `{}`", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::Matches;
    use crate::validation::constraints::Constraint;

    #[fixture]
    fn phone_constraint() -> Matches {
        Matches::try_from(r"^\d{3}-\d{3}-\d{4}$").expect("valid phone regex")
    }

    #[fixture]
    fn email_constraint() -> Matches {
        Matches::try_from(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("valid email regex")
    }

    #[fixture]
    fn starts_with_test_constraint() -> Matches {
        Matches::try_from("^test").expect("valid starts with test regex")
    }

    #[fixture]
    fn ends_with_test_constraint() -> Matches {
        Matches::try_from("test$").expect("valid ends with test regex")
    }

    #[fixture]
    fn contains_test_constraint() -> Matches {
        Matches::try_from("test").expect("valid contains test regex")
    }

    #[fixture]
    fn digits_only_constraint() -> Matches {
        Matches::try_from(r"^\d+$").expect("valid digits only regex")
    }

    #[fixture]
    fn letters_only_constraint() -> Matches {
        Matches::try_from("^[a-zA-Z]+$").expect("valid letters only regex")
    }

    #[fixture]
    fn word_chars_constraint() -> Matches {
        Matches::try_from(r"^\w+$").expect("valid word chars regex")
    }

    #[fixture]
    fn case_sensitive_constraint() -> Matches {
        Matches::try_from("^Test$").expect("valid case sensitive regex")
    }

    #[fixture]
    fn case_insensitive_constraint() -> Matches {
        Matches::try_from("(?i)^test$").expect("valid case insensitive regex")
    }

    #[rstest]
    fn matches_constraint_phone_number(phone_constraint: Matches) {
        assert!(phone_constraint.check(&"123-456-7890".to_string()));
        assert!(phone_constraint.check(&"555-123-4567".to_string()));
        assert!(phone_constraint.check(&"000-000-0000".to_string()));

        assert!(!phone_constraint.check(&"invalid".to_string()));
        assert!(!phone_constraint.check(&"123-45-6789".to_string())); // wrong format
        assert!(!phone_constraint.check(&"1234-123-4567".to_string())); // too many digits in first group
        assert!(!phone_constraint.check(&"123-456-78901".to_string())); // too many digits in last group
        assert!(!phone_constraint.check(&"123 456 7890".to_string())); // spaces instead of hyphens
        assert!(!phone_constraint.check(&String::new())); // empty string
    }

    #[rstest]
    fn matches_constraint_email_pattern(email_constraint: Matches) {
        assert!(email_constraint.check(&"user@example.com".to_string()));
        assert!(
            email_constraint.check(&"test.email+tag@domain.org".to_string())
        );
        assert!(
            email_constraint.check(&"user123@test-domain.co.uk".to_string())
        );

        assert!(!email_constraint.check(&"invalid".to_string()));
        assert!(!email_constraint.check(&"user@".to_string()));
        assert!(!email_constraint.check(&"@domain.com".to_string()));
        assert!(!email_constraint.check(&"user.domain.com".to_string())); // missing @
        assert!(!email_constraint.check(&"user@domain".to_string())); // missing TLD
    }

    #[rstest]
    fn matches_constraint_simple_patterns(
        starts_with_test_constraint: Matches,
        ends_with_test_constraint: Matches,
        contains_test_constraint: Matches,
    ) {
        // Test starts with pattern
        assert!(starts_with_test_constraint.check(&"test123".to_string()));
        assert!(starts_with_test_constraint.check(&"testing".to_string()));
        assert!(!starts_with_test_constraint.check(&"123test".to_string()));

        // Test ends with pattern
        assert!(ends_with_test_constraint.check(&"123test".to_string()));
        assert!(ends_with_test_constraint.check(&"mytest".to_string()));
        assert!(!ends_with_test_constraint.check(&"test123".to_string()));

        // Test contains pattern
        assert!(contains_test_constraint.check(&"123test456".to_string()));
        assert!(contains_test_constraint.check(&"test".to_string()));
        assert!(contains_test_constraint.check(&"testing".to_string()));
        assert!(!contains_test_constraint.check(&"tst".to_string()));
    }

    #[rstest]
    fn matches_constraint_case_sensitivity(
        case_sensitive_constraint: Matches,
        case_insensitive_constraint: Matches,
    ) {
        assert!(case_sensitive_constraint.check(&"Test".to_string()));
        assert!(!case_sensitive_constraint.check(&"test".to_string()));
        assert!(!case_sensitive_constraint.check(&"TEST".to_string()));

        assert!(case_insensitive_constraint.check(&"test".to_string()));
        assert!(case_insensitive_constraint.check(&"Test".to_string()));
        assert!(case_insensitive_constraint.check(&"TEST".to_string()));
        assert!(case_insensitive_constraint.check(&"TeSt".to_string()));
    }

    #[rstest]
    fn matches_constraint_character_classes(
        digits_only_constraint: Matches,
        letters_only_constraint: Matches,
        word_chars_constraint: Matches,
    ) {
        // Digits only
        assert!(digits_only_constraint.check(&"123".to_string()));
        assert!(digits_only_constraint.check(&"0".to_string()));
        assert!(digits_only_constraint.check(&"999999".to_string()));
        assert!(!digits_only_constraint.check(&"123abc".to_string()));
        assert!(!digits_only_constraint.check(&String::new()));

        // Letters only
        assert!(letters_only_constraint.check(&"abc".to_string()));
        assert!(letters_only_constraint.check(&"ABC".to_string()));
        assert!(letters_only_constraint.check(&"AbC".to_string()));
        assert!(!letters_only_constraint.check(&"abc123".to_string()));
        assert!(!letters_only_constraint.check(&String::new()));

        // Word characters (letters, digits, underscore)
        assert!(word_chars_constraint.check(&"abc123".to_string()));
        assert!(word_chars_constraint.check(&"test_value".to_string()));
        assert!(word_chars_constraint.check(&"_underscore".to_string()));
        assert!(!word_chars_constraint.check(&"hello world".to_string())); // space is not \w
        assert!(!word_chars_constraint.check(&"hello-world".to_string())); // hyphen is not \w
    }

    #[rstest]
    fn matches_constraint_quantifiers() {
        // Optional character
        let optional = Matches::try_from("^colou?r$")
            .expect("valid optional quantifier regex");
        assert!(optional.check(&"color".to_string()));
        assert!(optional.check(&"colour".to_string()));
        assert!(!optional.check(&"colouur".to_string()));

        // One or more
        let one_or_more = Matches::try_from("^go+d$")
            .expect("valid one or more quantifier regex");
        assert!(one_or_more.check(&"god".to_string()));
        assert!(one_or_more.check(&"good".to_string()));
        assert!(one_or_more.check(&"goood".to_string()));
        assert!(!one_or_more.check(&"gd".to_string()));

        // Zero or more
        let zero_or_more = Matches::try_from("^go*d$")
            .expect("valid zero or more quantifier regex");
        assert!(zero_or_more.check(&"gd".to_string()));
        assert!(zero_or_more.check(&"god".to_string()));
        assert!(zero_or_more.check(&"good".to_string()));
        assert!(zero_or_more.check(&"goood".to_string()));

        // Exact count
        let exact_count = Matches::try_from(r"^\d{3}$")
            .expect("valid exact count quantifier regex");
        assert!(exact_count.check(&"123".to_string()));
        assert!(exact_count.check(&"000".to_string()));
        assert!(!exact_count.check(&"12".to_string()));
        assert!(!exact_count.check(&"1234".to_string()));

        // Range count
        let range_count = Matches::try_from(r"^\d{2,4}$")
            .expect("valid range count quantifier regex");
        assert!(range_count.check(&"12".to_string()));
        assert!(range_count.check(&"123".to_string()));
        assert!(range_count.check(&"1234".to_string()));
        assert!(!range_count.check(&"1".to_string()));
        assert!(!range_count.check(&"12345".to_string()));
    }

    #[rstest]
    fn matches_constraint_groups_and_alternatives() {
        // Alternatives
        let alternatives = Matches::try_from("^(cat|dog|bird)$")
            .expect("valid alternatives regex");
        assert!(alternatives.check(&"cat".to_string()));
        assert!(alternatives.check(&"dog".to_string()));
        assert!(alternatives.check(&"bird".to_string()));
        assert!(!alternatives.check(&"fish".to_string()));
        assert!(!alternatives.check(&"catdog".to_string()));

        // Non-capturing groups
        let non_capturing = Matches::try_from(r"^(?:Mr|Ms|Dr)\. [A-Z][a-z]+$")
            .expect("valid non-capturing groups regex");
        assert!(non_capturing.check(&"Mr. Smith".to_string()));
        assert!(non_capturing.check(&"Ms. Johnson".to_string()));
        assert!(non_capturing.check(&"Dr. Brown".to_string()));
        assert!(!non_capturing.check(&"Prof. Davis".to_string()));
        assert!(!non_capturing.check(&"Mr Smith".to_string())); // missing dot
    }

    #[rstest]
    fn matches_constraint_unicode() {
        let unicode_pattern = Matches::try_from(r"^[\p{L}\p{N}]+$")
            .expect("valid unicode pattern regex");

        // Should match Unicode letters and numbers
        assert!(unicode_pattern.check(&"café".to_string()));
        assert!(unicode_pattern.check(&"naïve".to_string()));
        assert!(unicode_pattern.check(&"Москва".to_string()));
        assert!(unicode_pattern.check(&"東京123".to_string()));
        assert!(unicode_pattern.check(&"서울456".to_string()));

        // Should not match punctuation or symbols
        assert!(!unicode_pattern.check(&"hello world".to_string()));
        assert!(!unicode_pattern.check(&"test@example.com".to_string()));
        assert!(!unicode_pattern.check(&"hello!".to_string()));
    }

    #[rstest]
    fn matches_constraint_error_message(phone_constraint: Matches) {
        let error_msg =
            <Matches as Constraint<String>>::error_msg(&phone_constraint);

        assert!(error_msg.contains("must match pattern"));
        assert!(error_msg.contains(r"^\d{3}-\d{3}-\d{4}$"));
        assert!(error_msg.len() > 20); // Should be a meaningful message
    }

    #[rstest]
    fn matches_try_from_valid_regex() {
        let result = Matches::try_from("^[a-z]+$");
        assert!(result.is_ok());

        let constraint = result.expect("valid regex");
        assert!(constraint.check(&"hello".to_string()));
        assert!(!constraint.check(&"Hello".to_string()));
    }

    #[rstest]
    fn matches_try_from_invalid_regex() {
        // Invalid regex patterns should return Err
        assert!(Matches::try_from("[").is_err()); // Unclosed bracket
        assert!(Matches::try_from("(").is_err()); // Unclosed parenthesis
        assert!(Matches::try_from("*").is_err()); // Invalid quantifier
        assert!(Matches::try_from("?").is_err()); // Invalid quantifier
        assert!(Matches::try_from("+").is_err()); // Invalid quantifier
        assert!(Matches::try_from("(?P<").is_err()); // Invalid named group
        assert!(Matches::try_from(r"\").is_err()); // Incomplete escape
    }

    #[rstest]
    fn matches_constraint_empty_string() {
        // Pattern that matches empty string
        let empty_allowed =
            Matches::try_from("^$").expect("valid empty allowed regex");
        assert!(empty_allowed.check(&String::new()));
        assert!(!empty_allowed.check(&"a".to_string()));

        // Pattern that matches empty or non-empty
        let optional_content =
            Matches::try_from("^.*$").expect("valid optional content regex");
        assert!(optional_content.check(&String::new()));
        assert!(optional_content.check(&"anything".to_string()));

        // Pattern that requires non-empty
        let non_empty_required =
            Matches::try_from("^.+$").expect("valid non-empty required regex");
        assert!(!non_empty_required.check(&String::new()));
        assert!(non_empty_required.check(&"a".to_string()));
    }

    #[rstest]
    fn matches_constraint_special_characters() {
        // Test escaping special regex characters
        let literal_dot = Matches::try_from(r"^hello\.world$")
            .expect("valid literal dot regex");
        assert!(literal_dot.check(&"hello.world".to_string()));
        assert!(!literal_dot.check(&"helloXworld".to_string()));

        // Test literal brackets
        let literal_brackets = Matches::try_from(r"^\[test\]$")
            .expect("valid literal brackets regex");
        assert!(literal_brackets.check(&"[test]".to_string()));
        assert!(!literal_brackets.check(&"test".to_string()));

        // Test literal parentheses
        let literal_parens = Matches::try_from(r"^\(test\)$")
            .expect("valid literal parentheses regex");
        assert!(literal_parens.check(&"(test)".to_string()));
        assert!(!literal_parens.check(&"test".to_string()));
    }

    #[rstest]
    fn matches_constraint_multiline() {
        // Single line mode (default)
        let single_line =
            Matches::try_from("^test$").expect("valid single line regex");
        assert!(single_line.check(&"test".to_string()));
        assert!(!single_line.check(&"test\nmore".to_string()));

        // Multiline mode
        let multiline =
            Matches::try_from("(?m)^test$").expect("valid multiline regex");
        assert!(multiline.check(&"test".to_string()));
        assert!(multiline.check(&"before\ntest\nafter".to_string()));
        assert!(!multiline.check(&"testing".to_string()));
    }
}
