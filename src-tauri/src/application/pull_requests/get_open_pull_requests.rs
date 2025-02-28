use crate::{
    application::{
        dtos::PullRequestDto,
        traits::{AzureDevOpsRepository, GitRepositoryRepository, SecretRepository},
    },
    domain::{
        enums::GitProvider,
        models::{GitRepository, PullRequest},
    },
};
use anyhow::Result;
use std::sync::Arc;
use tokio::task::JoinSet;

/// Responsible for getting all open pull requests across all
/// imported git repositories
pub struct GetOpenPullRequestsQuery<A, G, S>
where
    A: AzureDevOpsRepository,
    G: GitRepositoryRepository,
    S: SecretRepository,
{
    azure_devops_repository: Arc<A>,
    git_repository_repository: G,
    secret_repository: S,
}

impl<A, G, S> GetOpenPullRequestsQuery<A, G, S>
where
    A: AzureDevOpsRepository + 'static,
    G: GitRepositoryRepository,
    S: SecretRepository,
{
    /// Create a new instance of the query
    ///
    /// # Arguments
    ///
    /// * `azure_devops_repository` - The repository to get pull requests from Azure DevOps
    /// * `git_repository_repository` - The repository to get imported git repositories
    /// * `secret_repository` - The repositories to get secrets
    pub fn new(
        azure_devops_repository: A,
        git_repository_repository: G,
        secret_repository: S,
    ) -> Self {
        Self {
            azure_devops_repository: Arc::new(azure_devops_repository),
            git_repository_repository,
            secret_repository,
        }
    }

    /// Execute the query
    ///
    /// # Errors
    ///
    /// Any errors that might occur
    pub async fn execute(&self) -> Result<Vec<PullRequestDto>> {
        // Get the relevant git repositories, marked as active
        let git_repos = self
            .git_repository_repository
            .get_all_git_repositories()
            .await?;
        let active_git_repos = git_repos
            .into_iter()
            .filter(|x| x.is_active)
            .collect::<Vec<GitRepository>>();
        // Get instances that can be shared across threads safely
        // Build the join set to retrieve the pull requests for all git repositories
        let mut join_set = JoinSet::<Result<Vec<PullRequest>>>::new();
        for gr in active_git_repos {
            let pat = self.secret_repository.get_secret(&gr.pat_secret_key)?;
            let azure_devops_repo = Arc::clone(&self.azure_devops_repository);
            join_set.spawn(async move {
                match gr.git_provider {
                    GitProvider::AzureDevOps => {
                        let res = azure_devops_repo
                            .get_open_pull_requests_in_repository(&pat, &gr.context, &gr.name)
                            .await?;
                        Ok(res)
                    }
                }
            });
        }
        // Collect the results from all tasks
        let mut result = vec![];
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(x) => {
                    let dto: Vec<PullRequestDto> = x?.iter().map(|x| x.into()).collect();
                    result.extend(dto);
                }
                Err(err) => {
                    log::info!(
                        "Error getting Azure DevOps git repository: {:?}",
                        err.to_string()
                    );
                }
            }
        }
        result.sort_by(|a, b| a.creation_date.cmp(&b.creation_date));
        Ok(result)
    }
}
