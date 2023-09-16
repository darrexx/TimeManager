use crate::reqwest_helper::{
    error::ApiError,
    helper::{get, patch, post},
};

use super::{
    client::AzureDevopsClient,
    models::Workitem,
    response_types::{CurrentWorkitemsForMe, TeamProjects, Teams, WorkitemList, WorkitemListValue},
};

macro_rules! wiql_query {
    () => {
        concat!(
            r#"{{ "query": ""#,
            "SELECT [System.Id],[System.WorkItemType],[System.Title],[System.AssignedTo],[System.State],[System.Tags] ",
            "FROM WorkItems ",
            "WHERE [System.TeamProject] = '{0}' AND [System.AssignedTo] = @Me ",
            r"AND [System.IterationPath] = @CurrentIteration('[{0}]\\{1}')",
            r#"" }}"#)
    }
}

pub async fn get_my_workitems_for_current_iteration(
    client: &AzureDevopsClient,
    base_url: &String,
    organization: &String,
    project_name: &String,
    team_name: &String,
) -> Result<Vec<i64>, ApiError> {
    let body = format!(wiql_query!(), project_name, team_name);
    let url = format!("https://{base_url}/{organization}/_apis/wit/wiql?api-version=7.0");

    post(&client.0, url, body, |x: CurrentWorkitemsForMe| {
        x.work_items.into_iter().map(|x| x.id).collect()
    })
    .await
}

#[allow(dead_code)]
pub async fn get_team_projects(
    client: &AzureDevopsClient,
    base_url: &String,
    organization: &String,
) -> Result<Vec<String>, ApiError> {
    let url = format!("https://{base_url}/{organization}/_apis/projects?api-version=7.0");

    get(&client.0, url, |x: TeamProjects| {
        x.value.into_iter().map(|x| x.name).collect()
    })
    .await
}

#[allow(dead_code)]
pub async fn get_teams(
    client: &AzureDevopsClient,
    base_url: &String,
    organization: &String,
    project_name: &String,
) -> Result<Vec<String>, ApiError> {
    let url = format!(
        "https://{base_url}/{organization}/_apis/projects/{project_name}/teams?api-version=7.0"
    );

    get(&client.0, url, |x: Teams| {
        x.value.into_iter().map(|x| x.name).collect()
    })
    .await
}

pub async fn get_workitems_by_ids(
    client: &AzureDevopsClient,
    base_url: &String,
    organization: &String,
    project_name: &String,
    workitem_ids: Vec<i64>,
) -> Result<Vec<Workitem>, ApiError> {
    let ids = workitem_ids
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let url = format!("https://{base_url}/{organization}/{project_name}/_apis/wit/workitems?ids={ids}&api-version=7.0");

    get(&client.0, url, |x: WorkitemList| {
        x.value
            .into_iter()
            .map(|x| Workitem {
                id: x.id,
                name: x.fields.system_title,
            })
            .collect()
    })
    .await
}

pub async fn update_workitem_to_in_progress(
    client: &AzureDevopsClient,
    base_url: &String,
    organization: &String,
    project_name: &String,
    workitem_id: i64,
) -> Result<(), ApiError> {
    let body = r#"[
        {
          "op": "add",
          "path": "/fields/System.State",
          "value": "In Progress"
        }
      ]
      "#;
    let url = format!("https://{base_url}/{organization}/{project_name}/_apis/wit/workitems/{workitem_id}?api-version=7.0");

    patch(
        &client.0,
        url,
        body.into(),
        String::from("application/json-patch+json"),
        |_: WorkitemListValue| (),
    )
    .await
}
