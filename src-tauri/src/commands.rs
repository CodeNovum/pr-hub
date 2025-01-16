use crate::{
    logic,
    model::{
        core::organization::Organization,
        devops::{project::Project, pull_request::PullRequest},
        requests::devops_request::DevOpsRequest,
    },
};

/// Command to retrieve the list of all organizations from the database.
///
/// # Returns
///
/// The list of all organizations.
///
#[tauri::command]
pub async fn get_organizations() -> Vec<Organization> {
    let organizations = logic::organizations::get_organizations(false).await;
    organizations.unwrap_or_default()
}

/// Command to add an organization.
///
/// # Arguments
///
/// * `orga_name` - The name of the organization to add
///
#[tauri::command]
pub async fn add_organization(orga_name: String, pat_value: String) -> Result<i64, String> {
    let result = logic::organizations::add_organization(&orga_name, &pat_value);
    match result {
        Ok(id) => Ok(id),
        Err(error) => Err(error.to_string()),
    }
}

/// Command to remove an organization.
///
/// # Arguments
///
/// * `id` - The id of the organization to remove.
///
#[tauri::command]
pub fn remove_organization(id: i64) -> Result<(), String> {
    let result = logic::organizations::remove_organization(&id);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

/// Command to update the PAT of an organization.
///
/// # Arguments
///
/// * `id` - The id of the organization to update the PAT for.
/// * `pat_value` - The new PAT value.
///
#[tauri::command]
pub fn update_pat(id: i64, pat_value: &str) -> Result<(), String> {
    let result = logic::organizations::update_pat(&id, pat_value);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

/// Command to retrieve the list of all projects from Azure DevOps the user has
/// access to because of the imported DevOps organizations.
///
/// # Returns
///
/// The list of all projects.
///
#[tauri::command]
pub async fn get_projects() -> Result<Vec<Project>, String> {
    let result = logic::devops::get_projects().await;
    match result {
        Ok(projects) => Ok(projects),
        Err(error) => Err(error.to_string()),
    }
}

/// Command to retrieve the list of all open pull requests.
///
/// # Arguments
///
/// * `request_models` - The list of DevOps request models to retrieve the open pull requests for.
///
/// # Returns
///
/// The list of all open pull requests.
///
#[tauri::command]
pub async fn get_open_pull_requests_batched(
    request_models: Vec<DevOpsRequest>,
) -> Result<Vec<PullRequest>, String> {
    let result = logic::devops::get_open_pull_requests_batched(&request_models).await;
    match result {
        Ok(pull_requests) => Ok(pull_requests),
        Err(error) => Err(error.to_string()),
    }
}
