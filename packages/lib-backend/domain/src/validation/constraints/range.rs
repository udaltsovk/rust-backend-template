use std::fmt::Display;

use num_traits::Num;

use crate::validation::constraints::Constraint;

macro_rules! range_constraint {
    ($name: ident, $func: ident, $msg: literal) => {
        pub struct $name<T>(pub T)
        where
            T: Num + PartialOrd + Display;

        impl<T> Constraint<T> for $name<T>
        where
            T: Num + PartialOrd + Display,
        {
            fn check(&self, value: &T) -> bool {
                value.$func(&self.0)
            }

            fn error_msg(&self) -> String {
                format!("{} than {}", $msg, self.0)
            }
        }
    };
}

range_constraint!(Max, le, "can't be greater");

range_constraint!(LessThan, lt, "must be less");

range_constraint!(Min, ge, "can't be less");

range_constraint!(GreaterThan, gt, "must be greater");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{GreaterThan, LessThan, Max, Min};
    use crate::validation::constraints::Constraint;

    #[rstest]
    #[case(10, 5, true)]
    #[case(10, 10, true)]
    #[case(10, 15, false)]
    #[case(0, 0, true)]
    #[case(-5, -10, true)]
    #[case(-5, 0, false)]
    fn test_max_i32_constraint(
        #[case] max_val: i32,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        let constraint = Max(max_val);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("can't be greater than {max_val}")
        );
    }

    #[rstest]
    #[case(10.5, 5.2, true)]
    #[case(10.5, 10.5, true)]
    #[case(10.5, 15.8, false)]
    #[case(0.0, 0.0, true)]
    #[case(-5.5, -10.2, true)]
    #[case(-5.5, 0.1, false)]
    fn test_max_f64_constraint(
        #[case] max_val: f64,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        let constraint = Max(max_val);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("can't be greater than {max_val}")
        );
    }

    #[rstest]
    #[case(10, 5, true)]
    #[case(10, 10, false)]
    #[case(10, 15, false)]
    #[case(0, 1, false)]
    #[case(-5, -10, true)]
    #[case(-5, 0, false)]
    fn test_less_than_i32_constraint(
        #[case] limit: i32,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        let constraint = LessThan(limit);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("must be less than {limit}")
        );
    }

    #[rstest]
    #[case(10.5, 5.2, true)]
    #[case(10.5, 10.5, false)]
    #[case(10.5, 15.8, false)]
    #[case(0.0, -0.1, true)]
    #[case(-5.5, -10.2, true)]
    #[case(-5.5, 0.1, false)]
    fn test_less_than_f64_constraint(
        #[case] limit: f64,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        let constraint = LessThan(limit);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("must be less than {limit}")
        );
    }

    #[rstest]
    #[case(10, 5, false)]
    #[case(10, 10, true)]
    #[case(10, 15, true)]
    #[case(0, 0, true)]
    #[case(-5, -10, false)]
    #[case(-5, 0, true)]
    fn test_min_i32_constraint(
        #[case] min_val: i32,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        let constraint = Min(min_val);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("can't be less than {min_val}")
        );
    }

    #[rstest]
    #[case(10.5, 5.2, false)]
    #[case(10.5, 10.5, true)]
    #[case(10.5, 15.8, true)]
    #[case(0.0, 0.0, true)]
    #[case(-5.5, -10.2, false)]
    #[case(-5.5, 0.1, true)]
    fn test_min_f64_constraint(
        #[case] min_val: f64,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        let constraint = Min(min_val);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("can't be less than {min_val}")
        );
    }

    #[rstest]
    #[case(10, 5, false)]
    #[case(10, 10, false)]
    #[case(10, 15, true)]
    #[case(0, 1, true)]
    #[case(-5, -10, false)]
    #[case(-5, 0, true)]
    fn test_greater_than_i32_constraint(
        #[case] limit: i32,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        let constraint = GreaterThan(limit);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("must be greater than {limit}")
        );
    }

    #[rstest]
    #[case(10.5, 5.2, false)]
    #[case(10.5, 10.5, false)]
    #[case(10.5, 15.8, true)]
    #[case(0.0, 0.1, true)]
    #[case(-5.5, -10.2, false)]
    #[case(-5.5, 0.1, true)]
    fn test_greater_than_f64_constraint(
        #[case] limit: f64,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        let constraint = GreaterThan(limit);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(),
            format!("must be greater than {limit}")
        );
    }

    #[rstest]
    fn test_range_constraints_with_usize() {
        let max_constraint = Max(100usize);
        let min_constraint = Min(10usize);
        let less_than_constraint = LessThan(50usize);
        let greater_than_constraint = GreaterThan(20usize);

        assert!(max_constraint.check(&50usize));
        assert!(!max_constraint.check(&150usize));

        assert!(min_constraint.check(&50usize));
        assert!(!min_constraint.check(&5usize));

        assert!(less_than_constraint.check(&30usize));
        assert!(!less_than_constraint.check(&60usize));

        assert!(greater_than_constraint.check(&30usize));
        assert!(!greater_than_constraint.check(&15usize));
    }

    #[rstest]
    fn test_range_constraints_with_u8() {
        let max_constraint = Max(200u8);
        let min_constraint = Min(50u8);

        assert!(max_constraint.check(&100u8));
        assert!(!max_constraint.check(&255u8));

        assert!(min_constraint.check(&100u8));
        assert!(!min_constraint.check(&25u8));
    }

    #[rstest]
    fn test_boundary_values() {
        // Test exact boundary conditions
        let max_10 = Max(10i32);
        assert!(max_10.check(&10)); // Should be true (<=)
        assert!(!max_10.check(&11)); // Should be false

        let min_10 = Min(10i32);
        assert!(min_10.check(&10)); // Should be true (>=)
        assert!(!min_10.check(&9)); // Should be false

        let less_than_10 = LessThan(10i32);
        assert!(!less_than_10.check(&10)); // Should be false (<)
        assert!(less_than_10.check(&9)); // Should be true

        let greater_than_10 = GreaterThan(10i32);
        assert!(!greater_than_10.check(&10)); // Should be false (>)
        assert!(greater_than_10.check(&11)); // Should be true
    }

    #[rstest]
    fn test_negative_numbers() {
        let max_constraint = Max(-5i32);
        assert!(max_constraint.check(&-10));
        assert!(max_constraint.check(&-5));
        assert!(!max_constraint.check(&0));

        let min_constraint = Min(-5i32);
        assert!(!min_constraint.check(&-10));
        assert!(min_constraint.check(&-5));
        assert!(min_constraint.check(&0));
    }

    #[rstest]
    fn test_floating_point_precision() {
        let constraint = Max(1.0f64);

        assert!(constraint.check(&0.999_999_9));
        assert!(constraint.check(&1.0));
        assert!(!constraint.check(&1.000_000_1));
    }

    #[rstest]
    fn test_error_messages_format() {
        let max_constraint = Max(42i32);
        let min_constraint = Min(10i32);
        let less_than_constraint = LessThan(100i32);
        let greater_than_constraint = GreaterThan(5i32);

        assert_eq!(max_constraint.error_msg(), "can't be greater than 42");
        assert_eq!(min_constraint.error_msg(), "can't be less than 10");
        assert_eq!(less_than_constraint.error_msg(), "must be less than 100");
        assert_eq!(
            greater_than_constraint.error_msg(),
            "must be greater than 5"
        );
    }
}
