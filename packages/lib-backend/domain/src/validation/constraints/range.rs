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
