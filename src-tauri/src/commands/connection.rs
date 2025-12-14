use tauri::{command, State};
use tracing::error;

use crate::utilities::app_state::AppState;

#[command]
pub async fn create_connection(
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn update_connection(
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    Ok(())
}

#[command]
pub async fn delete_connection(app_state: State<'_, AppState>) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    Ok(())
}

#[command]
pub async fn get_connection(
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    Ok(())
}

#[command]
pub async fn get_all_connections(
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    Ok(())
}
