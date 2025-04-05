// Database schema definition
use sqlx::sqlite::SqlitePool;

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create tables if they don't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
