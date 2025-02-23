use super::models::{
    GitRepository as AzureDevopsGitRepository, PullRequest as AzureDevopsPullRequest, Response,
    TeamProjectReference,
};
use crate::{
    application::traits::AzureDevOpsRepository,
    domain::models::{GitRepository, PullRequest},
};
use anyhow::Result;
use async_trait::async_trait;
use base64::Engine;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
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
        let result = response
            .value
            .iter()
            .map(|x| PullRequest {
                repository_name: repository_name.to_string(),
                title: x.title.to_string(),
                merge_status: x.merge_status.to_string(),
                creator_name: x.created_by.display_name.to_string(),
                creation_date: x.creation_date,
                number_of_comments: 0,
                number_of_closed_comments: 0,
            })
            .collect::<Vec<PullRequest>>();
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
                        name: x.name.to_string(),
                        context: format!("{}/{}", organization_name, project_name),
                        is_active: true,
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
