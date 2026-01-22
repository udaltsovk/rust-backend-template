#![expect(
    clippy::expl_impl_clone_on_copy,
    reason = "we're just copying Option code :)"
)]

use result_like::OptionLike;

#[derive(OptionLike, Debug, Hash)]
#[must_use = "if you need a type that can be ignored, consider `Option` instead"]
pub enum Nullable<T> {
    NonNull(T),
    Null,
}

#[derive(OptionLike, Debug, Hash)]
#[must_use = "if you need a type that can be ignored, consider `Option` instead"]
pub enum Optional<T> {
    Present(T),
    Missing,
}
pub type OptionalNullable<T> = Optional<Nullable<T>>;

impl<T> OptionalNullable<T> {
    #[expect(non_upper_case_globals, reason = "we're imitating an enum")]
    pub const Missing: Self = Self::Missing;
    #[expect(non_upper_case_globals, reason = "we're imitating an enum")]
    pub const Null: Self = Self::Present(Nullable::Null);

    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }

    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Present(Nullable::Null))
    }

    pub const fn is_just(&self) -> bool {
        matches!(self, Self::Present(Nullable::NonNull(_)))
    }

    #[expect(non_snake_case, reason = "we're imitating an enum")]
    pub const fn Just(value: T) -> Self {
        Self::Present(Nullable::NonNull(value))
    }

    pub fn flatten(self) -> Option<T> {
        self.into_option()?.into_option()
    }

    pub fn flatten_into<I>(self) -> Option<I>
    where
        T: Into<I>,
    {
        self.flatten().map(T::into)
    }
}

impl<T> From<OptionalNullable<T>> for Option<T> {
    fn from(optional_nullable: OptionalNullable<T>) -> Self {
        optional_nullable.flatten()
    }
}
