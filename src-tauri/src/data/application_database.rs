use lazy_static::lazy_static;
use rusqlite::Connection;
use std::{fs, sync::RwLock};

lazy_static! {
    static ref BASE_PATH: RwLock<String> = RwLock::new("".to_string());
}

const DATABASE_FILE_NAME: &str = "app.db";

/// Retrieve a connection to the local application database
///
/// # Returns
///
/// The connection to the local application database.
///
pub fn get_db_connection() -> Connection {
    let base_path = BASE_PATH.read().unwrap();
    let mut path = "".to_string();
    path.push_str(&base_path);
    _ = fs::create_dir_all(path.clone());
    path.push('/');
    path.push_str(DATABASE_FILE_NAME);
    Connection::open(path).unwrap()
}

/// Initialize the local application database
///
/// # Arguments
///
/// * `path` - The path to the database on the local file system
///
pub fn init_db(path: &str) {
    {
        let mut mystr = BASE_PATH.write().unwrap();
        *mystr = path.to_string();
    }
    {
        let db_connection: Connection = get_db_connection();
        _ = db_connection.execute(
            "CREATE TABLE IF NOT EXISTS organizations (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL UNIQUE
            )",
            (),
        );
    }
}
