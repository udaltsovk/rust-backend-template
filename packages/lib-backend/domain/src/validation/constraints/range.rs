use std::fmt::Display;

use macros::{constraint, constraint_check};
pub use num_traits::Num;

use super::{Constraint, Validation};

macro_rules! range_constraint {
    ($name:ident, $func:ident $(,)?) => {
        #[constraint]
        pub struct $name<T>
        where
            T: Num + PartialOrd + Display,
        {
            err_fn: fn(&T, &T) -> String,
            limit: T,
        }

        #[constraint_check(T)]
        impl<T> $name<T>
        where
            T: Num + PartialOrd + Display,
        {
            fn is_valid(&self, value: &T) -> bool {
                value.$func(&self.limit)
            }
        }
    };
}

range_constraint!(Max, le);

range_constraint!(LessThan, lt);

range_constraint!(Min, ge);

range_constraint!(GreaterThan, gt);
