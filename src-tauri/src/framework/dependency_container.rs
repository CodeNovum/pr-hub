use crate::infrastructure::{
    azure_devops::repository::AzureDevOpsRestRepository,
    database::{connection::init_db_connection, repositories::GitRepositoryDatabaseRepository},
    secret_storage::KeyringRepository,
};
use sqlx::SqlitePool;
use std::sync::Arc;

/// Container that manages how dependencies are resolved
pub struct DependencyContainer {
    pub database_connection_pool: Arc<SqlitePool>,
    pub git_repository_repository_fac: fn(&Self) -> GitRepositoryDatabaseRepository,
    pub azure_devops_repository_fac: fn() -> AzureDevOpsRestRepository,
    pub secret_repository_fac: fn() -> KeyringRepository,
}

impl DependencyContainer {
    pub fn new(app_data_dir_path: &str) -> Self {
        let database_connection_pool = tokio::runtime::Runtime::new()
            .expect("Tokio runtime needs to be created for syncronously setting up the connection pool at app start")
            .block_on(init_db_connection(app_data_dir_path))
            .expect("Could not create a connection to the database");
        Self {
            database_connection_pool: Arc::new(database_connection_pool),
            git_repository_repository_fac: |di_container| {
                GitRepositoryDatabaseRepository::new(Arc::clone(
                    &di_container.database_connection_pool,
                ))
            },
            azure_devops_repository_fac: || AzureDevOpsRestRepository::default(),
            secret_repository_fac: || KeyringRepository::new("pr-hub".to_string()),
        }
    }
}
