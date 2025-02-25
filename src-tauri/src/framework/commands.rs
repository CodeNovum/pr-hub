use super::dependency_container::DependencyContainer;
use crate::application::{
    dtos::{GitRepositoryDto, PullRequestDto},
    git_repositories::{
        get_git_repositories::GitRepositoriesQuery,
        import_azure_devops_organization_repositories::DevOpsOrgaImporter,
        remove_git_repository::RemoveGitRepositoryCommand,
        toggle_git_repository_active_state::ToggleGitRepositoryActiveStateCommand,
        update_pat_for_git_repository::UpdatePatForGitRepositoryCommand,
    },
    pull_requests::get_open_pull_requests::GetOpenPullRequestsQuery,
};
use tauri::State;

/// Tauri command to query for all imported git repositories
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
///
/// # Returns
///
/// * `Result<Vec<GitRepositoryDto>, String>` - The list of retrieved git repositories
///
/// # Errors
///
/// Any errors that might occur as string message
#[tauri::command]
pub async fn get_git_repositories(
    di_container: State<'_, DependencyContainer>,
) -> Result<Vec<GitRepositoryDto>, String> {
    let git_repository_repository = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    let query = GitRepositoriesQuery::new(git_repository_repository);
    let result = query.execute().await;
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
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
/// Any errors that might occur as string message
#[tauri::command]
pub async fn import_azure_devops_organization_repositories(
    di_container: State<'_, DependencyContainer>,
    organization_name: &str,
    pat: &str,
) -> Result<(), String> {
    let azure_devops_repository = di_container
        .azure_devops_repository_fac
        .produce(&di_container);
    let git_repository_repository = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    let secret_repository = di_container.secret_repository_fac.produce(&di_container);
    let importer = DevOpsOrgaImporter::new(
        azure_devops_repository,
        git_repository_repository,
        secret_repository,
    );
    let result = importer.import(organization_name, pat).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

/// Tauri command to remove a single imported git repository from the application
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
/// * `id` - The unique identifier of the repository to remove
///
/// # Errors
///
/// Any errors that might occur as string message
#[tauri::command]
pub async fn remove_git_repository(
    di_container: State<'_, DependencyContainer>,
    id: u32,
) -> Result<(), String> {
    let git_repository_repository = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    let secret_repository = di_container.secret_repository_fac.produce(&di_container);
    let command = RemoveGitRepositoryCommand::new(git_repository_repository, secret_repository);
    let result = command.execute(&id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

/// Tauri command to toggle the active state of a single imported git repository
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
/// * `id` - The unique identifier of the repository
///
/// # Errors
///
/// Any errors that might occur as string message
#[tauri::command]
pub async fn toggle_git_repository_active_state(
    di_container: State<'_, DependencyContainer>,
    id: u32,
) -> Result<(), String> {
    let git_repository_repository = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    let command = ToggleGitRepositoryActiveStateCommand::new(git_repository_repository);
    let result = command.execute(&id).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

/// Tauri command to update the PAT of an already imported git repository
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
/// * `id` - The unique identifier of the repository
/// * `pat` - The new value for the PAT
///
/// # Errors
///
/// Any errors that might occur as string message
#[tauri::command]
pub async fn update_pat_for_git_repository(
    di_container: State<'_, DependencyContainer>,
    id: u32,
    pat: &str,
) -> Result<(), String> {
    let git_repository_repository = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    let secret_repository = di_container.secret_repository_fac.produce(&di_container);
    let command =
        UpdatePatForGitRepositoryCommand::new(git_repository_repository, secret_repository);
    let result = command.execute(&id, pat).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(
                "Failed to update PAT for git repository with id {}: {}",
                id,
                err.to_string()
            );
            Err(err.to_string())
        }
    }
}

/// Tauri command to get all open pull requests across active imported
/// git repositories
///
/// # Arguments
///
/// * `di_container` - The container to resolve dependencies
///
/// # Errors
///
/// Any errors that might occur as string message
#[tauri::command]
pub async fn get_open_pull_requests(
    di_container: State<'_, DependencyContainer>,
) -> Result<Vec<PullRequestDto>, String> {
    let azure_devops_repository = di_container
        .azure_devops_repository_fac
        .produce(&di_container);
    let git_repository_repository = di_container
        .git_repository_repository_fac
        .produce(&di_container);
    let secret_repository = di_container.secret_repository_fac.produce(&di_container);
    let query = GetOpenPullRequestsQuery::new(
        azure_devops_repository,
        git_repository_repository,
        secret_repository,
    );
    let result = query.execute().await;
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}
