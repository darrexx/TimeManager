use super::customer::Customer;
use super::project::Project;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTeamResponse {
    pub id: Option<i32>,
    pub name: String,
    // pub members: Option<Vec<TeamMember>>,
    pub customers: Option<Vec<Customer>>,
    pub projects: Option<Vec<Project>>,
    // pub activities: Option<Vec<Activity>>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTeamsResponse {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
}
