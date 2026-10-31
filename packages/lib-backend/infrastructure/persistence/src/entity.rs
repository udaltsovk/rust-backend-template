use std::{
    any::type_name,
    num::{NonZero, ZeroablePrimitive},
};

use domain::{DomainType, Id, IdRepr, validation::error::ValidationErrors};

pub trait IdFromDb<T, N>
where
    Self: Sized,
    N: ZeroablePrimitive,
    NonZero<N>: IdRepr,
{
    fn into_id(self) -> Id<T, NonZero<N>>;
}

impl<T, N> IdFromDb<T, N> for N
where
    N: ZeroablePrimitive,
    NonZero<N>: IdRepr,
{
    fn into_id(self) -> Id<T, NonZero<N>> {
        Id::new(NonZero::new(self).unwrap_or_else(|| {
            panic!("DB returned `id` = 0 for an `Id` column; expected non-zero")
        }))
    }
}

pub trait DomainTypeFromDb<T, I>
where
    Self: Sized,
    T: DomainType<I> + TryFrom<Self, Error = ValidationErrors>,
    I: From<T> + Clone,
{
    fn into_domain(self) -> T;
}

impl<F, T, I> DomainTypeFromDb<T, I> for F
where
    T: DomainType<I> + TryFrom<F, Error = ValidationErrors>,
    I: From<T> + Clone,
{
    fn into_domain(self) -> T {
        self.try_into().unwrap_or_else(|_| {
            panic!(
                "Expected `{}` from the db should be valid",
                type_name::<T>()
            )
        })
    }
}
