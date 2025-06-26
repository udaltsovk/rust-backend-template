#[derive(thiserror::Error, Debug)]
pub enum UseCaseError<E> {
    #[error(transparent)]
    Adapter(#[from] E),
}
