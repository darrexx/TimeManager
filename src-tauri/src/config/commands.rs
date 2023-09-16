use tauri::{AppHandle, Manager, State};

use crate::{
    azure_devops::client::{configure_devops_httpclient, AzureDevopsClient},
    state::models::ConfigState,
};

use super::models::{AzureDevopsConfig, Config};

#[tauri::command]
pub async fn save_devops_config(
    app_handle: AppHandle,
    config: State<'_, ConfigState>,
    devops_config: AzureDevopsConfig,
) -> Result<(), ()> {
    let devops_client = configure_devops_httpclient(&devops_config.user, &devops_config.pat);
    app_handle.manage(AzureDevopsClient(devops_client));

    let mut config = config.lock().await;
    config.devops_config = devops_config;

    confy::store("timemanager", None, Config::from(&config)).unwrap();

    Ok(())
}
