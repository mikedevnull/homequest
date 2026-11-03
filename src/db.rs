use sqlx::sqlite::SqliteConnectOptions;

#[derive(Clone, Debug)]
pub struct DatabaseOptions {
    pub sqlite: SqliteConnectOptions,
}

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: sqlx::SqlitePool,
}

impl Database {
    pub async fn new(options: DatabaseOptions) -> Result<Self, sqlx::Error> {
        let pool = sqlx::SqlitePool::connect_with(options.sqlite.clone()).await?;
        Ok(Database { pool })
    }
}
