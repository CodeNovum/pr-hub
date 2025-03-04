use super::{
    enums::{CommentType, PullRequestCommentThreadStatus},
    models::{
        GitRepository as AzureDevopsGitRepository, PullRequest as AzureDevopsPullRequest,
        PullRequestCommentThread, Response, TeamProjectReference,
    },
};
use anyhow::Result;
use application::traits::AzureDevOpsRepository;
use async_trait::async_trait;
use base64::Engine;
use domain::{
    enums::GitProvider,
    models::{GitRepository, PullRequest},
};
use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use std::sync::Arc;
use tokio::task::JoinSet;

const DEVOPS_API_VERSION: &str = "6.0";
const DEVOPS_API_BASE_URL: &str = "https://dev.azure.com";

/// Repository to access Azure DevOps using it's REST API
pub struct AzureDevOpsRestRepository {
    http_client: Client,
}

impl Default for AzureDevOpsRestRepository {
    /// Create a default instance with a configured http client that can
    /// access the REST API using a PAT
    fn default() -> Self {
        let mut default_http_client_headers = HeaderMap::new();
        let api_version_header =
            HeaderValue::from_str(DEVOPS_API_VERSION).expect("Could not create api version header");
        default_http_client_headers.insert("api-version", api_version_header);
        Self {
            http_client: Client::builder()
                .https_only(true)
                .default_headers(default_http_client_headers)
                .build()
                .expect("Could not create HTTP client for the Azure DevOps REST API"),
        }
    }
}

#[async_trait]
impl AzureDevOpsRepository for AzureDevOpsRestRepository {
    async fn get_open_pull_requests_in_repository(
        &self,
        pat: &str,
        context: &str,
        repository_name: &str,
    ) -> Result<Vec<PullRequest>> {
        // Get all pull requests
        let relative_url = format!(
            "{}/_apis/git/repositories/{}/pullrequests",
            context, repository_name
        );
        let response = perform_get_request::<Response<AzureDevopsPullRequest>>(
            &self.http_client,
            pat,
            &relative_url,
        )
        .await?;
        // Create a copy of the http client that is safe to be shared between threads
        let http_client = Arc::new(self.http_client.clone());
        // Get comment threads for each pull request concurrently to determine how many
        // comments were made for the PR and how many of those are marked as done
        let mut join_set = JoinSet::<Result<PullRequest>>::new();
        for x in response.value {
            let pat = pat.to_string();
            let http_client_arc = Arc::clone(&http_client);
            let context = context.to_string();
            let repository = repository_name.to_string();
            join_set.spawn(async move {
                let relative_url = format!(
                    "{}/_apis/git/repositories/{}/pullRequests/{}/threads",
                    context, repository, x.pull_request_id
                );
                let response = perform_get_request::<Response<PullRequestCommentThread>>(
                    &http_client_arc,
                    &pat,
                    &relative_url,
                )
                .await?;
                let comments = response.value.iter().filter(|x| {
                    x.comments
                        .iter()
                        .any(|y| y.comment_type == CommentType::Text)
                });
                let solved_comments = comments.clone().filter(|x| {
                    x.status == PullRequestCommentThreadStatus::Closed
                        || x.status == PullRequestCommentThreadStatus::Fixed
                        || x.status == PullRequestCommentThreadStatus::WontFix
                        || x.status == PullRequestCommentThreadStatus::ByDesign
                });
                let pr = PullRequest {
                    id: x.pull_request_id,
                    repository_name: repository.to_string(),
                    title: x.title.to_string(),
                    merge_status: x.merge_status.to_string(),
                    creator_name: x.created_by.display_name.to_string(),
                    creation_date: x.creation_date,
                    number_of_comments: comments.count(),
                    number_of_closed_comments: solved_comments.count(),
                    link: format!(
                        "https://dev.azure.com/{}/_git/{}/pullrequest/{}",
                        context, repository, x.pull_request_id
                    ),
                };
                Ok(pr)
            });
        }
        // Collect the results from all tasks and return all found domain models
        let mut result = vec![];
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(x) => result.push(x?),
                Err(err) => {
                    log::info!(
                        "Error getting Azure DevOps git repository: {:?}",
                        err.to_string()
                    );
                }
            }
        }
        Ok(result)
    }

    async fn get_repositories_in_organization(
        &self,
        pat: &str,
        organization_name: &str,
    ) -> Result<Vec<GitRepository>> {
        // Create a copy of the http client that is safe to be shared between threads
        let http_client = Arc::new(self.http_client.clone());
        // Get all the projects first, cause the relation in Azure DevOps between
        // project and git repository is 1:n
        let projects_path = format!("{}/_apis/projects", organization_name);
        let projects = perform_get_request::<Response<TeamProjectReference>>(
            &Arc::clone(&http_client),
            pat,
            &projects_path,
        )
        .await?
        .value;
        // Get all repositories across all found projects and map them to the domain model
        let mut join_set = JoinSet::<Result<Vec<GitRepository>>>::new();
        for project in projects {
            let organization_name = organization_name.to_string();
            let project_name = project.name.to_string();
            let pat = pat.to_string();
            let http_client_arc = Arc::clone(&http_client);
            join_set.spawn(async move {
                let git_repos_path = format!(
                    "{}/{}/_apis/git/repositories",
                    organization_name, project_name
                );
                let response = perform_get_request::<Response<AzureDevopsGitRepository>>(
                    &http_client_arc,
                    &pat,
                    &git_repos_path,
                )
                .await?;
                let result = response
                    .value
                    .iter()
                    .map(|x| GitRepository {
                        id: 0,
                        name: x.name.to_string(),
                        context: format!("{}/{}", organization_name, project_name),
                        git_provider: GitProvider::AzureDevOps,
                        is_active: true,
                        pat_secret_key: format!("azuredevops-{}", organization_name),
                    })
                    .collect::<Vec<GitRepository>>();
                Ok(result)
            });
        }
        // Collect the results from all tasks and return all found domain models
        let mut result = vec![];
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(x) => result.extend(x?),
                Err(err) => {
                    log::info!(
                        "Error getting Azure DevOps git repository: {:?}",
                        err.to_string()
                    );
                }
            }
        }
        Ok(result)
    }
}

/// Helper to perform a GET HTTP request to the Azure DevOps API
///
/// # Arguments
///
/// * `pat` - The personal access token to authorize the request
/// * `path` - The relative path to the API resource
///
/// # Returns
///
/// * `Result<T>` - The result of the request parsed as T
///
/// # Errors
///
/// Any error that might occur
async fn perform_get_request<T>(http_client: &Client, pat: &str, path: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let url = format!("{}/{}", DEVOPS_API_BASE_URL, path);
    let encoded_pat = base64::engine::general_purpose::STANDARD.encode(format!(":{}", pat));
    let auth_value = format!("Basic {}", &encoded_pat);
    let auth_header_value = HeaderValue::from_str(&auth_value)?;
    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header_value);
        headers
    };
    let response = http_client.get(url).headers(headers).send().await?;
    let json = response.json::<T>().await?;
    Ok(json)
}
