use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, SqlitePool};

/// Provides access to the application database using a connection pool
pub struct PrHubDatabase {
    connection_pool: SqlitePool,
}

impl PrHubDatabase {
    /// Create a new instance to manage the access to the local application sqlite database
    ///
    /// # Arguments
    ///
    /// * `app_data_dir_path` - The path to the app data directory
    pub fn new(app_data_dir_path: &str) -> Self {
        let path = format!("{}/pr-hub-db.sqlite", app_data_dir_path);
        let connection_pool = tokio::runtime::Runtime::new()
            .expect("Tokio runtime needs to be created for syncronously setting up the connection pool at app start")
            .block_on(init_db_connection(&path))
            .expect("Could not create a connection to the database");
        Self { connection_pool }
    }

    /// Get the managed database connection pool
    pub fn get_pool(&self) -> &SqlitePool {
        &self.connection_pool
    }
}

/// Initialize the connection to the sqlite database,
/// apply pending migrations and
/// return a new connection pool as result
///
/// # Arguments
///
/// * `db_path` - The path to the sqlite file
///
/// # Returns
///
/// * `Result<SqlitePool>` - The connection pool to access the sqlite database
///
/// # Errors
///
/// Any errors that might occur
async fn init_db_connection(db_path: &str) -> Result<SqlitePool> {
    if !sqlx::Sqlite::database_exists(db_path).await? {
        sqlx::Sqlite::create_database(db_path).await?;
    }
    let pool = SqlitePool::connect(db_path).await?;
    sqlx::migrate!("src/infrastructure/database/migrations")
        .run(&pool)
        .await?;
    Ok(pool)
}
