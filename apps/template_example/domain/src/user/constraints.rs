use std::sync::LazyLock;

use lib::domain::validation::constraints::{self, ConstraintVec};

pub static NAME_SURNAME_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(
                constraints::length::Min::with_err(|_, limit| {
                    format!("must be at least {limit} characters long")
                })
                .limit(1)
                .build(),
            )
            .add_constraint(
                constraints::length::Max::with_err(|_, limit| {
                    format!("must be at most {limit} characters long")
                })
                .limit(100)
                .build(),
            )
    });
