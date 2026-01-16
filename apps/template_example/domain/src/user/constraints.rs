use std::sync::LazyLock;

use lib::domain::validation::constraints::{self, ConstraintVec};

pub static NAME_SURNAME_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(constraints::length::Min(1))
            .add_constraint(constraints::length::Max(100))
    });
