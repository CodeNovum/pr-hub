pub mod commands;
pub mod constants;
pub mod data;
pub mod http;
pub mod logic;
pub mod model;
pub mod repositories;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_default()
                .into_os_string()
                .into_string();
            match app_data_dir {
                Ok(app_data_dir) => {
                    data::application_database::init_db(&app_data_dir);
                }
                Err(error) => {
                    println!("Error: {}", error.into_string().unwrap());
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_organizations,
            commands::get_projects,
            commands::get_open_pull_requests_batched,
            commands::add_organization,
            commands::remove_organization,
            commands::update_pat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
