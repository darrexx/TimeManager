use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSheet {
    pub activity: Option<i32>,
    pub project: Option<i32>,
    pub user: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub id: Option<i32>,
    pub begin: String,
    pub end: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub rate: Option<f32>,
    #[serde(rename = "internalRate")]
    pub internal_rate: Option<f32>,
    #[serde(rename = "fixedRate")]
    pub fixed_rate: Option<f32>,
    #[serde(rename = "hourlyRate")]
    pub hourly_rate: Option<f32>,
    pub exported: bool,
    pub billable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostTimeSheetBody {
    pub begin: String,
    pub end: String,
    pub project: i32,
    pub activity: i32,
    pub description: String,
    pub billable: bool,
}
