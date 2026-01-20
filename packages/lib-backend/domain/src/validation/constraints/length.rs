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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::{GreaterThan, LessThan, Max, Min};
    use crate::validation::constraints::Constraint;

    #[fixture]
    fn max_5() -> Max {
        Max(5)
    }

    #[fixture]
    fn min_3() -> Min {
        Min(3)
    }

    #[fixture]
    fn less_than_6() -> LessThan {
        LessThan(6)
    }

    #[fixture]
    fn greater_than_4() -> GreaterThan {
        GreaterThan(4)
    }

    #[rstest]
    #[case(5, "hello", true)]
    #[case(3, "hello", false)]
    #[case(5, "world", true)]
    #[case(4, "hello", false)]
    fn max_string_constraint(
        #[case] max_len: usize,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let constraint = Max(max_len);
        assert_eq!(constraint.check(&input.to_string()), expected);
        assert_eq!(
            <Max as Constraint<String>>::error_msg(&constraint),
            format!("must be at most {max_len} characters long")
        );
    }

    #[rstest]
    #[case(3, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(2, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(5, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(3, Vec::<i32>::new(), true)]
    fn max_vec_constraint(
        #[case] max_len: usize,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        let constraint = Max(max_len);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            <Max as Constraint<Vec<i32>>>::error_msg(&constraint),
            format!("must be at most {max_len} items long")
        );
    }

    #[rstest]
    #[case(5, "hello", false)]
    #[case(4, "hello", false)]
    #[case(6, "hello", true)]
    #[case(3, "hello", false)]
    fn less_than_string_constraint(
        #[case] limit: usize,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let constraint = LessThan(limit);
        assert_eq!(constraint.check(&input.to_string()), expected);
        assert_eq!(
            <LessThan as Constraint<String>>::error_msg(&constraint),
            format!("must be less {limit} characters long")
        );
    }

    #[rstest]
    #[case(4, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(3, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(2, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(1, Vec::<i32>::new(), true)]
    fn less_than_vec_constraint(
        #[case] limit: usize,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        let constraint = LessThan(limit);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            <LessThan as Constraint<Vec<i32>>>::error_msg(&constraint),
            format!("must be less {limit} items long")
        );
    }

    #[rstest]
    #[case(5, "hello", true)]
    #[case(6, "hello", false)]
    #[case(3, "hello", true)]
    #[case(5, "", false)]
    fn min_string_constraint(
        #[case] min_len: usize,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let constraint = Min(min_len);
        assert_eq!(constraint.check(&input.to_string()), expected);
        assert_eq!(
            <Min as Constraint<String>>::error_msg(&constraint),
            format!("must be at least {min_len} characters long")
        );
    }

    #[rstest]
    #[case(3, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(4, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(2, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(0, Vec::<i32>::new(), true)]
    fn min_vec_constraint(
        #[case] min_len: usize,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        let constraint = Min(min_len);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            <Min as Constraint<Vec<i32>>>::error_msg(&constraint),
            format!("must be at least {min_len} items long")
        );
    }

    #[rstest]
    #[case(4, "hello", true)]
    #[case(5, "hello", false)]
    #[case(6, "hello", false)]
    #[case(3, "hello", true)]
    fn greater_than_string_constraint(
        #[case] limit: usize,
        #[case] input: &str,
        #[case] expected: bool,
    ) {
        let constraint = GreaterThan(limit);
        assert_eq!(constraint.check(&input.to_string()), expected);
        assert_eq!(
            <GreaterThan as Constraint<String>>::error_msg(&constraint),
            format!("must be greater {limit} characters long")
        );
    }

    #[rstest]
    #[case(2, vec![1_i32, 2_i32, 3_i32], true)]
    #[case(3, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(4, vec![1_i32, 2_i32, 3_i32], false)]
    #[case(0, vec![1_i32], true)]
    fn greater_than_vec_constraint(
        #[case] limit: usize,
        #[case] input: Vec<i32>,
        #[case] expected: bool,
    ) {
        let constraint = GreaterThan(limit);
        assert_eq!(constraint.check(&input), expected);
        assert_eq!(
            <GreaterThan as Constraint<Vec<i32>>>::error_msg(&constraint),
            format!("must be greater {limit} items long")
        );
    }

    #[rstest]
    fn unicode_string_length() {
        // Test that we count characters, not bytes
        {
            let constraint = Max(3);
            assert!(!constraint.check(&"café".to_string())); // 4 bytes, 4 chars - should fail for Max(3)
        }

        {
            let constraint = Max(4);
            assert!(constraint.check(&"café".to_string())); // Should pass for Max(4)
        }

        {
            let constraint = Min(4);
            assert!(constraint.check(&"café".to_string())); // Should pass for Min(4)
        }

        // Test with emoji
        {
            let constraint = Max(2);
            assert!(constraint.check(&"🚀🎉".to_string())); // 2 emoji characters
        }

        {
            let constraint = Max(1);
            assert!(!constraint.check(&"🚀🎉".to_string())); // Should fail for Max(1)
        }
    }

    #[rstest]
    fn empty_collections() {
        let max_constraint = Max(0);
        let min_constraint = Min(0);
        let less_than_constraint = LessThan(1);
        let greater_than_constraint = GreaterThan(0);

        // Empty string
        let empty_string = String::new();
        assert!(max_constraint.check(&empty_string));
        assert!(min_constraint.check(&empty_string));
        assert!(less_than_constraint.check(&empty_string));
        assert!(!greater_than_constraint.check(&empty_string));

        // Empty vector
        let empty_vec: Vec<i32> = vec![];
        assert!(max_constraint.check(&empty_vec));
        assert!(min_constraint.check(&empty_vec));
        assert!(less_than_constraint.check(&empty_vec));
        assert!(!greater_than_constraint.check(&empty_vec));
    }
}
