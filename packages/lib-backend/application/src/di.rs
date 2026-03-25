use std::ops::Deref;

use entrait::entrait;

#[derive(Clone, Copy)]
pub struct DependencyContainer<D>(D);

impl<D> From<D> for DependencyContainer<D> {
    fn from(value: D) -> Self {
        Self(value)
    }
}

impl<D> Deref for DependencyContainer<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[entrait(pub Has)]
fn get_dependency<D>(dependency: &DependencyContainer<D>) -> &D {
    &dependency.0
}

#[macro_export]
macro_rules! impl_Has {
    (
        struct: $struct: ident,
        $($dependency: ty: |$s: ident| $body: expr),* $(,)?
    ) => {
        $(
            impl $crate::di::Has<$dependency> for $struct {
                fn get_dependency(&self) -> &$dependency {
                    let $s = self;
                    $body
                }
            }
        )*
    };
}
