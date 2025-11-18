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
    use rstest::rstest;

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

    #[rstest]
    fn test_constraints_builder_new() {
        let builder = ConstraintsBuilder::<String>::new("test_field");
        assert_eq!(builder.name, "test_field");
        assert!(builder.constraints.is_empty());
    }

    #[rstest]
    fn test_constraints_builder_add_constraint() {
        let constraint = TestConstraint {
            should_pass: true,
            message: "test message".to_string(),
        };

        let builder = ConstraintsBuilder::<String>::new("test_field")
            .add_constraint(constraint);

        assert_eq!(builder.name, "test_field");
        assert_eq!(builder.constraints.len(), 1);
    }

    #[rstest]
    fn test_constraints_builder_multiple_constraints() {
        let constraint1 = TestConstraint {
            should_pass: true,
            message: "message1".to_string(),
        };
        let constraint2 = TestConstraint {
            should_pass: false,
            message: "message2".to_string(),
        };

        let builder = ConstraintsBuilder::<String>::new("test_field")
            .add_constraint(constraint1)
            .add_constraint(constraint2);

        assert_eq!(builder.constraints.len(), 2);
    }

    #[rstest]
    fn test_constraints_builder_build() {
        let constraint = TestConstraint {
            should_pass: true,
            message: "test message".to_string(),
        };

        let constraints = ConstraintsBuilder::<String>::new("test_field")
            .add_constraint(constraint)
            .build();

        assert_eq!(constraints.name(), "test_field");
        assert_eq!(constraints.constraints.len(), 1);
    }

    #[rstest]
    fn test_constraints_builder_method() {
        let constraints = Constraints::<String>::builder("test_field").build();
        assert_eq!(constraints.name(), "test_field");
        assert!(constraints.constraints.is_empty());
    }

    #[rstest]
    fn test_constraints_name() {
        let constraints = Constraints::<String>::builder("my_field").build();
        assert_eq!(constraints.name(), "my_field");
    }

    #[rstest]
    fn test_constraints_check_all_pass() {
        let constraint1 = TestConstraint {
            should_pass: true,
            message: "error1".to_string(),
        };
        let constraint2 = TestConstraint {
            should_pass: true,
            message: "error2".to_string(),
        };

        let constraints = Constraints::<String>::builder("test_field")
            .add_constraint(constraint1)
            .add_constraint(constraint2)
            .build();

        let errors = constraints.check(&"test_value".to_string());
        assert!(errors.into_inner().is_empty());
    }

    #[rstest]
    fn test_constraints_check_some_fail() {
        let constraint1 = TestConstraint {
            should_pass: true,
            message: "error1".to_string(),
        };
        let constraint2 = TestConstraint {
            should_pass: false,
            message: "error2".to_string(),
        };

        let constraints = Constraints::<String>::builder("test_field")
            .add_constraint(constraint1)
            .add_constraint(constraint2)
            .build();

        let errors = constraints.check(&"test_value".to_string());
        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 1);
        assert_eq!(error_list[0].0, "test_field");
        assert_eq!(error_list[0].1, "error2");
    }

    #[rstest]
    fn test_constraints_check_all_fail() {
        let constraint1 = TestConstraint {
            should_pass: false,
            message: "error1".to_string(),
        };
        let constraint2 = TestConstraint {
            should_pass: false,
            message: "error2".to_string(),
        };

        let constraints = Constraints::<String>::builder("test_field")
            .add_constraint(constraint1)
            .add_constraint(constraint2)
            .build();

        let errors = constraints.check(&"test_value".to_string());
        let error_list = errors.into_inner();
        assert_eq!(error_list.len(), 2);
        assert_eq!(error_list[0].0, "test_field");
        assert_eq!(error_list[0].1, "error1");
        assert_eq!(error_list[1].0, "test_field");
        assert_eq!(error_list[1].1, "error2");
    }

    #[rstest]
    fn test_constraints_check_empty() {
        let constraints = Constraints::<String>::builder("test_field").build();

        let errors = constraints.check(&"test_value".to_string());
        assert!(errors.into_inner().is_empty());
    }
}
