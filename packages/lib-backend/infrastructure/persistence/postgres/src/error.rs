use mobc_sqlx::{mobc, sqlx};

#[derive(thiserror::Error, Debug)]
pub enum PostgresAdapterError {
    #[error(transparent)]
    Pool(#[from] mobc::Error<sqlx::Error>),

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
