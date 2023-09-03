use crate::state::models::ConfigState;

use super::{
    api::{get_my_workitems_for_current_iteration, get_workitems_by_ids},
    client::AzureDevopsClient,
    error::AzureDevopsError,
    models::Workitem,
};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn get_workitems(
    app_handle: AppHandle,
    config: State<'_, ConfigState>,
) -> Result<Vec<Workitem>, AzureDevopsError> {
    if let Some(client) = app_handle.try_state::<AzureDevopsClient>() {
        let config = config.lock().await;

        let workitem_ids = get_my_workitems_for_current_iteration(
            client.get(),
            &config.devops_config.base_url,
            &config.devops_config.organization,
            &config.devops_config.project,
            &config.devops_config.team,
        )
        .await?;

        let workitems = get_workitems_by_ids(
            client.get(),
            &config.devops_config.base_url,
            &config.devops_config.organization,
            &config.devops_config.project,
            workitem_ids,
        )
        .await?;

        Ok(workitems)
    } else {
        Err(AzureDevopsError::Unauthorized)
    }
}
