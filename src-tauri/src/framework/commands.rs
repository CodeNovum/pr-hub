use crate::application::git_repositories::import_azure_devops_organization_repositories::DevOpsOrgaImporter;

use super::dependency_container::DependencyContainer;
use tauri::State;

#[tauri::command]
pub async fn get_git_repositories(
    di_container: State<'_, DependencyContainer>,
) -> Result<(), String> {
    let _service = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    Ok(())
}

/// Tauri command to import all git repositories from a single Azure DevOps
/// organization into the application
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
/// * `organization_name` - The name of the Azure DevOps organization
/// * `pat` - The private access token to access all git repositories
///
/// # Errors
///
/// Any errors that might occur
#[tauri::command]
pub async fn import_azure_devops_organization_repositories(
    di_container: State<'_, DependencyContainer>,
    organization_name: &str,
    pat: &str,
) -> Result<(), String> {
    // Resolve the commands dependencies
    let azure_devops_repository = di_container
        .azure_devops_repository_fac
        .produce(&di_container);
    // Construct and run the import command
    let importer = DevOpsOrgaImporter::new(azure_devops_repository);
    importer.import(organization_name, pat).await;
    Ok(())
}

/// Removes a single imported git repository from the application
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
/// * `id` - The unique identifier of the repository to remove
///
/// # Errors
///
/// Any errors that might occur
#[tauri::command]
pub fn remove_git_repository(
    di_container: State<'_, DependencyContainer>,
    id: u64,
) -> Result<(), String> {
    let _service = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    Ok(())
}

#[tauri::command]
pub fn update_pat_for_git_repository(
    di_container: State<'_, DependencyContainer>,
) -> Result<(), String> {
    let _service = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    Ok(())
}

#[tauri::command]
pub async fn get_open_pull_requests(
    di_container: State<'_, DependencyContainer>,
) -> Result<(), String> {
    let _service = di_container
        .azure_devops_repository_fac
        .produce(&di_container);
    Ok(())
}
