use tauri::State;

use crate::{reqwest_helper::error::ApiError, state::models::ConfigState};

use super::{
    client::KimaiClient,
    models::{activity::KimaiActivity, customer::GetCustomersResponse, project::Project},
};

#[tauri::command]
pub async fn get_projects(
    client: State<'_, KimaiClient>,
    config: State<'_, ConfigState>,
    customer_id: i32,
) -> Result<Vec<Project>, ApiError> {
    let config = config.lock().await;

    super::api::get_projects(client.get(), &config.kimai_config.base_url, &customer_id).await
}

#[tauri::command]
pub async fn get_customers(
    client: State<'_, KimaiClient>,
    config: State<'_, ConfigState>,
) -> Result<Vec<GetCustomersResponse>, ApiError> {
    let config = config.lock().await;

    super::api::get_customers(client.get(), &config.kimai_config.base_url).await
}

#[tauri::command]
pub async fn get_kimai_activities(
    client: State<'_, KimaiClient>,
    config: State<'_, ConfigState>,
    project_id: i32,
) -> Result<Vec<KimaiActivity>, ApiError> {
    let config = config.lock().await;

    super::api::get_activities(client.get(), &config.kimai_config.base_url, &project_id).await
}
