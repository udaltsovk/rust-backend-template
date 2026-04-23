use std::fmt::Display;

use bon::Builder;
pub use num_traits::Num;

use super::Constraint;

macro_rules! range_constraint {
    ($name:ident, $func:ident $(,)?) => {
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

            fn error_msg(
                &self,
                rejected_value: &T,
            ) -> String {
                (self.err_fn)(rejected_value, &self.limit)
            }
        }
    };
}

range_constraint!(Max, le);

range_constraint!(LessThan, lt);

range_constraint!(Min, ge);

range_constraint!(GreaterThan, gt);
