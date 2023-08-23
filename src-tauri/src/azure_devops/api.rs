use reqwest::{Client, Error, Response};
use thiserror::Error;

use super::response_types::{CurrentWorkitemsForMe, TeamProjects, Teams, WorkitemList};

#[derive(Error, Debug)]
pub enum AzureDevopsError {
    #[error("reqwest Error")]
    ReqwestError(reqwest::Error),
    #[error("authentication not valid")]
    Unauthorized,
    #[error("error statuscode was returned")]
    ErrorStatusCode(reqwest::Error),
    #[error("json parsing was not successful")]
    ResponseJsonParseError(reqwest::Error),
}

macro_rules! wiql_query {
    () => {
        concat!(
            "SELECT [System.Id],[System.WorkItemType],[System.Title],[System.AssignedTo],[System.State],[System.Tags]",
            "FROM WorkItems",
            "WHERE [System.TeamProject] = '{0}]' AND [System.AssignedTo] = @Me",
            r"AND [System.IterationPath] = @CurrentIteration('[{0}]\\{1}')")
    }
}

pub async fn get_my_workitems_for_current_iteration(
    client: Client,
    base_url: String,
    organization: String,
    project_name: String,
    team_name: String,
) -> Result<Vec<i64>, AzureDevopsError> {
    let body = format!(wiql_query!(), project_name, team_name);
    let url = format!("https://{base_url}/{organization}/_apis/wit/wiql?api-version=7.0");

    post(client, url, body, |x: CurrentWorkitemsForMe| {
        x.work_items.into_iter().map(|x| x.id).collect()
    })
    .await
}

pub async fn get_team_projects(
    client: Client,
    base_url: String,
    organization: String,
) -> Result<Vec<String>, AzureDevopsError> {
    let url = format!("https://{base_url}/{organization}/_apis/projects?api-version=7.0");

    get(client, url, |x: TeamProjects| {
        x.value.into_iter().map(|x| x.name).collect()
    })
    .await
}

pub async fn get_teams(
    client: Client,
    base_url: String,
    organization: String,
    project_name: String,
) -> Result<Vec<String>, AzureDevopsError> {
    let url = format!(
        "https://{base_url}/{organization}/_apis/projects/{project_name}/teams?api-version=7.0"
    );

    get(client, url, |x: Teams| {
        x.value.into_iter().map(|x| x.name).collect()
    })
    .await
}

pub async fn get_workitems_by_ids(
    client: Client,
    base_url: String,
    organization: String,
    project_name: String,
    workitem_ids: Vec<i64>,
) -> Result<Vec<String>, AzureDevopsError> {
    let ids = workitem_ids
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let url = format!("https://{base_url}/{organization}/{project_name}/_apis/wit/workitems?ids={ids}&api-version=7.0");

    get(client, url, |x: WorkitemList| {
        x.value.into_iter().map(|x| x.fields.system_title).collect()
    })
    .await
}

async fn get<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    client: Client,
    url: String,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, AzureDevopsError> {
    let result = client.get(url).send().await;

    parse_result(result, f).await
}

async fn post<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    client: Client,
    url: String,
    body: String,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, AzureDevopsError> {
    let result = client.post(url).body(body).send().await;

    parse_result(result, f).await
}

async fn parse_result<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    result: Result<Response, Error>,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, AzureDevopsError> {
    let response = match result {
        Ok(res) => res,
        Err(e) => return Err(AzureDevopsError::ReqwestError(e)),
    };

    match response.error_for_status() {
        Ok(res) => match res.json::<ResponseType>().await {
            Ok(parsed) => Ok(f(parsed)),
            Err(e) => Err(AzureDevopsError::ResponseJsonParseError(e)),
        },
        Err(e) => {
            if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) {
                Err(AzureDevopsError::Unauthorized)
            } else {
                Err(AzureDevopsError::ErrorStatusCode(e))
            }
        }
    }
}
