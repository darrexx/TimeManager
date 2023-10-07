use crate::reqwest_helper::{
    error::ApiError,
    helper::{get, get_with_query_params, post},
};

use super::{
    client::KimaiClient,
    models::{
        activity::KimaiActivity,
        customer::GetCustomersResponse,
        project::Project,
        team::{GetTeamResponse, GetTeamsResponse},
        timesheet::{PostTimeSheetBody, TimeSheet},
    },
};

pub async fn get_customers(
    client: &KimaiClient,
    base_url: &String,
) -> Result<Vec<GetCustomersResponse>, ApiError> {
    let url = format!("https://{base_url}/customers");

    get(&client.0, url, std::convert::identity).await
}

pub async fn get_projects(
    client: &KimaiClient,
    base_url: &String,
    customer_id: &i32,
) -> Result<Vec<Project>, ApiError> {
    let url = format!("https://{base_url}/projects");

    let mut query_params = Vec::new();
    query_params.push(("customer".into(), customer_id.to_string()));

    get_with_query_params(&client.0, url, std::convert::identity, &query_params).await
}

pub async fn get_activities(
    client: &KimaiClient,
    base_url: &String,
    project_id: &i32,
) -> Result<Vec<KimaiActivity>, ApiError> {
    let url = format!("https://{base_url}/activities");

    let mut query_params = Vec::new();
    query_params.push(("project".into(), project_id.to_string()));
    get_with_query_params(&client.0, url, std::convert::identity, &query_params).await
}

#[allow(dead_code)]
pub async fn get_teams(
    client: &KimaiClient,
    base_url: &String,
) -> Result<Vec<GetTeamsResponse>, ApiError> {
    let url = format!("https://{base_url}/teams");

    get(&client.0, url, std::convert::identity).await
}

#[allow(dead_code)]
pub async fn get_team(
    client: &KimaiClient,
    base_url: &String,
    team_id: &i32,
) -> Result<GetTeamResponse, ApiError> {
    let url = format!("https://{base_url}/teams/{team_id}");

    get(&client.0, url, std::convert::identity).await
}

pub async fn post_timesheet(
    client: &KimaiClient,
    base_url: &String,
    body: PostTimeSheetBody,
) -> Result<TimeSheet, ApiError> {
    let url = format!("https://{base_url}/timesheets");

    post(
        &client.0,
        url,
        serde_json::ser::to_string(&body).unwrap(),
        std::convert::identity,
    )
    .await
}
