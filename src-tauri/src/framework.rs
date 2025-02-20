mod commands;
mod dependency_container;

use dependency_container::DependencyContainer;
use tauri::Manager;

/// The tauri application, including setup and the execution function
pub struct TauriApp {}

impl TauriApp {
    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .setup(|app| {
                let di_container = DependencyContainer::new();
                app.manage(di_container);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                commands::get_git_repositories,
                commands::import_azure_devops_organization_repositories,
                commands::remove_git_repository,
                commands::update_pat_for_git_repository,
                commands::get_open_pull_requests
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
