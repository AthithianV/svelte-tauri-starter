pub mod commands;
pub mod entity;
pub mod model;
pub mod repository;
pub mod service;
pub mod utilities;

use tauri::{Manager, State};

pub use commands::connection::*;
pub use commands::queue::*;
pub use utilities::{app_state::AppState, local_db::initialize_local_db, logging::init_logging};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .manage(app_state.clone())
        .setup(|app| {
            init_logging();

            let handle = app.handle().clone();
            // spawn async task to initialize AppState
            tauri::async_runtime::spawn(async move {
                match initialize_local_db().await {
                    Ok(db_conn) => {
                        let state: State<'_, AppState> = handle.state();
                        state.set_app_db_connection(db_conn).await
                    }
                    Err(e) => println!("Failed to initialize DB: {}", e),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_connection,
            update_connection,
            delete_connection,
            get_connection,
            get_all_connections,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
