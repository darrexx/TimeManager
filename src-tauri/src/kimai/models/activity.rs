use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KimaiActivity {
    pub parent_title: Option<String>,
    pub project: Option<i32>,
    pub id: Option<i32>,
    pub name: String,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
    pub color: Option<String>,
}
