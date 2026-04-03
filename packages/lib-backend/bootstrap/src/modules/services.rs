#[macro_export]
macro_rules! impl_services {
    (
        @parse
        struct: $struct: ident $(,)?
    ) => {};

    (
        @parse
        struct: $struct: ident,
        $service: ident: |$s: ident| $body:expr
    ) => {
        impl AsRef<dyn $service<Self>> for $struct {
            fn as_ref(&self) -> &(dyn $service<Self>) {
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
        $service: ident: |$s: ident| $body: expr,
        $($rest: tt)*
    ) => {
        impl_services!(@parse struct: $struct, $service: |$s| $body);

        impl_services!(@parse struct: $struct, $($rest)*);
    };

    (
        @parse
        struct: $struct: ident,
        $delegate: ident: $implementation: ty,
        $($rest: tt)*
    ) => {
        impl_services!(@parse struct: $struct, $delegate: $implementation);

        impl_services!(@parse struct: $struct, $($rest)*);
    };

    (
        struct: $struct: ident,
        $($rest: tt)*
    ) => {
        impl_services!(@parse struct: $struct, $($rest)*);
    };
}
