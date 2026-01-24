#[macro_export]
macro_rules! application_result {
    ($name: ident) => {
        $crate::pastey::paste! {
            pub type [<$name Result>]<T> = Result<T, [<$name Error>]>;
        }
    };
    ($name: ident<$($generic: ident),+>) => {
        $crate::pastey::paste! {
            pub type [<$name Result>]<$($generic),+, T> = Result<T, [<$name Error>]<$($generic),+>>;
        }
    };
}
