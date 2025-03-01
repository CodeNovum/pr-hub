use anyhow::Result;
use sqlx::{SqlitePool, migrate::MigrateDatabase};

/// Initialize the connection to the sqlite database,
/// apply pending migrations and
/// return a new connection pool as result
///
/// # Arguments
///
/// * `app_data_dir_path` - The path to the app data dir
///
/// # Returns
///
/// * `Result<SqlitePool>` - The connection pool to access the sqlite database
///
/// # Errors
///
/// Any errors that might occur
pub async fn init_db_connection(app_data_dir_path: &str) -> Result<SqlitePool> {
    let db_path = &format!("{}/pr-hub-db.sqlite", app_data_dir_path);
    if !sqlx::Sqlite::database_exists(db_path).await? {
        sqlx::Sqlite::create_database(db_path).await?;
    }
    let pool = SqlitePool::connect(db_path).await?;
    sqlx::migrate!("src/database/migrations").run(&pool).await?;
    Ok(pool)
}
