use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub devops_config: AzureDevopsConfig,
}

impl From<&MutexGuard<'_, Config>> for Config {
    fn from(value: &MutexGuard<'_, Config>) -> Self {
        Config {
            devops_config: AzureDevopsConfig {
                base_url: value.devops_config.base_url.clone(),
                user: value.devops_config.user.clone(),
                pat: value.devops_config.pat.clone(),
                organization: value.devops_config.organization.clone(),
                project: value.devops_config.project.clone(),
                team: value.devops_config.team.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
