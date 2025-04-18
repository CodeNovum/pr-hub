use crate::traits::GitRepositoryRepository;
use anyhow::Result;

/// Responsible for toggling the flag indicating whether an imported
/// git repository is considered `active` or not
pub struct ToggleGitRepositoryActiveStateCommand<G>
where
    G: GitRepositoryRepository,
{
    git_repository_repository: G,
}

impl<G> ToggleGitRepositoryActiveStateCommand<G>
where
    G: GitRepositoryRepository,
{
    /// Create a new instance of the command
    ///
    /// # Arguments
    ///
    /// * `git_repository_repository` - The repository to access git repositories
    pub fn new(git_repository_repository: G) -> Self {
        Self {
            git_repository_repository,
        }
    }

    pub async fn execute(&self, id: &u32) -> Result<()> {
        let mut git_repo = self
            .git_repository_repository
            .get_git_repository_by_id(id)
            .await?;
        git_repo.is_active = !git_repo.is_active;
        self.git_repository_repository
            .update_git_repository(git_repo)
            .await?;
        Ok(())
    }
}
