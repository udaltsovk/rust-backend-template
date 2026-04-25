#[macro_export]
macro_rules! repository_impl_struct {
    ($name:ident) => {
        $crate::pastey::paste! {
            pub struct [< $name RepositoryImpl >];

            impl [< $name RepositoryImpl >] {
                pub fn static_ref() -> &'static Self {
                    &Self
                }
            }
        }
    };
}
