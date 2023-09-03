use super::models::{ConfigState, Frontend, FrontendState};
use crate::config::models::{Config, FrontendConfig};
use tauri::State;

#[tauri::command]
pub async fn get_config(config: State<'_, ConfigState>) -> Result<Config, String> {
    let config = config.lock().await;

    Ok(Config::from(&config))
}

#[tauri::command]
pub async fn set_config(config: FrontendConfig) -> Result<(), ()> {
    confy::store("timemanager", None, Config::from(config)).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn get_frontend_state(frontend_state: State<'_, FrontendState>) -> Result<Frontend, ()> {
    let state = frontend_state.lock().await;

    Ok(Frontend::from(&state))
}
