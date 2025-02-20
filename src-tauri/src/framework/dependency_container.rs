use crate::{
    application::traits::{AzureDevOpsRepository, GitRepositoryRepository},
    infrastructure::{
        azure_devops::repository::AzureDevOpsRestRepository,
        database::repositories::GitRepositoryDatabaseRepository,
    },
};

/// Must be implemented by factories that are being used to resolve transient dependencies
pub trait Factory<T>: Send + Sync {
    /// Produce a new instance of the given type
    ///
    /// # Arguments
    ///
    /// * `di_container` - The dependency container to resolve eventual instance dependencies
    fn produce(&self, di_container: &DependencyContainer) -> T;
}

/// Factory responsible for producing git repository repositories
struct GitRepositoryRepositoryFac;

impl Factory<Box<dyn GitRepositoryRepository>> for GitRepositoryRepositoryFac {
    fn produce(&self, _di_container: &DependencyContainer) -> Box<dyn GitRepositoryRepository> {
        Box::new(GitRepositoryDatabaseRepository {})
    }
}

/// Factory responsible for Azure DevOps repositories
struct AzureDevOpsRepositoryFac;

impl Factory<Box<dyn AzureDevOpsRepository>> for AzureDevOpsRepositoryFac {
    fn produce(&self, _di_container: &DependencyContainer) -> Box<dyn AzureDevOpsRepository> {
        Box::new(AzureDevOpsRestRepository::default())
    }
}

/// Container that manages how dependencies are resolved
pub struct DependencyContainer {
    pub git_repository_repository_fac: Box<dyn Factory<Box<dyn GitRepositoryRepository>>>,
    pub azure_devops_repository_fac: Box<dyn Factory<Box<dyn AzureDevOpsRepository>>>,
}

impl DependencyContainer {
    pub fn new() -> Self {
        Self {
            git_repository_repository_fac: Box::new(GitRepositoryRepositoryFac {}),
            azure_devops_repository_fac: Box::new(AzureDevOpsRepositoryFac {}),
        }
    }
}
