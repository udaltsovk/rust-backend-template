use std::fmt::Display;

use crate::validation::constrains::Constrain;

macro_rules! range_constrain {
    ($name: ident, $func: ident, $msg: literal) => {
        pub struct $name<T>(pub T)
        where
            T: PartialOrd + Display;

        impl<T> Constrain<T> for $name<T>
        where
            T: PartialOrd + Display,
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

range_constrain!(Max, le, "can't be greater");

range_constrain!(LessThan, lt, "must be less");

range_constrain!(Min, ge, "can't be less");

range_constrain!(GreaterThan, gt, "must be greater");
