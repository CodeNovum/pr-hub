use crate::{
    application::traits::{AzureDevOpsRepository, GitRepositoryRepository, SecretRepository},
    infrastructure::{
        azure_devops::repository::AzureDevOpsRestRepository,
        database::{connection::PrHubDatabase, repositories::GitRepositoryDatabaseRepository},
        secret_storage::KeyringRepository,
    },
};
use std::sync::Arc;

/// Must be implemented by factories that are being used to resolve transient dependencies
pub trait Factory<T>: Send + Sync {
    /// Produce a new instance of the given type
    ///
    /// # Arguments
    ///
    /// * `di_container` - The dependency container to resolve dependencies
    fn produce(&self, di_container: &DependencyContainer) -> T;
}

/// Factory responsible for producing git repository repositories
struct GitRepositoryRepositoryFac;

impl Factory<Box<dyn GitRepositoryRepository>> for GitRepositoryRepositoryFac {
    fn produce(&self, di_container: &DependencyContainer) -> Box<dyn GitRepositoryRepository> {
        Box::new(GitRepositoryDatabaseRepository::new(Arc::clone(
            &di_container.database_access,
        )))
    }
}

/// Factory responsible for Azure DevOps repositories
struct AzureDevOpsRepositoryFac;

impl Factory<Box<dyn AzureDevOpsRepository>> for AzureDevOpsRepositoryFac {
    fn produce(&self, _di_container: &DependencyContainer) -> Box<dyn AzureDevOpsRepository> {
        Box::new(AzureDevOpsRestRepository::default())
    }
}

/// Factory responsible for secret repositories
struct SecretRepositoryFac;

impl Factory<Box<dyn SecretRepository>> for SecretRepositoryFac {
    fn produce(&self, _di_container: &DependencyContainer) -> Box<dyn SecretRepository> {
        Box::new(KeyringRepository::new("pr-hub".to_string()))
    }
}

/// Container that manages how dependencies are resolved
pub struct DependencyContainer {
    pub database_access: Arc<PrHubDatabase>,
    pub git_repository_repository_fac: Box<dyn Factory<Box<dyn GitRepositoryRepository>>>,
    pub azure_devops_repository_fac: Box<dyn Factory<Box<dyn AzureDevOpsRepository>>>,
    pub secret_repository_fac: Box<dyn Factory<Box<dyn SecretRepository>>>,
}

impl DependencyContainer {
    pub fn new(app_data_dir_path: &str) -> Self {
        let database_access = Arc::new(PrHubDatabase::new(app_data_dir_path));
        Self {
            database_access,
            git_repository_repository_fac: Box::new(GitRepositoryRepositoryFac {}),
            azure_devops_repository_fac: Box::new(AzureDevOpsRepositoryFac {}),
            secret_repository_fac: Box::new(SecretRepositoryFac {}),
        }
    }
}
