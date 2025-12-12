use mobc_sqlx::{mobc, sqlx};

#[derive(thiserror::Error, Debug)]
pub enum PostgresAdapterError {
    #[error(transparent)]
    Pool(#[from] mobc::Error<sqlx::Error>),

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

#[cfg(test)]
mod tests {
    use mobc_sqlx::{mobc, sqlx};
    use rstest::rstest;

    use super::PostgresAdapterError;

    #[rstest]
    fn postgres_adapter_error_from_mobc_error() {
        let mobc_error = mobc::Error::Inner(sqlx::Error::RowNotFound);
        let error_display = mobc_error.to_string();

        let error: PostgresAdapterError = mobc_error.into();

        assert!(matches!(error, PostgresAdapterError::Pool(_)));
        assert_eq!(error.to_string(), error_display);
    }

    #[rstest]
    fn postgres_adapter_error_from_sqlx_error() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let error_display = sqlx_error.to_string();

        let error: PostgresAdapterError = sqlx_error.into();

        assert!(matches!(error, PostgresAdapterError::Database(_)));
        assert_eq!(error.to_string(), error_display);
    }

    #[rstest]
    fn postgres_adapter_error_debug() {
        let error = PostgresAdapterError::Database(sqlx::Error::RowNotFound);
        let debug_output = format!("{error:?}");

        assert!(debug_output.contains("Database"));
        assert!(debug_output.contains("RowNotFound"));
    }
}
