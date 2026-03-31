#[macro_export]
macro_rules! impl_services {
    (
        struct: $struct: ident,
        $($service: ident: |$s: ident| $body: expr),* $(,)?
    ) => {
        $(
            impl AsRef<dyn $service<Self>> for $struct {
                fn as_ref(&self) -> &(dyn $service<Self>) {
                    let $s = self;
                    $body
                }
            }
        )*
    };
}
