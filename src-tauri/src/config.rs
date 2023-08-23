use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub devops_config: AzureDevopsConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AzureDevopsConfig {
    pub base_url: String,
    pub user: String,
    pub pat: String,
    pub organization: String,
    pub project: String,
    pub team: String,
}

impl Default for AzureDevopsConfig {
    fn default() -> Self {
        Self {
            user: String::from("user"),
            pat: String::from("pat"),
            base_url: String::from("dev.azure.com"),
            organization: String::from("organization"),
            project: String::from("project"),
            team: String::from("team"),
        }
    }
}
