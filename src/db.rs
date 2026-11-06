use sqlx::SqlitePool;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

pub async fn migrate(db: SqlitePool) -> Result<(), sqlx::Error> {
    MIGRATOR.run(&db).await?;
    Ok(())
}
