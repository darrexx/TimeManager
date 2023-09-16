use super::team::GetTeamResponse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCustomerResponse {
    pub id: Option<i32>,
    pub name: String,
    pub number: Option<String>,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
    pub company: Option<String>,
    #[serde(rename = "vatId")]
    pub vat_id: Option<String>,
    pub contact: Option<String>,
    pub address: Option<String>,
    pub country: String,
    pub currency: String,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub timezone: String,
    pub teams: Option<Vec<GetTeamResponse>>,
    pub budget: f32,
    #[serde(rename = "timeBudget")]
    pub time_budget: i32,
    #[serde(rename = "budgetType")]
    pub budget_type: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCustomersResponse {
    pub id: Option<i32>,
    pub name: String,
    pub number: Option<String>,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
    pub currency: String,
    pub color: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub id: Option<i32>,
    pub name: String,
    pub number: Option<String>,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
    pub color: Option<String>,
}
