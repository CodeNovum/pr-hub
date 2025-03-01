use crate::traits::{AzureDevOpsRepository, GitRepositoryRepository, SecretRepository};
use anyhow::Result;

/// Responsible for importing all git repositories from a single
/// Azure DevOps organization
pub struct DevOpsOrgaImporter<A, G, S>
where
    A: AzureDevOpsRepository,
    G: GitRepositoryRepository,
    S: SecretRepository,
{
    azure_devops_repository: A,
    git_repository_repository: G,
    secret_repository: S,
}

impl<A, G, S> DevOpsOrgaImporter<A, G, S>
where
    A: AzureDevOpsRepository,
    G: GitRepositoryRepository,
    S: SecretRepository,
{
    /// Create a new instance of the importer
    ///
    /// # Arguments
    ///
    /// * `azure_devops_repository` - The repository to get the git repositories from Azure DevOps
    /// * `git_repository_repository` - The repository to access git repositories
    /// * `secret_repository` - The repository to access secrets
    pub fn new(
        azure_devops_repository: A,
        git_repository_repository: G,
        secret_repository: S,
    ) -> Self {
        Self {
            azure_devops_repository,
            git_repository_repository,
            secret_repository,
        }
    }

    /// Import all the git repositories by querying the from the Azure
    /// DevOps organization, store them in the database and store the
    /// PAT in the secret storage
    ///
    /// # Arguments
    ///
    /// * `organization_name` - The name of the Azure DevOps organization
    /// * `pat` - The PAT to access all git repositories
    pub async fn import(&self, organization_name: &str, pat: &str) -> Result<()> {
        let git_repositories = self
            .azure_devops_repository
            .get_repositories_in_organization(pat, organization_name)
            .await?;
        if let Some(first) = git_repositories.first() {
            // When importing a whole organization, all repositories share the same
            // PAT and therefore only one secret needs to be stored
            self.secret_repository
                .set_secret(&first.pat_secret_key, pat)?;
            // Store all the repositories after the secret is created
            for gr in git_repositories {
                self.git_repository_repository
                    .create_git_repository(gr)
                    .await?;
            }
        }
        Ok(())
    }
}
