use mobc_sqlx::{mobc::Error as MobcError, sqlx::Error as SqlxError};

#[derive(thiserror::Error, Debug)]
pub enum PostgresAdapterError {
    #[error(transparent)]
    Pool(#[from] MobcError<SqlxError>),

    #[error(transparent)]
    Database(#[from] SqlxError),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{MobcError, PostgresAdapterError, SqlxError};

    #[rstest]
    fn postgres_adapter_error_from_mobc_error() {
        let mobc_error = MobcError::Inner(SqlxError::RowNotFound);
        let error_display = mobc_error.to_string();

        let error: PostgresAdapterError = mobc_error.into();

        assert!(matches!(error, PostgresAdapterError::Pool(_)));
        assert_eq!(error.to_string(), error_display);
    }

    #[rstest]
    fn postgres_adapter_error_from_sqlx_error() {
        let sqlx_error = SqlxError::RowNotFound;
        let error_display = sqlx_error.to_string();

        let error: PostgresAdapterError = sqlx_error.into();

        assert!(matches!(error, PostgresAdapterError::Database(_)));
        assert_eq!(error.to_string(), error_display);
    }

    #[rstest]
    fn postgres_adapter_error_debug() {
        let error = PostgresAdapterError::Database(SqlxError::RowNotFound);
        let debug_output = format!("{error:?}");

        assert!(debug_output.contains("Database"));
        assert!(debug_output.contains("RowNotFound"));
    }
}
