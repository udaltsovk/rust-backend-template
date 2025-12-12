use domain::validation::error::ValidationErrors;

pub trait ParseableJson<T> {
    fn parse(self) -> Result<T, ValidationErrors>;
}

impl<J, T> ParseableJson<Vec<T>> for Vec<J>
where
    J: ParseableJson<T>,
{
    fn parse(self) -> Result<Vec<T>, ValidationErrors> {
        let (errors, converted): (Vec<_>, Vec<_>) = self
            .into_iter()
            .map(|v| match v.parse() {
                Ok(ok) => (None, Some(ok)),
                Err(err) => (Some(err), None),
            })
            .unzip();
        errors
            .into_iter()
            .flatten()
            .collect::<ValidationErrors>()
            .into_result(|_| converted.into_iter().flatten().collect())
    }
}

#[cfg(test)]
mod tests {
    use domain::validation::error::ValidationErrors;
    use rstest::rstest;

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestItem {
        value: String,
    }

    impl ParseableJson<TestItem> for String {
        fn parse(self) -> Result<TestItem, ValidationErrors> {
            if self.is_empty() {
                let mut errors = ValidationErrors::new();
                errors.push("value", "cannot be empty");
                Err(errors)
            } else {
                Ok(TestItem {
                    value: self,
                })
            }
        }
    }

    #[rstest]
    fn parse_empty_vec_returns_empty_vec() {
        let input: Vec<String> = vec![];
        let result = input.parse();

        assert!(result.is_ok());
        let parsed: Vec<TestItem> =
            result.expect("Parse should succeed for empty vec");
        assert!(parsed.is_empty());
    }

    #[rstest]
    fn parse_vec_with_all_valid_items() {
        let input = vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ];
        let result = input.parse();

        assert!(result.is_ok());
        let parsed: Vec<TestItem> =
            result.expect("Parse should succeed for valid items");
        assert_eq!(parsed.len(), 3);
        assert_eq!(
            parsed.first().expect("First item should exist").value,
            "item1"
        );
        assert_eq!(
            parsed.get(1).expect("Second item should exist").value,
            "item2"
        );
        assert_eq!(
            parsed.get(2).expect("Third item should exist").value,
            "item3"
        );
    }

    #[rstest]
    fn parse_vec_with_all_invalid_items() {
        let input = vec![String::new(), String::new()];
        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        assert!(result.is_err());
        let errors = result.expect_err("Parse should fail for invalid items");
        assert_eq!(errors.into_inner().len(), 2);
    }

    #[rstest]
    fn parse_vec_with_mixed_valid_and_invalid_items() {
        let input = vec![
            "valid1".to_string(),
            String::new(),
            "valid2".to_string(),
            String::new(),
        ];
        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        assert!(result.is_err());
        let errors =
            result.expect_err("Parse should fail when some items are invalid");
        // Should have 2 validation errors for the empty strings
        assert_eq!(errors.into_inner().len(), 2);
    }

    #[rstest]
    fn parse_vec_with_single_valid_item() {
        let input = vec!["single_item".to_string()];
        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        assert!(result.is_ok());
        let parsed: Vec<TestItem> =
            result.expect("Parse should succeed for single valid item");
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed.first().expect("Item should exist").value,
            "single_item"
        );
    }

    #[rstest]
    fn parse_vec_with_single_invalid_item() {
        let input = vec![String::new()];
        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        assert!(result.is_err());
        let errors = result.expect_err("Parse should fail for invalid item");
        assert_eq!(errors.into_inner().len(), 1);
    }

    // Test that the implementation correctly collects all validation errors
    #[rstest]
    fn parse_vec_collects_all_validation_errors() {
        let input = vec![
            "valid".to_string(),
            String::new(),
            "also_valid".to_string(),
            String::new(),
            String::new(),
        ];
        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        assert!(result.is_err());
        let errors =
            result.expect_err("Parse should fail when some items are invalid");
        // Should collect exactly 3 validation errors for the 3 empty strings
        let inner_errors = errors.into_inner();
        assert_eq!(inner_errors.len(), 3);

        // Verify all errors are about the "value" field (with index prefixes from Vec parsing)
        for (field, message) in inner_errors {
            assert!(field.ends_with("value")); // Field names will be prefixed with indices like "1.value", "3.value", etc.
            assert_eq!(message, "cannot be empty");
        }
    }

    // Test edge case with large vector
    #[rstest]
    fn parse_large_vec_with_mixed_items() {
        let mut input = Vec::new();
        for i in 0_i32..100_i32 {
            if i % 2_i32 == 0_i32 {
                input.push(format!("item_{i}"));
            } else {
                input.push(String::new()); // Invalid
            }
        }

        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        assert!(result.is_err());
        let errors = result
            .expect_err("Parse should fail for mixed valid/invalid items");
        // Should have 50 validation errors (for odd indices)
        assert_eq!(errors.into_inner().len(), 50);
    }

    // Test that valid items are not included when there are validation errors
    #[rstest]
    fn parse_vec_returns_no_items_when_errors_exist() {
        let input = vec![
            "valid_item".to_string(),
            String::new(), // This will cause an error
            "another_valid_item".to_string(),
        ];

        let result: Result<Vec<TestItem>, ValidationErrors> = input.parse();

        // When there are validation errors, no items should be returned
        assert!(result.is_err());
        let errors =
            result.expect_err("Parse should fail when any item is invalid");
        assert_eq!(errors.into_inner().len(), 1);
    }

    // Test trait implementation for different types
    impl ParseableJson<i32> for String {
        fn parse(self) -> Result<i32, ValidationErrors> {
            str::parse::<i32>(&self).map_err(|_| {
                let mut errors = ValidationErrors::new();
                errors.push("number", "invalid number format");
                errors
            })
        }
    }

    #[rstest]
    fn parse_vec_works_with_different_types() {
        let input = vec!["42".to_string(), "100".to_string(), "7".to_string()];
        let result: Result<Vec<i32>, ValidationErrors> = input.parse();

        assert!(result.is_ok());
        let parsed = result.expect("Parse should succeed for valid numbers");
        assert_eq!(parsed, vec![42_i32, 100_i32, 7_i32]);
    }

    #[rstest]
    fn parse_vec_with_invalid_numbers() {
        let input = vec![
            "42".to_string(),
            "not_a_number".to_string(),
            "100".to_string(),
            "also_invalid".to_string(),
        ];
        let result: Result<Vec<i32>, ValidationErrors> = input.parse();

        assert!(result.is_err());
        let errors = result.expect_err("Parse should fail for invalid numbers");
        assert_eq!(errors.into_inner().len(), 2); // Two invalid number strings
    }
}
