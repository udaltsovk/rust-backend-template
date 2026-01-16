use std::fmt::Display;

pub use num_traits::Num;

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
    fn max_i32_constraint(
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
    fn max_f64_constraint(
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
    fn less_than_i32_constraint(
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
    fn less_than_f64_constraint(
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
    fn min_i32_constraint(
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
    fn min_f64_constraint(
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
    fn greater_than_i32_constraint(
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
    fn greater_than_f64_constraint(
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
    // i32 tests
    #[case(Max(10i32), 5, true)]
    #[case(Max(10i32), 10, true)]
    #[case(Max(10i32), 15, false)]
    #[case(Max(0i32), 0, true)]
    #[case(Max(-5i32), -10, true)]
    #[case(Max(-5i32), 0, false)]
    #[case(Min(10i32), 5, false)]
    #[case(Min(10i32), 10, true)]
    #[case(Min(10i32), 15, true)]
    #[case(Min(0i32), 0, true)]
    #[case(Min(-5i32), -10, false)]
    #[case(Min(-5i32), 0, true)]
    #[case(LessThan(10i32), 5, true)]
    #[case(LessThan(10i32), 10, false)]
    #[case(LessThan(10i32), 15, false)]
    #[case(LessThan(0i32), 1, false)]
    #[case(LessThan(-5i32), -10, true)]
    #[case(LessThan(-5i32), 0, false)]
    #[case(GreaterThan(10i32), 5, false)]
    #[case(GreaterThan(10i32), 10, false)]
    #[case(GreaterThan(10i32), 15, true)]
    #[case(GreaterThan(0i32), 1, true)]
    #[case(GreaterThan(-5i32), -10, false)]
    #[case(GreaterThan(-5i32), 0, true)]
    // f64 tests
    #[case(Max(10.5f64), 5.2, true)]
    #[case(Max(10.5f64), 10.5, true)]
    #[case(Max(10.5f64), 15.8, false)]
    #[case(Max(0.0f64), 0.0, true)]
    #[case(Max(-5.5f64), -10.2, true)]
    #[case(Max(-5.5f64), 0.1, false)]
    #[case(Min(10.5f64), 5.2, false)]
    #[case(Min(10.5f64), 10.5, true)]
    #[case(Min(10.5f64), 15.8, true)]
    #[case(Min(0.0f64), 0.0, true)]
    #[case(Min(-5.5f64), -10.2, false)]
    #[case(Min(-5.5f64), 0.1, true)]
    #[case(LessThan(10.5f64), 5.2, true)]
    #[case(LessThan(10.5f64), 10.5, false)]
    #[case(LessThan(10.5f64), 15.8, false)]
    #[case(LessThan(0.0f64), -0.1, true)]
    #[case(LessThan(-5.5f64), -10.2, true)]
    #[case(LessThan(-5.5f64), 0.1, false)]
    #[case(GreaterThan(10.5f64), 5.2, false)]
    #[case(GreaterThan(10.5f64), 10.5, false)]
    #[case(GreaterThan(10.5f64), 15.8, true)]
    #[case(GreaterThan(0.0f64), 0.1, true)]
    #[case(GreaterThan(-5.5f64), -10.2, false)]
    #[case(GreaterThan(-5.5f64), 0.1, true)]
    // usize tests
    #[case(Max(100usize), 50usize, true)]
    #[case(Max(100usize), 150usize, false)]
    #[case(Min(10usize), 50usize, true)]
    #[case(Min(10usize), 5usize, false)]
    #[case(LessThan(50usize), 30usize, true)]
    #[case(LessThan(50usize), 60usize, false)]
    #[case(GreaterThan(20usize), 30usize, true)]
    #[case(GreaterThan(20usize), 15usize, false)]
    // u8 tests
    #[case(Max(200u8), 100u8, true)]
    #[case(Max(200u8), 255u8, false)]
    #[case(Min(50u8), 100u8, true)]
    #[case(Min(50u8), 25u8, false)]
    // floating point precision tests
    #[case(Max(1.0f64), 0.999_999_9, true)]
    #[case(Max(1.0f64), 1.0, true)]
    #[case(Max(1.0f64), 1.000_000_1, false)]
    fn range_constraints<T>(
        #[case] constraint: impl Constraint<T>,
        #[case] value: T,
        #[case] expected: bool,
    ) where
        T: std::fmt::Display + num_traits::Num + PartialOrd,
    {
        assert_eq!(constraint.check(&value), expected);
    }

    #[rstest]
    fn error_messages_format() {
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
