#[macro_export]
macro_rules! impl_repositories {
    (
        @parse
        struct: $struct:ident $(,)?
    ) => {};

    (
        @parse
        struct: $struct: ident,
        $repository: ident: |$s: ident| $body: expr
    ) => {
        impl AsRef<dyn $repository<Self> + Sync> for $struct {
            fn as_ref(&self) -> &(dyn $repository<Self> + Sync) {
                let $s = self;
                $body
            }
        }
    };

    (
        @parse
        struct: $struct: ident,
        $delegate: ident: $implementation: ty
    ) => {
        impl $delegate<Self> for $struct {
            type Target = $implementation;
        }
    };

    (
        @parse
        struct: $struct: ident,
        $repository: ident: |$s: ident| $body: expr,
        $($rest: tt)*
    ) => {
        impl_repositories!(@parse struct: $struct, $repository: |$s| $body);

        impl_repositories!(@parse struct: $struct, $($rest)*);
    };

    (
        @parse
        struct: $struct: ident,
        $delegate: ident: $implementation: ty,
        $($rest: tt)*
    ) => {
        impl_repositories!(@parse struct: $struct, $delegate: $implementation);

        impl_repositories!(@parse struct: $struct, $($rest)*);
    };

    (
        struct: $struct: ident,
        $($rest: tt)*
    ) => {
        impl_repositories!(@parse struct: $struct, $($rest)*);
    };
}
