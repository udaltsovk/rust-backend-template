use std::{fmt::Debug, str::FromStr};

use domain::{DomainType, input_impls, validation::ExternalInput};
use model_mapper::Mapper;
use serde::{Deserialize, Deserializer, Serialize};
use serde_value::{Value, ValueDeserializer};
use tap::{Conv as _, Pipe as _};
#[cfg(feature = "openapi")]
use utoipa::{
    __dev::ComposeSchema,
    ToSchema,
    openapi::{RefOr, Schema},
};

// TODO: Fix openapi
// Maybe we should consider using aide...

#[derive(Mapper, Default, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = ExternalInput, into)]
pub enum UserInput<T> {
    Ok(T),
    WrongType(Value),
    #[mapper(rename = None)]
    Null,
    #[default]
    Missing,
}

input_impls!(
    input = UserInput,
    ok = Ok,
    wrong_type = WrongType,
    value = Value,
    none = Null,
    missing = Missing
);

impl<T> UserInput<T> {
    pub fn from_domain<D, I>(domain_value: D) -> Self
    where
        D: DomainType<I>,
        I: Clone + From<D>,
        T: From<I>,
    {
        domain_value.conv::<I>().conv::<T>().pipe(Self::Ok)
    }
}

impl<'de, T> Deserialize<'de> for UserInput<T>
where
    T: Deserialize<'de> + Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value = Value::deserialize(deserializer)?;

        match Option::deserialize(ValueDeserializer::<D::Error>::new(
            raw_value.clone(),
        )) {
            Ok(Some(value)) => Self::Ok(value),
            Ok(None) => Self::Null,
            Err(_) => Self::WrongType(raw_value),
        }
        .pipe(Ok)
    }
}

impl<T> Serialize for UserInput<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Self::Ok(value) = self {
            value.serialize(serializer)
        } else {
            None::<T>.serialize(serializer)
        }
    }
}

#[cfg(feature = "openapi")]
impl<T> ComposeSchema for UserInput<T>
where
    T: ComposeSchema,
{
    fn compose(new_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        T::compose(new_generics)
    }
}

#[cfg(feature = "openapi")]
impl<T> ToSchema for UserInput<T> where T: ToSchema + ComposeSchema {}

#[derive(Serialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LossyUserInput<T>(pub UserInput<T>);

impl<T> LossyUserInput<T> {
    pub fn from_domain<D, I>(domain_value: D) -> Self
    where
        D: DomainType<I>,
        I: Clone + From<D>,
        T: From<I>,
    {
        UserInput::from_domain(domain_value).pipe(Self)
    }
}

impl<T, D> From<LossyUserInput<T>> for ExternalInput<D>
where
    Self: From<UserInput<T>>,
{
    fn from(input: LossyUserInput<T>) -> Self {
        input.0.into()
    }
}

impl<'de, T> Deserialize<'de> for LossyUserInput<T>
where
    T: Deserialize<'de> + Debug + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value = Value::deserialize(deserializer)?;

        let deserialized_option =
            Option::deserialize(ValueDeserializer::<D::Error>::new(
                raw_value.clone(),
            ));

        let raw_value_string = if let Value::String(value) = &raw_value {
            Some(value)
        } else {
            None
        };

        match (deserialized_option, raw_value_string) {
            (Ok(Some(value)), _) => UserInput::Ok(value),
            (Ok(None), _) => UserInput::Null,
            (Err(_), Some(value)) => value.parse().map_or_else(
                |_| UserInput::WrongType(raw_value),
                UserInput::Ok,
            ),
            (Err(_), _) => UserInput::WrongType(raw_value),
        }
        .pipe(Self)
        .pipe(Ok)
    }
}

#[cfg(feature = "openapi")]
impl<T> ComposeSchema for LossyUserInput<T>
where
    T: ComposeSchema,
{
    fn compose(new_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        UserInput::<T>::compose(new_generics)
    }
}

#[cfg(feature = "openapi")]
impl<T> ToSchema for LossyUserInput<T> where T: ToSchema + ComposeSchema {}
