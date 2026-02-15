use std::{
    fmt::{Debug, Display},
    sync::LazyLock,
};

use lib::{
    DomainType,
    domain::{
        DomainType as _, impl_try_from_external_input,
        pastey::paste,
        validation::{
            Constraints,
            constraints::{self, range::Num},
            error::{ValidationErrors, ValidationResult},
        },
    },
};
use serde::Serialize;

#[derive(DomainType, Debug)]
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
        Constraints::builder()
            .add_constraint(
                constraints::range::Min::with_err(|_, limit| {
                    format!("can't be less than {limit}")
                })
                .limit(T::zero())
                .build(),
            )
            .add_constraint(
                constraints::range::Max::with_err(|_, limit| {
                    format!("can't be greater than {limit}")
                })
                .limit(T::from_str_radix("100", 10).expect("a valid number"))
                .build(),
            )
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

                fn try_from(value: $type) -> ValidationResult<Self> {
                    [<CONSTRAINTS_ $type:upper>].check(&value).into_result(|_| {
                        Self(value.try_into().unwrap_or_else(
                            Self::it_should_be_safe_to_unwrap(),
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

impl_try_from_external_input!(
    domain_type = UserTargetSettingsAge,
    input_type = i64,
);
