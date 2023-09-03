use tauri::{AppHandle, Manager, State};

use crate::{
    azure_devops::client::{configure_devops_httpclient, AzureDevopsClient},
    state::models::ConfigState,
};

use super::models::Config;

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn save_devops_config(
    app_handle: AppHandle,
    config: State<'_, ConfigState>,
    url: String,
    user: String,
    pat: String,
    organization: String,
    project: String,
    team: String,
) -> Result<(), ()> {
    let devops_client = configure_devops_httpclient(&user, &pat);
    app_handle.manage(AzureDevopsClient(devops_client));

    let mut config = config.lock().await;
    config.devops_config.base_url = url;
    config.devops_config.user = user;
    config.devops_config.pat = pat;
    config.devops_config.organization = organization;
    config.devops_config.project = project;
    config.devops_config.team = team;

    confy::store("timemanager", None, Config::from(&config)).unwrap();

    Ok(())
}
