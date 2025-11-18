use std::{error::Error, fmt, marker::PhantomData};

use crate::validation::ValidationConfirmation;

#[derive(Clone, Debug)]
pub struct ValidationErrors(Vec<(String, String)>);

impl ValidationErrors {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn into_inner(self) -> Vec<(String, String)> {
        self.0
    }

    pub fn extend(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    pub fn push(&mut self, path: impl ToString, message: impl ToString) {
        self.0.push((path.to_string(), message.to_string()));
    }

    pub fn into_result<T, F>(self, ok_fn: F) -> Result<T, Self>
    where
        F: FnOnce(ValidationConfirmation) -> T,
    {
        let confirmation = ValidationConfirmation {
            _phantom: PhantomData,
        };
        self.0.is_empty().then(|| ok_fn(confirmation)).ok_or(self)
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors = self
            .0
            .iter()
            .map(|(path, err)| format!("{path}: {err}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("Validation errors: [\n{errors}\n]").fmt(f)
    }
}

impl Error for ValidationErrors {}

impl From<Vec<ValidationErrors>> for ValidationErrors {
    fn from(errors: Vec<ValidationErrors>) -> Self {
        errors.into_iter().enumerate().fold(
            ValidationErrors::default(),
            |mut acc, (i, e)| {
                let errors =
                    e.0.into_iter()
                        .map(|(path, error)| (format!("{i}.{path}"), error))
                        .collect();
                acc.extend(ValidationErrors(errors));
                acc
            },
        )
    }
}

impl FromIterator<ValidationErrors> for ValidationErrors {
    fn from_iter<T: IntoIterator<Item = ValidationErrors>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ValidationErrors;

    #[rstest]
    fn test_validation_errors_new() {
        let errors = ValidationErrors::new();
        assert!(errors.0.is_empty());
    }

    #[rstest]
    fn test_validation_errors_default() {
        let errors = ValidationErrors::default();
        assert!(errors.0.is_empty());
    }

    #[rstest]
    fn test_validation_errors_push() {
        let mut errors = ValidationErrors::new();
        errors.push("field1", "error message 1");
        errors.push("field2", "error message 2");

        assert_eq!(errors.0.len(), 2);
        assert_eq!(
            errors.0[0],
            ("field1".to_string(), "error message 1".to_string())
        );
        assert_eq!(
            errors.0[1],
            ("field2".to_string(), "error message 2".to_string())
        );
    }

    #[rstest]
    fn test_validation_errors_into_inner() {
        let mut errors = ValidationErrors::new();
        errors.push("test_field", "test error");

        let inner = errors.into_inner();
        assert_eq!(inner.len(), 1);
        assert_eq!(
            inner[0],
            ("test_field".to_string(), "test error".to_string())
        );
    }

    #[rstest]
    fn test_validation_errors_extend() {
        let mut errors1 = ValidationErrors::new();
        errors1.push("field1", "error1");

        let mut errors2 = ValidationErrors::new();
        errors2.push("field2", "error2");
        errors2.push("field3", "error3");

        errors1.extend(errors2);

        assert_eq!(errors1.0.len(), 3);
        assert_eq!(errors1.0[0], ("field1".to_string(), "error1".to_string()));
        assert_eq!(errors1.0[1], ("field2".to_string(), "error2".to_string()));
        assert_eq!(errors1.0[2], ("field3".to_string(), "error3".to_string()));
    }

    #[rstest]
    fn test_validation_errors_into_result_success() {
        let errors = ValidationErrors::new(); // Empty errors

        let result = errors.into_result(|_confirmation| "success");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[rstest]
    fn test_validation_errors_into_result_failure() {
        let mut errors = ValidationErrors::new();
        errors.push("field", "error");

        let result = errors.into_result(|_confirmation| "success");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0.len(), 1);
        assert_eq!(err.0[0], ("field".to_string(), "error".to_string()));
    }

    #[rstest]
    fn test_validation_errors_display() {
        let mut errors = ValidationErrors::new();
        errors.push("field1", "error message 1");
        errors.push("field2", "error message 2");

        let display_str = format!("{}", errors);

        assert!(display_str.contains("Validation errors:"));
        assert!(display_str.contains("field1: error message 1"));
        assert!(display_str.contains("field2: error message 2"));
    }

    #[rstest]
    fn test_validation_errors_display_empty() {
        let errors = ValidationErrors::new();
        let display_str = format!("{}", errors);

        assert_eq!(display_str, "Validation errors: [\n\n]");
    }

    #[rstest]
    fn test_validation_errors_from_vec() {
        let mut errors1 = ValidationErrors::new();
        errors1.push("field1", "error1");

        let mut errors2 = ValidationErrors::new();
        errors2.push("field2", "error2");

        let mut errors3 = ValidationErrors::new();
        errors3.push("field3", "error3");

        let combined_errors =
            ValidationErrors::from(vec![errors1, errors2, errors3]);

        assert_eq!(combined_errors.0.len(), 3);
        assert_eq!(
            combined_errors.0[0],
            ("0.field1".to_string(), "error1".to_string())
        );
        assert_eq!(
            combined_errors.0[1],
            ("1.field2".to_string(), "error2".to_string())
        );
        assert_eq!(
            combined_errors.0[2],
            ("2.field3".to_string(), "error3".to_string())
        );
    }

    #[rstest]
    fn test_validation_errors_from_iter() {
        let mut errors1 = ValidationErrors::new();
        errors1.push("field1", "error1");

        let mut errors2 = ValidationErrors::new();
        errors2.push("field2", "error2");

        let combined_errors: ValidationErrors =
            vec![errors1, errors2].into_iter().collect();

        assert_eq!(combined_errors.0.len(), 2);
        assert_eq!(
            combined_errors.0[0],
            ("0.field1".to_string(), "error1".to_string())
        );
        assert_eq!(
            combined_errors.0[1],
            ("1.field2".to_string(), "error2".to_string())
        );
    }

    #[rstest]
    fn test_validation_errors_from_empty_vec() {
        let combined_errors =
            ValidationErrors::from(vec![] as Vec<ValidationErrors>);
        assert!(combined_errors.0.is_empty());
    }

    #[rstest]
    fn test_validation_errors_multiple_errors_in_single_validation() {
        let mut errors1 = ValidationErrors::new();
        errors1.push("field1", "error1");
        errors1.push("field1", "error2"); // Same field, multiple errors

        let mut errors2 = ValidationErrors::new();
        errors2.push("field2", "error3");

        let combined_errors = ValidationErrors::from(vec![errors1, errors2]);

        assert_eq!(combined_errors.0.len(), 3);
        assert_eq!(
            combined_errors.0[0],
            ("0.field1".to_string(), "error1".to_string())
        );
        assert_eq!(
            combined_errors.0[1],
            ("0.field1".to_string(), "error2".to_string())
        );
        assert_eq!(
            combined_errors.0[2],
            ("1.field2".to_string(), "error3".to_string())
        );
    }

    #[rstest]
    fn test_validation_errors_clone() {
        let mut errors = ValidationErrors::new();
        errors.push("field", "error");

        let cloned_errors = errors.clone();

        assert_eq!(errors.0.len(), cloned_errors.0.len());
        assert_eq!(errors.0[0], cloned_errors.0[0]);
    }

    #[rstest]
    fn test_validation_errors_debug() {
        let mut errors = ValidationErrors::new();
        errors.push("field", "error");

        let debug_str = format!("{:?}", errors);

        assert!(debug_str.contains("ValidationErrors"));
        assert!(debug_str.contains("field"));
        assert!(debug_str.contains("error"));
    }
}
