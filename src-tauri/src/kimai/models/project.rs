use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub customer: Option<i32>,
    pub id: Option<i32>,
    pub name: String,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
    #[serde(rename = "globalActivities")]
    pub global_activities: bool,
    pub color: Option<String>,
}
