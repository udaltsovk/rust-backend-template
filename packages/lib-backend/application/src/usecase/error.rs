#[macro_export]
macro_rules! usecase_result {
    ($name: ident) => {
        pastey::paste! {
            pub type [<$name UseCaseResult>]<R, S, T> = Result<T, [<$name UseCaseError>]<R, S>>;
        }
    };
}
