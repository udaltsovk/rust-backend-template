#![expect(internal_features, reason = "We desperately need nonzero")]
#![feature(trait_alias, nonzero_internals)]

use std::{
    fmt,
    hash::Hash,
    marker::PhantomData,
    num::{NonZero, ZeroablePrimitive},
};

#[doc(hidden)]
pub use pastey;
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "id-uuidv4", feature = "id-uuidv7"))]
use uuid::Uuid;

pub mod validation;

pub trait IdRepr = Copy
    + Clone
    + fmt::Debug
    + PartialEq
    + Eq
    + Hash
    + fmt::Display
    + Serialize
    + for<'de> Deserialize<'de>;

pub trait GenerateId: IdRepr {
    fn generate() -> Self;
}

#[cfg(any(feature = "id-uuidv4", feature = "id-uuidv7"))]
impl GenerateId for Uuid {
    fn generate() -> Self {
        cfg_select! {
            feature = "id-uuidv4" => Self::new_v4(),
            _ => Self::now_v7(),
        }
    }
}

type DefaultIdRepr = cfg_select! {
    feature = "id-uuidv4" => Uuid,
    feature = "id-uuidv7" => Uuid,
    all(feature = "id-uuidv4", feature = "id-uuidv7") => {
        compile_error!("only one id UUID version feature may be selected")
    },
    _ => NonZero::<i64>,
};

pub struct Id<T, R = DefaultIdRepr> {
    pub value: R,
    _entity: PhantomData<T>,
}

impl<T, R: IdRepr> Copy for Id<T, R> {}

impl<T, R: IdRepr> Clone for Id<T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, R: IdRepr> fmt::Debug for Id<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Id").field("value", &self.value).finish()
    }
}

impl<T, R: IdRepr> PartialEq for Id<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, R: IdRepr> Eq for Id<T, R> {}

impl<T, R: IdRepr> Hash for Id<T, R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T, R: IdRepr> Id<T, R> {
    #[must_use]
    pub const fn new(value: R) -> Self {
        Self {
            value,
            _entity: PhantomData,
        }
    }

    #[must_use]
    pub const fn into_value(self) -> R {
        self.value
    }
}

impl<T, R: GenerateId> Id<T, R> {
    #[must_use]
    pub fn generate() -> Self {
        Self::new(R::generate())
    }
}

impl<T, R: IdRepr> From<R> for Id<T, R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

#[cfg(any(feature = "id-uuidv4", feature = "id-uuidv7"))]
impl<T> From<Id<T, Self>> for Uuid {
    fn from(id: Id<T, Self>) -> Self {
        id.value
    }
}

impl<T, N> From<Id<T, Self>> for NonZero<N>
where
    N: ZeroablePrimitive,
{
    fn from(id: Id<T, Self>) -> Self {
        id.value
    }
}

impl<T, R: IdRepr> fmt::Display for Id<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl<'de, T, R: IdRepr> Deserialize<'de> for Id<T, R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        R::deserialize(deserializer).map(Self::from)
    }
}

impl<T, R: IdRepr> Serialize for Id<T, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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
}
