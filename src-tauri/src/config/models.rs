use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub devops_config: AzureDevopsConfig,
    pub kimai_config: KimaiConfig,
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
    pub kimai_config: KimaiConfig,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct KimaiConfig {
    pub base_url: String,
    pub user: String,
    pub token: String,
}
