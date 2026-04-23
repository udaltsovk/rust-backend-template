use std::{fmt, marker::PhantomData};

use derive_where::derive_where;
#[doc(hidden)]
pub use pastey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod validation;

#[derive_where(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id<T> {
    pub value: Uuid,
    _entity: PhantomData<T>,
}

impl<T> Id<T> {
    #[must_use]
    pub const fn new(value: Uuid) -> Self {
        Self {
            value,
            _entity: PhantomData,
        }
    }

    #[must_use]
    pub fn generate() -> Self {
        Self::new(Uuid::now_v7())
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(id: Id<T>) -> Self {
        id.value
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(Self::from)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

pub trait DomainType<T>: AsRef<T> + AsMut<T>
where
    Self: Sized,
    T: From<Self> + Clone,
{
    fn into_inner(self) -> T {
        self.into()
    }

    fn cloned_inner(&self) -> T {
        self.as_ref().clone()
    }

    #[must_use]
    fn it_should_be_safe_to_unwrap<E>()
    -> impl FnOnce(E) -> T {
        move |_| {
            panic!(
                "We've validated field value, so it \
                 should be safe"
            )
        }
    }
}
