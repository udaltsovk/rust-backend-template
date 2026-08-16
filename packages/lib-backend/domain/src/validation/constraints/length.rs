use macros::{constraint, constraint_check};

use super::{Constraint, Validation};

macro_rules! length_constraint {
    ($name:ident, $func:ident $(,)?) => {
        #[constraint]
        pub struct $name<T> {
            err_fn: fn(&T, usize) -> String,
            limit: usize,
        }

        #[constraint_check(String)]
        impl $name<String> {
            fn is_valid(&self, value: &str) -> bool {
                value.chars().count().$func(&self.limit)
            }
        }

        #[constraint_check(Vec<T>)]
        impl<T> $name<Vec<T>> {
            fn is_valid(&self, value: &[T]) -> bool {
                value.len().$func(&self.limit)
            }
        }
    };
}

length_constraint!(Max, le);

length_constraint!(LessThan, lt);

length_constraint!(Min, ge);

length_constraint!(GreaterThan, gt);
