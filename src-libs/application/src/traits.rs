use anyhow::Result;
use async_trait::async_trait;
use domain::models::{GitRepository, PullRequest};

/// Must be implemented by repositories responsible
/// for accessing git repository data
#[async_trait]
pub trait GitRepositoryRepository: Send + Sync {
    /// Get all imported git repositories
    ///
    /// # Returns
    ///
    /// * `Result<Vec<GitRepository>>` - The list of found git repositories
    ///
    /// # Errors
    ///
    /// Any error that might occur
    async fn get_all_git_repositories(&self) -> Result<Vec<GitRepository>>;

    /// Receive a single git repository by it's unique identifier
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the git repository
    ///
    /// # Returns
    ///
    /// * `Result<GitRepository>` - The found git repository
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    async fn get_git_repository_by_id(&self, id: &u32) -> Result<GitRepository>;

    /// Updates a single git repository
    ///
    /// # Arguments
    ///
    /// * `git_repository` - The model of the updated git repository
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    async fn update_git_repository(&self, git_repository: GitRepository) -> Result<()>;

    /// Creates a single git repository
    ///
    /// # Arguments
    ///
    /// * `git_repository` - The model of the git repository
    ///
    /// # Errors
    ///
    /// Any error that might occur
    async fn create_git_repository(&self, git_repository: GitRepository) -> Result<()>;

    /// Deletes a single git repository
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the git repository
    ///
    /// # Errors
    ///
    /// Any error that might occur
    async fn delete_git_repository(&self, id: &u32) -> Result<()>;
}

/// Must be implemented by repositories responsible
/// for accessing data from Azure DevOps
#[async_trait]
pub trait AzureDevOpsRepository: Send + Sync {
    /// Get all open pull requests of a singe Azure DevOps git repository
    ///
    /// # Arguments
    ///
    /// * `pat` - The private access token to authenticate the REST API request
    /// * `context` - The request context in the format ORGANIZATION_NAME/PROJECT_NAME
    /// * `repository_name` - The name of the git repository to query data for
    ///
    /// # Returns
    ///
    /// * `Result<Vec<PullRequest>>` - The list of retrieved pull requests
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    async fn get_open_pull_requests_in_repository(
        &self,
        pat: &str,
        context: &str,
        repository_name: &str,
    ) -> Result<Vec<PullRequest>>;

    /// Get all git repositories inside a single Azure DevOps organization
    ///
    /// # Arguments
    ///
    /// * `pat` - The private access token to authenticate the REST API request
    /// * `organization_name` - The name of the Azure DevOps organization
    ///
    /// # Returns
    ///
    /// * `Result<Vec<GitRepository>>` - The list of retrieved git repositories
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    async fn get_repositories_in_organization(
        &self,
        pat: &str,
        organization_name: &str,
    ) -> Result<Vec<GitRepository>>;
}

/// Must be implemented by repositories that manage secrets
pub trait SecretRepository: Send + Sync {
    /// Retrieve a secret by key
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the secret
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The value of the secret
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    fn get_secret(&self, key: &str) -> Result<String>;

    /// Store a secret
    ///
    /// # Arguments
    ///
    /// * `key` - The key that later can be used to access the secret
    /// * `value` - The secret itself
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    fn set_secret(&self, key: &str, value: &str) -> Result<()>;

    /// Deletes a secret
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the secret
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    fn delete_secret(&self, key: &str) -> Result<()>;
}
