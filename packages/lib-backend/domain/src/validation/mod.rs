pub mod constraints;
pub mod error;
mod into_validators;
mod option_like;
mod try_from_external_input;
mod try_from_string;
mod validator;

pub use constraints::Constraints;
pub use option_like::{Nullable, Optional, OptionalNullable};
use serde_value::Value;
#[doc(hidden)]
pub use try_from_external_input::get_type_name;
pub use validator::{IntoValidator, Validator};

use crate::input_impls;

#[derive(Clone, Copy)]
pub struct ValidationConfirmation(());

#[derive(Default, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[must_use]
pub enum ExternalInput<T> {
    Ok(T),
    WrongType(Value),
    None,
    #[default]
    Missing,
}

input_impls!(
    input = ExternalInput,
    ok = Ok,
    wrong_type = WrongType,
    value = Value,
    none = None,
    missing = Missing
);

#[doc(hidden)]
#[macro_export]
macro_rules! input_impls {
    (
        input = $input: ident,
        ok = $ok: ident,
        wrong_type = $wrong_type: ident,
        value = $value: ident,
        none = $none: ident,
        missing = $missing: ident $(,)*
    ) => {
        impl<T> $input<T> {
            #[inline]
            pub const fn is_ok(&self) -> bool {
                matches!(self, Self::$ok(_))
            }

            #[inline]
            pub fn is_ok_and<F>(self, f: F) -> bool
            where
                F: FnOnce(T) -> bool,
            {
                match self {
                    Self::$ok(value) => f(value),
                    _ => false,
                }
            }

            #[inline]
            pub fn ok(self) -> Option<T> {
                match self {
                    Self::$ok(value) => Some(value),
                    _ => None,
                }
            }

            #[inline]
            pub const fn is_wrong_type(&self) -> bool {
                matches!(self, Self::$wrong_type(_))
            }

            #[inline]
            pub fn is_wrong_type_and<F>(self, f: F) -> bool
            where
                F: FnOnce($value) -> bool,
            {
                match self {
                    Self::$wrong_type(value) => f(value),
                    _ => false,
                }
            }

            #[inline]
            pub fn wrong_type(self) -> Option<$value> {
                match self {
                    Self::$wrong_type(value) => Some(value),
                    _ => None,
                }
            }

            #[inline]
            pub const fn is_none(&self) -> bool {
                matches!(self, Self::$none)
            }

            #[inline]
            pub const fn is_missing(&self) -> bool {
                matches!(self, Self::$missing)
            }

            #[inline]
            pub fn as_ref(&self) -> $input<&T> {
                use $input as I;
                match self {
                    Self::$ok(value) => I::$ok(value),
                    Self::$wrong_type(value) => I::$wrong_type(value.clone()),
                    Self::$none => I::$none,
                    Self::$missing => I::$missing,
                }
            }

            #[inline]
            pub fn as_mut(&mut self) -> $input<&mut T> {
                use $input as I;
                match self {
                    Self::$ok(value) => I::$ok(value),
                    Self::$wrong_type(value) => I::$wrong_type(value.clone()),
                    Self::$none => I::$none,
                    Self::$missing => I::$missing,
                }
            }

            #[inline]
            pub fn map<U, F>(self, op: F) -> $input<U>
            where
                F: FnOnce(T) -> U,
            {
                use $input as I;
                match self {
                    Self::$ok(value) => I::$ok(op(value)),
                    Self::$wrong_type(value) => I::$wrong_type(value),
                    Self::$none => I::$none,
                    Self::$missing => I::$missing,
                }
            }

            #[inline]
            pub fn map_or<U, F>(self, default: U, f: F) -> U
            where
                F: FnOnce(T) -> U,
            {
                match self {
                    Self::$ok(t) => f(t),
                    _ => default,
                }
            }

            #[inline]
            pub fn map_or_else<U, F, W, N, M>(
                self,
                f: F,
                wrong_type: W,
                none: N,
                missing: M,
            ) -> U
            where
                F: FnOnce(T) -> U,
                W: FnOnce($value) -> U,
                N: FnOnce() -> U,
                M: FnOnce() -> U,
            {
                match self {
                    Self::$ok(t) => f(t),
                    Self::$wrong_type(e) => wrong_type(e),
                    Self::$none => none(),
                    Self::$missing => missing(),
                }
            }

            #[inline]
            pub fn map_or_default<U, F>(self, f: F) -> U
            where
                F: FnOnce(T) -> U,
                U: Default,
            {
                match self {
                    Self::$ok(t) => f(t),
                    _ => U::default(),
                }
            }

            #[inline]
            pub fn inspect<F>(self, f: F) -> Self
            where
                F: FnOnce(&T),
            {
                if let Self::$ok(t) = &self {
                    f(t);
                }

                self
            }

            #[inline]
            pub fn inspect_mismatched_type<F>(self, f: F) -> Self
            where
                F: FnOnce(&$value),
            {
                if let Self::$wrong_type(e) = &self {
                    f(e);
                }

                self
            }

            #[inline]
            pub fn as_deref(&self) -> $input<&T::Target>
            where
                T: std::ops::Deref,
            {
                self.as_ref().map(T::deref)
            }

            #[inline]
            pub fn as_deref_mut(&mut self) -> $input<&mut T::Target>
            where
                T: std::ops::DerefMut,
            {
                self.as_mut().map(T::deref_mut)
            }

            #[inline]
            pub fn and<U>(self, inp: $input<U>) -> $input<U> {
                use $input as I;
                match self {
                    Self::$ok(_) => inp,
                    Self::$wrong_type(value) => I::$wrong_type(value),
                    Self::$none => I::$none,
                    Self::$missing => I::$missing,
                }
            }

            #[inline]
            pub fn and_then<U, F>(self, op: F) -> $input<U>
            where
                F: FnOnce(T) -> $input<U>,
            {
                use $input as I;
                match self {
                    Self::$ok(value) => op(value),
                    Self::$wrong_type(value) => I::$wrong_type(value),
                    Self::$none => I::$none,
                    Self::$missing => I::$missing,
                }
            }

            #[inline]
            pub fn or<F>(self, inp: Self) -> Self {
                match self {
                    Self::$ok(v) => Self::$ok(v),
                    _ => inp,
                }
            }
        }

        impl<T> $input<&T> {
            #[inline]
            pub fn copied(self) -> $input<T>
            where
                T: Copy,
            {
                self.map(|&value| value)
            }

            #[inline]
            pub fn cloned(self) -> $input<T>
            where
                T: Clone,
            {
                self.map(T::clone)
            }
        }

        impl<T> $input<&mut T> {
            #[inline]
            pub fn copied(self) -> $input<T>
            where
                T: Copy,
            {
                self.map(|&mut value| value)
            }

            #[inline]
            pub fn cloned(self) -> $input<T>
            where
                T: Clone,
            {
                self.map(|value| value.clone())
            }
        }

        impl<T> $input<Option<T>> {
            #[inline]
            pub fn transpose(self) -> Option<$input<T>> {
                use $input as I;
                match self {
                    Self::$ok(Some(value)) => Some(I::$ok(value)),
                    Self::$ok(None) => None,
                    Self::$wrong_type(value) => Some(I::$wrong_type(value)),
                    Self::$none => Some(I::$none),
                    Self::$missing => Some(I::$missing),
                }
            }
        }

        impl<T> Clone for $input<T>
        where
            T: Clone,
        {
            #[inline]
            fn clone(&self) -> Self {
                self.as_ref().cloned()
            }
        }

        impl<T> From<T> for $input<T> {
            fn from(value: T) -> Self {
                Self::$ok(value)
            }
        }

        impl<T> From<Option<Option<T>>> for $input<T> {
            fn from(double_option: Option<Option<T>>) -> Self {
                double_option.map_or(Self::$missing, |option| {
                    option.map_or(Self::$none, Self::$ok)
                })
            }
        }
    };
}
