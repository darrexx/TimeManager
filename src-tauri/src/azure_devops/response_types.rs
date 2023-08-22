use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWorkitemsForMe {
    pub query_type: String,
    pub query_result_type: String,
    pub as_of: String, //UTC ISO 8601 Date
    pub columns: Vec<Column>,
    pub work_items: Vec<WorkItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub reference_name: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkItem {
    pub id: i64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamProjects {
    pub count: i64,
    pub value: Vec<TeamProjectsValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamProjectsValue {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub state: String,
    pub revision: i64,
    pub visibility: String,
    pub last_update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkitemList {
    pub count: i64,
    pub value: Vec<WorkitemListValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkitemListValue {
    pub id: i64,
    pub rev: i64,
    pub fields: Fields,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    #[serde(rename = "System.AreaPath")]
    pub system_area_path: String,
    #[serde(rename = "System.TeamProject")]
    pub system_team_project: String,
    #[serde(rename = "System.IterationPath")]
    pub system_iteration_path: String,
    #[serde(rename = "System.WorkItemType")]
    pub system_work_item_type: String,
    #[serde(rename = "System.State")]
    pub system_state: String,
    #[serde(rename = "System.Reason")]
    pub system_reason: String,
    #[serde(rename = "System.AssignedTo")]
    pub system_assigned_to: System,
    #[serde(rename = "System.CreatedDate")]
    pub system_created_date: String,
    #[serde(rename = "System.CreatedBy")]
    pub system_created_by: System,
    #[serde(rename = "System.ChangedDate")]
    pub system_changed_date: String,
    #[serde(rename = "System.ChangedBy")]
    pub system_changed_by: System,
    #[serde(rename = "System.CommentCount")]
    pub system_comment_count: i64,
    #[serde(rename = "System.Title")]
    pub system_title: String,
    #[serde(rename = "Microsoft.VSTS.Common.ClosedDate")]
    pub microsoft_vsts_common_closed_date: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Common.StateChangeDate")]
    pub microsoft_vsts_common_state_change_date: String,
    #[serde(rename = "Microsoft.VSTS.Common.Priority")]
    pub microsoft_vsts_common_priority: i64,
    #[serde(rename = "System.Description")]
    pub system_description: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Scheduling.RemainingWork")]
    pub microsoft_vsts_scheduling_remaining_work: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub display_name: String,
    pub url: String,
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: String,
    pub unique_name: String,
    pub image_url: String,
    pub descriptor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub avatar: Avatar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Teams {
    pub value: Vec<TeamsValue>,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamsValue {
    pub id: String,
    pub name: String,
    pub url: String,
    pub description: String,
    pub identity_url: String,
    pub project_name: String,
    pub project_id: String,
}
