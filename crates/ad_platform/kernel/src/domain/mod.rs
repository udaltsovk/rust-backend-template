use std::{any::type_name, marker::PhantomData};

use derive_where::derive_where;
use uuid::Uuid;

use crate::domain::error::ValidationErrors;

pub mod client;
pub mod error;
pub mod session;

#[derive_where(Clone, Debug)]
pub struct Id<T> {
    pub value: Uuid,
    _entity: PhantomData<T>,
}
impl<T> Id<T> {
    pub fn new(value: Uuid) -> Self {
        Self {
            value,
            _entity: PhantomData,
        }
    }

    pub fn generate() -> Self {
        Self::new(Uuid::now_v7())
    }
}
impl<T> From<Uuid> for Id<T> {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

pub trait DomainType<T>
where
    Self: Sized,
    T: Clone,
{
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
    fn into_inner(self) -> T;

    fn cloned_inner(&self) -> T {
        self.value().clone()
    }

    fn parse<F>(
        value: F,
        mut errors: Vec<ValidationErrors>,
    ) -> (Vec<ValidationErrors>, impl FnOnce() -> Self)
    where
        Self: TryFrom<F, Error = ValidationErrors>,
    {
        let res = Self::try_from(value)
            .inspect_err(|err: &ValidationErrors| errors.push(err.clone()));
        (errors, || {
            res.unwrap_or_else(|_| {
                panic!(
                    "`{}` should be Ok because error vec is empty",
                    type_name::<Self>()
                )
            })
        })
    }
}
