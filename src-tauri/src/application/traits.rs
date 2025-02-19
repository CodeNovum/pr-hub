use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::{GitRepository, PullRequest};

/// Must be implemented by repositories responsible
/// for accessing git repository data
pub trait GitRepositoryRepository {}

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
    /// # Errors
    ///
    /// Any errors that might occur
    async fn get_repositories_in_organization(
        &self,
        pat: &str,
        organization_name: &str,
    ) -> Result<Vec<GitRepository>>;
}
