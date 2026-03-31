#[macro_export]
macro_rules! impl_repositories {
    (
        struct: $struct: ident,
        $($repository: ident: |$s: ident| $body: expr),* $(,)?
    ) => {
        $(
            impl AsRef<dyn $repository<Self> + Sync> for $struct {
                fn as_ref(&self) -> &(dyn $repository<Self> + Sync) {
                    let $s = self;
                    $body
                }
            }
        )*
    };
}
