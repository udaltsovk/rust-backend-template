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
    use rstest::{fixture, rstest};

    use super::ValidationErrors;

    #[fixture]
    fn validation_errors() -> ValidationErrors {
        ValidationErrors::new()
    }

    #[fixture]
    fn validation_errors_with_data() -> ValidationErrors {
        let mut errors = ValidationErrors::new();
        errors.push("field1", "error message 1");
        errors.push("field2", "error message 2");
        errors
    }

    #[fixture]
    fn single_error() -> ValidationErrors {
        let mut errors = ValidationErrors::new();
        errors.push("test_field", "test error");
        errors
    }

    #[rstest]
    #[case(ValidationErrors::new())] // new constructor
    #[case(ValidationErrors::default())] // default constructor
    fn validation_errors_constructors(#[case] errors: ValidationErrors) {
        assert!(errors.0.is_empty());
    }

    #[rstest]
    #[case("field1", "error message 1")]
    #[case("username", "must be valid")]
    #[case("email", "invalid format")]
    fn validation_errors_push(
        mut validation_errors: ValidationErrors,
        #[case] field: &str,
        #[case] message: &str,
    ) {
        validation_errors.push(field, message);

        assert_eq!(validation_errors.0.len(), 1);
        assert_eq!(
            validation_errors.0[0],
            (field.to_string(), message.to_string())
        );
    }

    #[rstest]
    fn validation_errors_into_inner(single_error: ValidationErrors) {
        let inner = single_error.into_inner();
        assert_eq!(inner.len(), 1);
        assert_eq!(
            inner[0],
            ("test_field".to_string(), "test error".to_string())
        );
    }

    #[rstest]
    fn validation_errors_extend(mut single_error: ValidationErrors) {
        let mut errors2 = ValidationErrors::new();
        errors2.push("field2", "error2");
        errors2.push("field3", "error3");

        single_error.extend(errors2);

        assert_eq!(single_error.0.len(), 3);
        assert_eq!(
            single_error.0[0],
            ("test_field".to_string(), "test error".to_string())
        );
        assert_eq!(
            single_error.0[1],
            ("field2".to_string(), "error2".to_string())
        );
        assert_eq!(
            single_error.0[2],
            ("field3".to_string(), "error3".to_string())
        );
    }

    #[rstest]
    fn validation_errors_into_result_success(
        validation_errors: ValidationErrors,
    ) {
        let result = validation_errors.into_result(|_confirmation| "success");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[rstest]
    fn validation_errors_into_result_failure(single_error: ValidationErrors) {
        let result = single_error.into_result(|_confirmation| "success");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0.len(), 1);
        assert_eq!(
            err.0[0],
            ("test_field".to_string(), "test error".to_string())
        );
    }

    #[rstest]
    fn validation_errors_display(
        validation_errors_with_data: ValidationErrors,
    ) {
        let display_str = format!("{}", validation_errors_with_data);

        assert!(display_str.contains("Validation errors:"));
        assert!(display_str.contains("field1: error message 1"));
        assert!(display_str.contains("field2: error message 2"));
    }

    #[rstest]
    fn validation_errors_display_empty(validation_errors: ValidationErrors) {
        let display_str = format!("{}", validation_errors);

        assert_eq!(display_str, "Validation errors: [\n\n]");
    }

    #[rstest]
    fn validation_errors_from_vec() {
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
    fn validation_errors_from_iter() {
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
    fn validation_errors_from_empty_vec() {
        let combined_errors =
            ValidationErrors::from(vec![] as Vec<ValidationErrors>);
        assert!(combined_errors.0.is_empty());
    }

    #[rstest]
    fn validation_errors_multiple_errors_in_single_validation() {
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
    fn validation_errors_clone(single_error: ValidationErrors) {
        let cloned_errors = single_error.clone();

        assert_eq!(single_error.0.len(), cloned_errors.0.len());
        assert_eq!(single_error.0[0], cloned_errors.0[0]);
    }

    #[rstest]
    fn validation_errors_debug(single_error: ValidationErrors) {
        let debug_str = format!("{:?}", single_error);

        assert!(debug_str.contains("ValidationErrors"));
        assert!(debug_str.contains("test_field"));
        assert!(debug_str.contains("test error"));
    }
}
