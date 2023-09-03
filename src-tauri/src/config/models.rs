use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub devops_config: AzureDevopsConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct AzureDevopsConfig {
    pub base_url: String,
    pub user: String,
    pub pat: String,
    pub organization: String,
    pub project: String,
    pub team: String,
    pub automatically_update_workitems: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrontendConfig {
    pub devops_config: FrontendAzureDevopsConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrontendAzureDevopsConfig {
    pub base_url: String,
    pub user: String,
    pub pat: String,
    pub organization: String,
    pub project: String,
    pub team: String,
    pub automatically_update_workitems: String,
}
