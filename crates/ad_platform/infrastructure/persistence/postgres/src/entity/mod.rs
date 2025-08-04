use std::any::type_name;

use kernel::domain::{DomainType, error::ValidationErrors};

pub(crate) mod client;

trait DomainTypeFromDb<T, I>
where
    Self: Sized,
    T: DomainType<I> + TryFrom<Self, Error = ValidationErrors>,
    I: Clone,
{
    fn into_domain(self) -> T;
}

impl<F, T, I> DomainTypeFromDb<T, I> for F
where
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
    I: Clone,
{
    fn into_domain(self) -> T {
        self.try_into().unwrap_or_else(|_| {
            panic!("Expected `{}` from the db to be valid", type_name::<T>())
        })
    }
}
