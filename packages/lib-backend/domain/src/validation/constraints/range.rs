use std::fmt::Display;

use bon::Builder;
pub use num_traits::Num;

use crate::validation::constraints::Constraint;

macro_rules! range_constraint {
    ($name: ident, $func: ident $(,)?) => {
        #[derive(Builder)]
        #[builder(derive(Clone), start_fn = with_err)]
        pub struct $name<T>
        where
            T: Num + PartialOrd + Display,
        {
            #[builder(start_fn)]
            err_fn: fn(&T, &T) -> String,
            limit: T,
        }

        impl<T> Constraint<T> for $name<T>
        where
            T: Num + PartialOrd + Display,
        {
            fn check(&self, value: &T) -> bool {
                value.$func(&self.limit)
            }

            fn error_msg(&self, rejected_value: &T) -> String {
                (self.err_fn)(rejected_value, &self.limit)
            }
        }
    };
}

range_constraint!(Max, le);

range_constraint!(LessThan, lt);

range_constraint!(Min, ge);

range_constraint!(GreaterThan, gt);

#[cfg(test)]
mod tests {
    use pastey::paste;
    use rstest::{fixture, rstest};

    use super::{GreaterThan, LessThan, Max, Min};
    use crate::validation::constraints::Constraint as _;

    macro_rules! fixtures {
        (
            constraint = $name: ident,
            types = [$($type: ty),* $(,)?],
            default_limit = $limit: expr,
            message = $msg: literal $(,)?
        ) => {
            paste! {
                $(
                    impl $name<$type>
                    {
                        fn err(_: &$type, limit: &$type) -> String {
                            format!("{} than {}", $msg, limit)
                        }
                    }

                    #[fixture]
                    fn [<$name:snake:lower _ $type>](#[default($type::$limit)] limit: $type) -> $name<$type>
                    {
                        $name::with_err($name::<$type>::err)
                            .limit(limit)
                            .build()
                    }
                )*
            }
        };
    }

    fixtures!(
        constraint = Max,
        types = [i32, f64],
        default_limit = MIN,
        message = "can't be greater"
    );
    fixtures!(
        constraint = LessThan,
        types = [i32, f64],
        default_limit = MAX,
        message = "must be less"
    );
    fixtures!(
        constraint = Min,
        types = [i32, f64],
        default_limit = MAX,
        message = "can't be less"
    );
    fixtures!(
        constraint = GreaterThan,
        types = [i32, f64],
        default_limit = MIN,
        message = "must be greater"
    );

    #[rstest]
    #[case(10_i32, 5_i32, true)]
    #[case(10_i32, 10_i32, true)]
    #[case(10_i32, 15_i32, false)]
    #[case(0_i32, 0_i32, true)]
    #[case(-5_i32, -10_i32, true)]
    #[case(-5_i32, 0_i32, false)]
    fn max_i32_constraint(
        #[case] limit: i32,
        #[from(max_i32)]
        #[with(limit)]
        constraint: Max<i32>,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Max::<i32>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10.5_f64, 5.2_f64, true)]
    #[case(10.5_f64, 10.5_f64, true)]
    #[case(10.5_f64, 15.8_f64, false)]
    #[case(0.0_f64, 0.0_f64, true)]
    #[case(-5.5_f64, -10.2_f64, true)]
    #[case(-5.5_f64, 0.1_f64, false)]
    fn max_f64_constraint(
        #[case] limit: f64,
        #[from(max_f64)]
        #[with(limit)]
        constraint: Max<f64>,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Max::<f64>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10_i32, 5_i32, true)]
    #[case(10_i32, 10_i32, false)]
    #[case(10_i32, 15_i32, false)]
    #[case(0_i32, 1_i32, false)]
    #[case(-5_i32, -10_i32, true)]
    #[case(-5_i32, 0_i32, false)]
    fn less_than_i32_constraint(
        #[case] limit: i32,
        #[from(less_than_i32)]
        #[with(limit)]
        constraint: LessThan<i32>,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            LessThan::<i32>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10.5_f64, 5.2_f64, true)]
    #[case(10.5_f64, 10.5_f64, false)]
    #[case(10.5_f64, 15.8_f64, false)]
    #[case(0.0_f64, -0.1_f64, true)]
    #[case(-5.5_f64, -10.2_f64, true)]
    #[case(-5.5_f64, 0.1_f64, false)]
    fn less_than_f64_constraint(
        #[case] limit: f64,
        #[from(less_than_f64)]
        #[with(limit)]
        constraint: LessThan<f64>,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            LessThan::<f64>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10_i32, 5_i32, false)]
    #[case(10_i32, 10_i32, true)]
    #[case(10_i32, 15_i32, true)]
    #[case(0_i32, 0_i32, true)]
    #[case(-5_i32, -10_i32, false)]
    #[case(-5_i32, 0_i32, true)]
    fn min_i32_constraint(
        #[case] limit: i32,
        #[from(min_i32)]
        #[with(limit)]
        constraint: Min<i32>,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Min::<i32>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10.5_f64, 5.2_f64, false)]
    #[case(10.5_f64, 10.5_f64, true)]
    #[case(10.5_f64, 15.8_f64, true)]
    #[case(0.0_f64, 0.0_f64, true)]
    #[case(-5.5_f64, -10.2_f64, false)]
    #[case(-5.5_f64, 0.1_f64, true)]
    fn min_f64_constraint(
        #[case] limit: f64,
        #[from(min_f64)]
        #[with(limit)]
        constraint: Min<f64>,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Min::<f64>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10_i32, 5_i32, false)]
    #[case(10_i32, 10_i32, false)]
    #[case(10_i32, 15_i32, true)]
    #[case(0_i32, 1_i32, true)]
    #[case(-5_i32, -10_i32, false)]
    #[case(-5_i32, 0_i32, true)]
    fn greater_than_i32_constraint(
        #[case] limit: i32,
        #[from(greater_than_i32)]
        #[with(limit)]
        constraint: GreaterThan<i32>,
        #[case] input: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            GreaterThan::<i32>::err(&input, &limit)
        );
    }

    #[rstest]
    #[case(10.5_f64, 5.2_f64, false)]
    #[case(10.5_f64, 10.5_f64, false)]
    #[case(10.5_f64, 15.8_f64, true)]
    #[case(0.0_f64, 0.1_f64, true)]
    #[case(-5.5_f64, -10.2_f64, false)]
    #[case(-5.5_f64, 0.1_f64, true)]
    fn greater_than_f64_constraint(
        #[case] limit: f64,
        #[from(greater_than_f64)]
        #[with(limit)]
        constraint: GreaterThan<f64>,
        #[case] input: f64,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            GreaterThan::<f64>::err(&input, &limit)
        );
    }
}
