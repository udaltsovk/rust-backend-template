use std::sync::Arc;

use derive_where::derive_where;

use crate::validation::error::ValidationErrors;

mod alphanumeric;
mod ascii;
mod ascii_alphanumeric;
mod email;
pub mod has;
pub mod length;
pub mod range;
mod regex;

pub use alphanumeric::IsAlphanumeric;
pub use ascii::IsAscii;
pub use ascii_alphanumeric::IsAsciiAlphanumeric;
pub use email::IsValidEmail;
pub use regex::Matches;

pub trait Constraint<T> {
    fn check(&self, value: &T) -> bool;

    fn error_msg(&self) -> String;
}

#[derive_where(Clone)]
pub struct ConstraintVec<T>(Vec<Arc<dyn Constraint<T> + Send + Sync>>);

impl<T> ConstraintVec<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_constraint(
        mut self,
        constraint: impl Constraint<T> + Send + Sync + 'static,
    ) -> Self {
        self.0.push(Arc::new(constraint));
        self
    }
}

impl<T> Default for ConstraintVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstraintsBuilder<T> {
    name: &'static str,
    constraints: ConstraintVec<T>,
}

impl<T> ConstraintsBuilder<T> {
    pub const fn new(name: &'static str) -> Self {
        Self::new_with_constraints(name, ConstraintVec::new())
    }

    pub const fn new_with_constraints(
        name: &'static str,
        constraints: ConstraintVec<T>,
    ) -> Self {
        Self {
            name,
            constraints,
        }
    }

    pub fn add_constraint(
        mut self,
        constraint: impl Constraint<T> + Send + Sync + 'static,
    ) -> Self {
        self.constraints = self.constraints.add_constraint(constraint);
        self
    }

    pub fn build(self) -> Constraints<T> {
        Constraints {
            name: self.name,
            constraints: self.constraints,
        }
    }
}

pub struct Constraints<T> {
    name: &'static str,
    constraints: ConstraintVec<T>,
}

impl<T> Constraints<T> {
    pub fn builder(name: &'static str) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new(name)
    }

    pub fn builder_with(
        name: &'static str,
        constraints: &ConstraintVec<T>,
    ) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new_with_constraints(name, constraints.clone())
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn derived(name: &'static str, source: &Constraints<T>) -> Self {
        Self {
            name,
            constraints: source.constraints.clone(),
        }
    }

    pub fn check(&self, value: &T) -> ValidationErrors {
        let mut errors = ValidationErrors::new();

        for constraint in &self.constraints.0 {
            if !constraint.check(value) {
                let message = constraint.error_msg();
                errors.push(self.name, message);
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::{Constraint, Constraints, ConstraintsBuilder};

    // Test constraint implementation
    struct TestConstraint {
        should_pass: bool,
        message: String,
    }

    impl<T> Constraint<T> for TestConstraint {
        fn check(&self, _value: &T) -> bool {
            self.should_pass
        }

        fn error_msg(&self) -> String {
            self.message.clone()
        }
    }

    #[fixture]
    fn passing_constraint() -> TestConstraint {
        TestConstraint {
            should_pass: true,
            message: "error1".to_string(),
        }
    }

    #[fixture]
    fn failing_constraint() -> TestConstraint {
        TestConstraint {
            should_pass: false,
            message: "error2".to_string(),
        }
    }

    #[fixture]
    fn test_field_name() -> &'static str {
        "test_field"
    }

    #[fixture]
    fn test_value() -> String {
        "test_value".to_string()
    }

    #[rstest]
    #[case("test_field")]
    #[case("my_field")]
    #[case("username")]
    fn constraints_builder_new(#[case] field_name: &'static str) {
        let builder = ConstraintsBuilder::<String>::new(field_name);
        assert_eq!(builder.name, field_name);
        assert!(builder.constraints.0.is_empty());
    }

    #[rstest]
    fn constraints_builder_add_constraint(
        test_field_name: &'static str,
        passing_constraint: TestConstraint,
    ) {
        let builder = ConstraintsBuilder::<String>::new(test_field_name)
            .add_constraint(passing_constraint);

        assert_eq!(builder.name, test_field_name);
        assert_eq!(builder.constraints.0.len(), 1);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(5)]
    fn constraints_builder_multiple_constraints(
        test_field_name: &'static str,
        #[case] constraint_count: usize,
    ) {
        let mut builder = ConstraintsBuilder::<String>::new(test_field_name);

        for i in 0..constraint_count {
            let constraint = TestConstraint {
                should_pass: i % 2 == 0,
                message: format!("message{}", i),
            };
            builder = builder.add_constraint(constraint);
        }

        assert_eq!(builder.constraints.0.len(), constraint_count);
    }

    #[rstest]
    fn constraints_builder_build(
        test_field_name: &'static str,
        passing_constraint: TestConstraint,
    ) {
        let constraints = ConstraintsBuilder::<String>::new(test_field_name)
            .add_constraint(passing_constraint)
            .build();

        assert_eq!(constraints.name(), test_field_name);
        assert_eq!(constraints.constraints.0.len(), 1);
    }

    #[rstest]
    fn constraints_builder_method(test_field_name: &'static str) {
        let constraints =
            Constraints::<String>::builder(test_field_name).build();
        assert_eq!(constraints.name(), test_field_name);
        assert!(constraints.constraints.0.is_empty());
    }

    #[rstest]
    #[case("my_field")]
    #[case("username")]
    #[case("email")]
    fn constraints_name(#[case] field_name: &'static str) {
        let constraints = Constraints::<String>::builder(field_name).build();
        assert_eq!(constraints.name(), field_name);
    }

    #[rstest]
    fn constraints_check_all_pass(
        test_field_name: &'static str,
        test_value: String,
        passing_constraint: TestConstraint,
    ) {
        let constraint2 = TestConstraint {
            should_pass: true,
            message: "error2".to_string(),
        };

        let constraints = Constraints::<String>::builder(test_field_name)
            .add_constraint(passing_constraint)
            .add_constraint(constraint2)
            .build();

        let errors = constraints.check(&test_value);
        assert!(errors.into_inner().is_empty());
    }

    #[rstest]
    fn constraints_check_some_fail(
        test_field_name: &'static str,
        test_value: String,
        passing_constraint: TestConstraint,
        failing_constraint: TestConstraint,
    ) {
        let constraints = Constraints::<String>::builder(test_field_name)
            .add_constraint(passing_constraint)
            .add_constraint(failing_constraint)
            .build();

        let errors = constraints.check(&test_value);
        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 1);
        assert_eq!(error_list[0].0, test_field_name);
        assert_eq!(error_list[0].1, "error2");
    }

    #[rstest]
    fn constraints_check_all_fail(
        test_field_name: &'static str,
        test_value: String,
        failing_constraint: TestConstraint,
    ) {
        let constraint2 = TestConstraint {
            should_pass: false,
            message: "error1".to_string(),
        };

        let constraints = Constraints::<String>::builder(test_field_name)
            .add_constraint(constraint2)
            .add_constraint(failing_constraint)
            .build();

        let errors = constraints.check(&test_value);
        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 2);
        assert_eq!(error_list[0].0, test_field_name);
        assert_eq!(error_list[0].1, "error1");
        assert_eq!(error_list[1].0, test_field_name);
        assert_eq!(error_list[1].1, "error2");
    }

    #[rstest]
    fn constraints_check_empty(
        test_field_name: &'static str,
        test_value: String,
    ) {
        let constraints =
            Constraints::<String>::builder(test_field_name).build();

        let errors = constraints.check(&test_value);
        assert!(errors.into_inner().is_empty());
    }

    #[test]
    fn constraints_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Constraints<String>>();
        assert_send_sync::<ConstraintsBuilder<String>>();
    }

    #[rstest]
    fn constraints_works_with_integers() {
        struct IntConstraint;
        impl Constraint<i32> for IntConstraint {
            fn check(&self, value: &i32) -> bool {
                *value > 0
            }

            fn error_msg(&self) -> String {
                "must be positive".to_string()
            }
        }

        let constraints = ConstraintsBuilder::new("age")
            .add_constraint(IntConstraint)
            .build();

        let errors = constraints.check(&10);
        assert!(errors.into_inner().is_empty());

        let errors = constraints.check(&-5);
        assert!(!errors.into_inner().is_empty());
    }
}
