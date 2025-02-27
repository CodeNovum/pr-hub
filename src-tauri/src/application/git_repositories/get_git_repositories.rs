use crate::application::{dtos::GitRepositoryDto, traits::GitRepositoryRepository};
use anyhow::Result;

/// Responsible for getting all imported git repositories
pub struct GitRepositoriesQuery<G>
where
    G: GitRepositoryRepository,
{
    git_repository_repository: G,
}

impl<G> GitRepositoriesQuery<G>
where
    G: GitRepositoryRepository,
{
    /// Create a new instance of the query
    ///
    /// # Arguments
    ///
    /// * `git_repository_repository` - The repository to for git repositories access
    pub fn new(git_repository_repository: G) -> Self {
        Self {
            git_repository_repository,
        }
    }

    /// Execute the query
    ///
    /// # Returns
    ///
    /// * `Result<Vec<GitRepositoryDto>, String>` The list of retrieved git repositories
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    pub async fn execute(&self) -> Result<Vec<GitRepositoryDto>> {
        let git_repos = self
            .git_repository_repository
            .get_all_git_repositories()
            .await?;
        let result = git_repos.iter().map(|x| x.into()).collect();
        Ok(result)
    }
}
