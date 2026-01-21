use std::{
    fmt::{Debug, Display},
    sync::LazyLock,
};

use lib::{
    DomainType,
    domain::{
        DomainType as _, try_from_option,
        validation::{
            Constraints,
            constraints::{self, range::Num},
            error::ValidationErrors,
        },
    },
    paste,
};
use serde::Serialize;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserTargetSettingsAge(u8);

impl UserTargetSettingsAge {
    fn constraints<T>() -> Constraints<T>
    where
        T: Num
            + Serialize
            + Clone
            + Debug
            + PartialOrd
            + Display
            + Send
            + Sync
            + 'static,
        T::FromStrRadixErr: Debug,
    {
        Constraints::builder("age")
            .add_constraint(constraints::range::Min(T::zero()))
            .add_constraint(constraints::range::Max(
                T::from_str_radix("100", 10).expect("a valid number"),
            ))
            .build()
    }
}

macro_rules! numeric_constraints {
    ($type: ty) => {
        paste! {
            static [<CONSTRAINTS_ $type:upper>]: LazyLock<Constraints<$type>> =
                LazyLock::new(UserTargetSettingsAge::constraints);

            impl TryFrom<$type> for UserTargetSettingsAge {
                type Error = ValidationErrors;

                fn try_from(value: $type) -> Result<Self, ValidationErrors> {
                    [<CONSTRAINTS_ $type:upper>].check(&value).into_result(|_| {
                        Self(value.try_into().unwrap_or_else(
                            Self::it_should_be_safe_to_unwrap([<CONSTRAINTS_ $type:upper>].name()),
                        ))
                    })
                }
            }
        }

        impl From<UserTargetSettingsAge> for $type {
            fn from(age: UserTargetSettingsAge) -> Self {
                age.0.into()
            }
        }
    };
}

numeric_constraints!(i16);
numeric_constraints!(i64);

try_from_option!(
    domain_type = UserTargetSettingsAge,
    from_ty = i64,
    constraints = CONSTRAINTS_I64
);
