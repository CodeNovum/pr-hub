use crate::{
    constants::SERVICE_NAME,
    logic,
    model::{
        devops::{git_repository::GitRepository, pull_request::PullRequest},
        requests::devops_request::DevOpsRequest,
    },
    repositories,
};
use anyhow::{anyhow, Result};
use futures::future::join_all;
use keyring::Entry;

/// Retrieve the list of all git repositories from Azure DevOps the user has access to
/// because of the imported DevOps organizations.
///
/// # Returns
///
/// The list of all repositories.
pub async fn get_repositories() -> Result<Vec<GitRepository>> {
    let organizations = logic::organizations::get_organizations(true).await?;
    let mut tasks = Vec::new();
    for organization in organizations.iter() {
        if !organization.is_pat_valid {
            continue;
        }
        tasks.push(repositories::devops::get_repositories(
            &organization.pat,
            &organization.name,
        ))
    }
    let task_results = join_all(tasks).await;
    let mut result = Vec::new();
    for task_result in task_results {
        result.push(task_result?);
    }
    Ok(result.into_iter().flatten().collect())
}

/// Retrieve all open pull requests for the given request model in a batched way.
///
/// # Arguments
///
/// * `request_models` - The list of all request models to retrieve the pull requests for.
pub async fn get_open_pull_requests_batched(
    request_models: &Vec<DevOpsRequest>,
) -> Result<Vec<PullRequest>> {
    let request_models_with_pats = get_pats_for_request_models(request_models);
    let mut tasks = Vec::new();
    for request_model in request_models_with_pats.iter() {
        if !request_model.organization.is_pat_valid {
            continue;
        }
        let pat = &request_model.organization.pat;
        for repo_tuple in request_model.repositories.iter() {
            if let Some(repo_tuple) = repo_tuple {
                tasks.push(async {
                    let mut result = Vec::new();
                    let response = repositories::devops::get_open_pull_requests(
                        pat,
                        &request_model.organization.name,
                        &repo_tuple.0,
                        &repo_tuple.1,
                    )
                    .await;
                    match response {
                        Ok(pull_requests) => result.extend(pull_requests),
                        Err(error) => {
                            println!("Error: {}", error);
                            return result;
                        }
                    }
                    for pull_request in result.iter_mut() {
                        pull_request.organization_name =
                            Some(request_model.organization.name.clone());
                        if pull_request.repository.is_none()
                            || pull_request.pull_request_id.is_none()
                        {
                            // Skip invalid pull requests.
                            continue;
                        }
                        let repository_result = pull_request
                            .repository
                            .as_ref()
                            .ok_or(anyhow!("Invalid repository."));
                        let repository = match repository_result {
                            Ok(repo) => repo,
                            Err(error) => {
                                println!("Error: {}", error);
                                return result;
                            }
                        };
                        if repository.id.is_none() {
                            // Skip invalid repositories.
                            continue;
                        }
                        let repository_id_result = repository
                            .id
                            .as_ref()
                            .ok_or(anyhow!("Invalid repository ID."));
                        let repository_id = match repository_id_result {
                            Ok(id) => id,
                            Err(error) => {
                                println!("Error: {}", error);
                                return result;
                            }
                        };
                        let pull_request_id_result = pull_request
                            .pull_request_id
                            .ok_or(anyhow!("Invalid pull request ID."));
                        let pull_request_id = match pull_request_id_result {
                            Ok(id) => id,
                            Err(error) => {
                                println!("Error: {}", error);
                                return result;
                            }
                        };
                        let comment_threads =
                            repositories::devops::get_pull_request_comment_threads(
                                pat,
                                &request_model.organization.name,
                                &repo_tuple.0,
                                repository_id,
                                &pull_request_id,
                            )
                            .await;
                        match comment_threads {
                            Ok(pull_request_comment_threads) => {
                                pull_request.comment_threads = Some(pull_request_comment_threads);
                            }
                            Err(error) => println!("Error: {}", error),
                        }
                    }
                    result
                });
            }
        }
    }
    let task_results = join_all(tasks).await;
    let mut result = Vec::new();
    for task_result in task_results {
        result.push(task_result);
    }
    Ok(result.into_iter().flatten().collect())
}

/// For a given list of DevOps request models, retrieve and set the secure stored PAT for the organizations.
///
/// # Arguments
///
/// * `models` - The list of DevOps request models to retrieve the PAT for.
///
/// # Returns
///
/// The list of DevOps request models with the PAT set.
fn get_pats_for_request_models(models: &Vec<DevOpsRequest>) -> Vec<DevOpsRequest> {
    let mut result = Vec::new();
    for model in models {
        let stored_pat = Entry::new(
            SERVICE_NAME,
            &format!("organization-{}", model.organization.id),
        );
        let pat_to_use;
        match stored_pat {
            Ok(pat) => {
                let pat = pat.get_password();
                match pat {
                    Ok(p) => {
                        pat_to_use = p.to_string();
                    }
                    Err(_) => pat_to_use = "".to_string(),
                }
            }
            Err(_) => pat_to_use = "".to_string(),
        }
        let mut new_model = model.clone();
        new_model.organization.pat = pat_to_use;
        result.push(new_model);
    }
    result
}
