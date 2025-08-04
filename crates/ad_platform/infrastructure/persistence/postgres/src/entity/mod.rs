use std::any::type_name;

use kernel::domain::{DomainType, error::ValidationErrors};

pub(crate) mod client;

trait DomainTypeFromDb<F, T, I>
where
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
    I: Clone,
{
    fn safe_parse(value: F) -> T;
}

impl<F, T, I> DomainTypeFromDb<F, T, I> for T
where
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
    I: Clone,
{
    fn safe_parse(value: F) -> T {
        value.try_into().unwrap_or_else(|_| {
            panic!("Expected `{}` from the db to be valid", type_name::<T>())
        })
    }
}
