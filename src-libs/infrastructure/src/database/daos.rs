use domain::{enums::GitProvider as DomainGitProvider, models::GitRepository};
use sqlx::{FromRow, Type};

#[derive(Type, Clone)]
#[sqlx(type_name = "git_provider", rename_all = "lowercase")]
pub enum GitProvider {
    AzureDevOps,
}

impl From<GitProvider> for DomainGitProvider {
    fn from(value: GitProvider) -> Self {
        match value {
            GitProvider::AzureDevOps => DomainGitProvider::AzureDevOps,
        }
    }
}

impl From<DomainGitProvider> for GitProvider {
    fn from(value: DomainGitProvider) -> Self {
        match value {
            DomainGitProvider::AzureDevOps => GitProvider::AzureDevOps,
        }
    }
}

#[derive(FromRow, Clone)]
pub struct GitRepositoryDao {
    pub id: u32,
    pub name: String,
    pub context: String,
    pub is_active: bool,
    pub git_provider: GitProvider,
    pub pat_secret_key: String,
}

impl From<&GitRepositoryDao> for GitRepository {
    fn from(value: &GitRepositoryDao) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            context: value.context.to_string(),
            git_provider: value.git_provider.clone().into(),
            is_active: value.is_active,
            pat_secret_key: value.pat_secret_key.to_string(),
        }
    }
}

impl From<GitRepositoryDao> for GitRepository {
    fn from(value: GitRepositoryDao) -> Self {
        Self {
            id: value.id,
            name: value.name,
            context: value.context,
            git_provider: value.git_provider.into(),
            is_active: value.is_active,
            pat_secret_key: value.pat_secret_key,
        }
    }
}

impl From<GitRepository> for GitRepositoryDao {
    fn from(value: GitRepository) -> Self {
        Self {
            id: value.id,
            name: value.name,
            context: value.context,
            git_provider: value.git_provider.into(),
            is_active: value.is_active,
            pat_secret_key: value.pat_secret_key,
        }
    }
}
