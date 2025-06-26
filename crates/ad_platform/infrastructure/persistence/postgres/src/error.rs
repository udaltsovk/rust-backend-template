#[derive(thiserror::Error, Debug)]
pub enum PostgresAdapterError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
