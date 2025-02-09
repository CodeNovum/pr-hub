use crate::{
    http::http_client::get_http_client,
    model::devops::{
        connection_data::ConnectionData, git_repository::GitRepository, project::Project,
        pull_request::PullRequest, pull_request_comment_thread::PullRequestCommentThread,
        response::Response,
    },
};
use anyhow::Result;
use base64;
use base64::Engine;
use futures::future::join_all;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde;

const DEVOPS_API_VERSION: &str = "6.0";
const DEVOPS_API_BASE_URL: &str = "https://dev.azure.com";

/// Performs a DevOps API get request.
///
/// # Arguments
///
/// * `pat` - The DevOps personal access token.
/// * `base_url` - The base url for the API to fetch.
/// * `relative_url` - The relative url to fetch.
///
/// # Returns
///
/// * `Result<T>` - The result of the request as the generic type T.
async fn perform_get_request<T>(
    pat: &str,
    base_url: &str,
    relative_url: &str,
    api_version: Option<&str>,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let devops_api_version = api_version.unwrap_or(DEVOPS_API_VERSION);
    let client = get_http_client();
    let url = format!(
        "{}/{}?api-version={}",
        base_url, relative_url, devops_api_version
    );
    let encoded_pat = base64::engine::general_purpose::STANDARD.encode(format!(":{}", pat));
    let auth_value = format!("Basic {}", &encoded_pat);
    let auth_header_value = HeaderValue::from_str(&auth_value)?;
    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header_value);
        headers
    };
    let response = client.get(url).headers(headers).send().await?;
    let json = response.json::<T>().await?;
    Ok(json)
}

/// Validates a given DevOps personal access token for accessing a given DevOps organization.
///
/// # Arguments
///
/// * `pat` - The DevOps personal access token.
/// * `organization_name` - The DevOps organization name.
///
/// # Returns
///
/// * `bool` - True if the given pat is valid for the given organization, false otherwise.
pub async fn validate_pat(pat: &str, organization_name: &str) -> bool {
    let relative_url = format!("{}/_apis/connectionData", organization_name);
    let result =
        perform_get_request::<ConnectionData>(pat, DEVOPS_API_BASE_URL, &relative_url, Some("1"))
            .await;
    result.is_ok()
}

/// Retrieves all projects from the DevOps organization.
///
/// # Arguments
///
/// * `pat` - The DevOps personal access token.
/// * `organization_name` - The DevOps organization name.
///
/// # Returns
///
/// * `Result<Vec<GitRepository>>` - The retrieved list of repositories.
pub async fn get_repositories(pat: &str, organization_name: &str) -> Result<Vec<GitRepository>> {
    let relative_url = format!("{}/_apis/projects", organization_name);
    let projects =
        perform_get_request::<Response<Project>>(pat, DEVOPS_API_BASE_URL, &relative_url, None)
            .await
            .map(|response| {
                response
                    .value
                    .unwrap_or_default()
                    .into_iter()
                    .map(|mut project| {
                        project.organization_name = Some(organization_name.to_string());
                        project
                    })
                    .collect::<Vec<Project>>()
            })?;
    let mut tasks = Vec::new();
    for project in projects {
        tasks.push(async move {
            let mut result = vec![];
            let relative_url = format!(
                "{}/{}/_apis/git/repositories",
                organization_name,
                project.name.clone()
            );
            let response = perform_get_request::<Response<GitRepository>>(
                pat,
                DEVOPS_API_BASE_URL,
                &relative_url,
                None,
            )
            .await;
            match response {
                Ok(r) => result.extend(
                    r.value
                        .into_iter()
                        .flatten()
                        .collect::<Vec<GitRepository>>(),
                ),
                Err(error) => {
                    println!("Error: {}", error);
                    return result;
                }
            }
            for repo in result.iter_mut() {
                if let Some(project) = repo.project.as_mut() {
                    project.organization_name = Some(organization_name.to_string());
                }
            }
            result
        });
    }
    let task_results = join_all(tasks).await;
    Ok(task_results.into_iter().flatten().collect())
}

/// Retrieves all open pull requests for a DevOps project.
///
/// # Arguments
///
/// * `pat` - The DevOps personal access token.
/// * `organization_name` - The DevOps organization name.
/// * `project_name` - The DevOps project name.
/// * `repository_name` - The DevOps repository name.
///
/// # Returns
///
/// * `Result<Vec<PullRequest>>` - The retrieved list of open pull requests.
pub async fn get_open_pull_requests(
    pat: &str,
    organization_name: &str,
    project_name: &str,
    repository_name: &str,
) -> Result<Vec<PullRequest>> {
    let relative_url = format!(
        "{}/{}/_apis/git/repositories/{}/pullrequests",
        organization_name, project_name, repository_name
    );
    let result =
        perform_get_request::<Response<PullRequest>>(pat, DEVOPS_API_BASE_URL, &relative_url, None)
            .await
            .map(|response| response.value.unwrap_or_default())?;
    Ok(result)
}

/// Retrieves all pull request comment threads for a single DevOps pull request.
///
/// # Arguments
///
/// * `pat` - The DevOps personal access token.
/// * `organization_name` - The DevOps organization name.
/// * `project_name` - The DevOps project name.
/// * `repository_id` - The DevOps repository id.
/// * `pull_request_id` - The DevOps pull request id.
///
/// # Returns
///
/// * `Result<Vec<PullRequestCommentThread>>` - The retrieved list of pull request comment threads.
pub async fn get_pull_request_comment_threads(
    pat: &str,
    organization_name: &str,
    project_name: &str,
    repository_id: &str,
    pull_request_id: &i32,
) -> Result<Vec<PullRequestCommentThread>> {
    let relative_url = format!(
        "{}/{}/_apis/git/repositories/{}/pullRequests/{}/threads",
        organization_name, project_name, repository_id, pull_request_id
    );
    let result = perform_get_request::<Response<PullRequestCommentThread>>(
        pat,
        DEVOPS_API_BASE_URL,
        &relative_url,
        None,
    )
    .await
    .map(|response| response.value.unwrap_or_default())?;
    Ok(result)
}
