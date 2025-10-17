use crate::validation::constraints::Constraint;

macro_rules! length_constraint {
    ($name: ident, $func: ident, $msg: literal) => {
        pub struct $name(pub usize);

        impl Constraint<String> for $name {
            fn check(&self, value: &String) -> bool {
                value.chars().count().$func(&self.0)
            }

            fn error_msg(&self) -> String {
                format!("must be {} {} characters long", $msg, self.0)
            }
        }

        impl<T> Constraint<Vec<T>> for $name {
            fn check(&self, value: &Vec<T>) -> bool {
                value.len().$func(&self.0)
            }

            fn error_msg(&self) -> String {
                format!("must be {} {} items long", $msg, self.0)
            }
        }
    };
}

length_constraint!(Max, le, "at most");

length_constraint!(LessThan, lt, "less");

length_constraint!(Min, ge, "at least");

length_constraint!(GreaterThan, gt, "greater");
