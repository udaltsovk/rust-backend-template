use mobc_sqlx::{SqlxConnectionManager, mobc::Pool};

pub type SqlxPool<DB> = Pool<SqlxConnectionManager<DB>>;
