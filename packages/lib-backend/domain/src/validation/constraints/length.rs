use bon::Builder;

use super::Constraint;

macro_rules! length_constraint {
    ($name:ident, $func:ident $(,)?) => {
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

            fn error_msg(
                &self,
                rejected_value: &String,
            ) -> String {
                (self.err_fn)(rejected_value, self.limit)
            }
        }

        impl<T> Constraint<Vec<T>> for $name<Vec<T>> {
            fn check(&self, value: &Vec<T>) -> bool {
                value.len().$func(&self.limit)
            }

            fn error_msg(
                &self,
                rejected_value: &Vec<T>,
            ) -> String {
                (self.err_fn)(rejected_value, self.limit)
            }
        }
    };
}

length_constraint!(Max, le);

length_constraint!(LessThan, lt);

length_constraint!(Min, ge);

length_constraint!(GreaterThan, gt);
