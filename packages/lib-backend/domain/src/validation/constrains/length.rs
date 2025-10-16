use crate::validation::constrains::Constrain;

macro_rules! length_constrain {
    ($name: ident, $func: ident, $msg: literal) => {
        pub struct $name(pub usize);

        impl Constrain<String> for $name {
            fn check(&self, value: &String) -> bool {
                value.chars().count().$func(&self.0)
            }

            fn error_msg(&self) -> String {
                format!("must be {} {} characters long", $msg, self.0)
            }
        }

        impl<T> Constrain<Vec<T>> for $name {
            fn check(&self, value: &Vec<T>) -> bool {
                value.len().$func(&self.0)
            }

            fn error_msg(&self) -> String {
                format!("must be {} {} items long", $msg, self.0)
            }
        }
    };
}

length_constrain!(Max, le, "at most");

length_constrain!(LessThan, lt, "less");

length_constrain!(Min, ge, "at least");

length_constrain!(GreaterThan, gt, "greater");
