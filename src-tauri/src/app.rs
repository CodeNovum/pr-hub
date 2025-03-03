use crate::{commands, dependency_container::DependencyContainer};
use std::{env, str::FromStr};
use tauri::Manager;

/// The tauri application, including setup and the execution function
pub struct TauriApp {}

impl TauriApp {
    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        let log_env = env::var("RUST_LOG").unwrap_or("info".to_string());
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .plugin(
                tauri_plugin_log::Builder::new()
                    .level(log::LevelFilter::from_str(&log_env).unwrap_or(log::LevelFilter::Info))
                    .build(),
            )
            .setup(|app| {
                let data_dir = app
                    .path()
                    .app_data_dir()
                    .expect("App data dir must be resolved")
                    .into_os_string()
                    .into_string()
                    .expect("App data dir needs to be represented as string");
                let di_container = DependencyContainer::new(&data_dir);
                app.manage(di_container);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                commands::get_git_repositories,
                commands::import_azure_devops_organization_repositories,
                commands::toggle_git_repository_active_state,
                commands::remove_git_repository,
                commands::update_pat_for_git_repository,
                commands::get_open_pull_requests
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
