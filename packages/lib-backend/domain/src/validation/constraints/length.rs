use bon::Builder;

use crate::validation::constraints::Constraint;

macro_rules! length_constraint {
    ($name: ident, $func: ident $(,)?) => {
        #[derive(Builder)]
        #[builder(derive(Clone), start_fn = with_err)]
        pub struct $name<T> {
            #[builder(start_fn)]
            err_fn: fn(&T, usize) -> String,
            limit: usize,
        }

        impl Constraint<String> for $name<String> {
            fn check(&self, value: &String) -> bool {
                value.chars().count().$func(&self.limit)
            }

            fn error_msg(&self, rejected_value: &String) -> String {
                (self.err_fn)(rejected_value, self.limit)
            }
        }

        impl<T> Constraint<Vec<T>> for $name<Vec<T>> {
            fn check(&self, value: &Vec<T>) -> bool {
                value.len().$func(&self.limit)
            }

            fn error_msg(&self, rejected_value: &Vec<T>) -> String {
                (self.err_fn)(rejected_value, self.limit)
            }
        }
    };
}

length_constraint!(Max, le);

length_constraint!(LessThan, lt);

length_constraint!(Min, ge);

length_constraint!(GreaterThan, gt);

#[cfg(test)]
mod tests {
    use pastey::paste;
    use rstest::{fixture, rstest};

    use super::{GreaterThan, LessThan, Max, Min};
    use crate::validation::constraints::Constraint as _;

    macro_rules! fixtures {
        (
            constraint = $name: ident,
            default_limit = $limit: expr,
            message = $msg: literal $(,)?
        ) => {
            paste! {
                impl $name<String> {
                    fn err(_: &String, limit: usize) -> String {
                        format!("must be {} {} characters long", $msg, limit)
                    }
                }

                #[fixture]
                fn [<$name:snake:lower _string>](#[default($limit)] limit: usize) -> $name<String> {
                    $name::with_err($name::<String>::err)
                        .limit(limit)
                        .build()
                }

                impl<T> $name<Vec<T>> {
                    fn err(_: &Vec<T>, limit: usize) -> String {
                        format!("must be {} {} items long", $msg, limit)
                    }
                }

                #[fixture]
                fn [<$name:snake:lower _vec>]<T>(#[default($limit)] limit: usize) -> $name<Vec<T>> {
                    $name::with_err($name::<Vec<_>>::err)
                        .limit(limit)
                        .build()
                }

            }
        };
    }

    fixtures!(
        constraint = Max,
        default_limit = usize::MIN,
        message = "at most"
    );
    fixtures!(
        constraint = LessThan,
        default_limit = usize::MAX,
        message = "less"
    );
    fixtures!(
        constraint = Min,
        default_limit = usize::MAX,
        message = "at least"
    );
    fixtures!(
        constraint = GreaterThan,
        default_limit = usize::MIN,
        message = "greater"
    );

    #[rstest]
    #[case(5, "hello", true)]
    #[case(3, "hello", false)]
    #[case(5, "world", true)]
    #[case(4, "hello", false)]
    fn max_string_constraint(
        #[case] limit: usize,
        #[from(max_string)]
        #[with(limit)]
        constraint: Max<String>,
        #[case] input: String,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Max::<String>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(3, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(2, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(5, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(3, Vec::<i32>::new(), true)]
    fn max_vec_constraint(
        #[case] limit: usize,
        #[from(max_vec)]
        #[with(limit)]
        constraint: Max<Vec<i32>>,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Max::<Vec<_>>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(5, "hello", false)]
    #[case(4, "hello", false)]
    #[case(6, "hello", true)]
    #[case(3, "hello", false)]
    fn less_than_string_constraint(
        #[case] limit: usize,
        #[from(less_than_string)]
        #[with(limit)]
        constraint: LessThan<String>,
        #[case] input: String,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            LessThan::<String>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(4, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(3, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(2, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(1, Vec::<i32>::new(), true)]
    fn less_than_vec_constraint(
        #[case] limit: usize,
        #[from(less_than_vec)]
        #[with(limit)]
        constraint: LessThan<Vec<i32>>,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            LessThan::<Vec<_>>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(5, "hello", true)]
    #[case(6, "hello", false)]
    #[case(3, "hello", true)]
    #[case(5, "", false)]
    fn min_string_constraint(
        #[case] limit: usize,
        #[from(min_string)]
        #[with(limit)]
        constraint: Min<String>,
        #[case] input: String,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Min::<String>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(3, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(4, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(2, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(0, Vec::<i32>::new(), true)]
    fn min_vec_constraint(
        #[case] limit: usize,
        #[from(min_vec)]
        #[with(limit)]
        constraint: Min<Vec<i32>>,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            Min::<Vec<_>>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(4, "hello", true)]
    #[case(5, "hello", false)]
    #[case(6, "hello", false)]
    #[case(3, "hello", true)]
    fn greater_than_string_constraint(
        #[case] limit: usize,
        #[from(greater_than_string)]
        #[with(limit)]
        constraint: GreaterThan<String>,
        #[case] input: String,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            GreaterThan::<String>::err(&input, limit)
        );
    }

    #[rstest]
    #[case(2, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(3, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(4, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(0, vec![1_i32], true)]
    fn greater_than_vec_constraint(
        #[case] limit: usize,
        #[from(greater_than_vec)]
        #[with(limit)]
        constraint: GreaterThan<Vec<i32>>,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            constraint.error_msg(&input),
            GreaterThan::<Vec<_>>::err(&input, limit)
        );
    }
}
