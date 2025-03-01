use crate::traits::{GitRepositoryRepository, SecretRepository};
use anyhow::Result;

/// Responsible for removing an imported git repository
pub struct RemoveGitRepositoryCommand<G, S>
where
    G: GitRepositoryRepository,
    S: SecretRepository,
{
    git_repository_repository: G,
    secret_repository: S,
}

impl<G, S> RemoveGitRepositoryCommand<G, S>
where
    G: GitRepositoryRepository,
    S: SecretRepository,
{
    /// Create a new instance of the command
    ///
    /// # Arguments
    ///
    /// * `git_repository_repository` - The repository to access git repositories
    /// * `secret_repository` - The repository to access secrets
    pub fn new(git_repository_repository: G, secret_repository: S) -> Self {
        Self {
            git_repository_repository,
            secret_repository,
        }
    }

    /// Execute the command
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the repository to remove
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    pub async fn execute(&self, id: &u32) -> Result<()> {
        let git_repos = self
            .git_repository_repository
            .get_all_git_repositories()
            .await?;
        let target_repo = git_repos
            .iter()
            .find(|x| x.id == *id)
            .ok_or_else(|| anyhow::anyhow!("No git repository with id found"))?;
        if git_repos
            .iter()
            .filter(|x| x.pat_secret_key == target_repo.pat_secret_key)
            .count()
            == 1
        {
            // Only remove the secret if the git repository to remove is the
            // last one referencing it
            let _ = self
                .secret_repository
                .delete_secret(&target_repo.pat_secret_key);
        }
        self.git_repository_repository
            .delete_git_repository(id)
            .await?;
        Ok(())
    }
}
