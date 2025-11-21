use tap::Tap as _;

use crate::validation::error::ValidationErrors;

mod alphanumeric;
mod ascii;
mod ascii_alphanumeric;
pub mod length;
pub mod range;
mod regex;

pub use alphanumeric::IsAlphanumeric;
pub use ascii::IsAscii;
pub use ascii_alphanumeric::IsAsciiAlphanumeric;
pub use regex::Matches;

pub trait Constraint<T> {
    fn check(&self, value: &T) -> bool;

    fn error_msg(&self) -> String;
}

pub struct ConstraintsBuilder<T> {
    name: &'static str,
    constraints: Vec<Box<dyn Constraint<T> + Send + Sync>>,
}

impl<T> ConstraintsBuilder<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(
        mut self,
        constraint: impl Constraint<T> + Send + Sync + 'static,
    ) -> Self {
        self.constraints.push(Box::new(constraint));
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
    constraints: Vec<Box<dyn Constraint<T> + Send + Sync>>,
}

impl<T> Constraints<T> {
    pub fn builder(name: &'static str) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new(name)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn check(&self, value: &T) -> ValidationErrors {
        ValidationErrors::new().tap_mut(|errors| {
            self.constraints.iter().for_each(|constraint| {
                if constraint.check(value) {
                    return;
                }

                let message = constraint.error_msg();
                errors.push(self.name, message);
            });
        })
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
        assert!(builder.constraints.is_empty());
    }

    #[rstest]
    fn constraints_builder_add_constraint(
        test_field_name: &'static str,
        passing_constraint: TestConstraint,
    ) {
        let builder = ConstraintsBuilder::<String>::new(test_field_name)
            .add_constraint(passing_constraint);

        assert_eq!(builder.name, test_field_name);
        assert_eq!(builder.constraints.len(), 1);
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

        assert_eq!(builder.constraints.len(), constraint_count);
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
        assert_eq!(constraints.constraints.len(), 1);
    }

    #[rstest]
    fn constraints_builder_method(test_field_name: &'static str) {
        let constraints =
            Constraints::<String>::builder(test_field_name).build();
        assert_eq!(constraints.name(), test_field_name);
        assert!(constraints.constraints.is_empty());
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
}
