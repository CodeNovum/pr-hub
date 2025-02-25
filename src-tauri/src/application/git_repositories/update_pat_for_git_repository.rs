use crate::application::traits::{GitRepositoryRepository, SecretRepository};
use anyhow::Result;

/// Responsible for updating the stored PAT for a single imported git repository
pub struct UpdatePatForGitRepositoryCommand {
    git_repository_repository: Box<dyn GitRepositoryRepository>,
    secret_repository: Box<dyn SecretRepository>,
}

impl UpdatePatForGitRepositoryCommand {
    /// Create a new instance of the command
    ///
    /// # Arguments
    ///
    /// * `git_repository_repository` - The repository to access git repositories
    /// * `secret_repository` - The repository to access secrets
    pub fn new(
        git_repository_repository: Box<dyn GitRepositoryRepository>,
        secret_repository: Box<dyn SecretRepository>,
    ) -> Self {
        Self {
            git_repository_repository,
            secret_repository,
        }
    }

    /// Execute the command
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the targeted git repository
    /// * `pat` - The PAT to store
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    pub async fn execute(&self, id: &u32, pat: &str) -> Result<()> {
        let git_repo = self
            .git_repository_repository
            .get_git_repository_by_id(id)
            .await?;
        self.secret_repository
            .set_secret(&git_repo.pat_secret_key, pat)?;
        Ok(())
    }
}
