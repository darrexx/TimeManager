use serde::{Deserialize, Serialize};
use tokio::sync::MutexGuard;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
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
                automatically_update_workitems: value.devops_config.automatically_update_workitems,
            },
        }
    }
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

impl Default for AzureDevopsConfig {
    fn default() -> Self {
        Self {
            user: String::from("user"),
            pat: String::from("pat"),
            base_url: String::from("dev.azure.com"),
            organization: String::from("organization"),
            project: String::from("project"),
            team: String::from("team"),
            automatically_update_workitems: false,
        }
    }
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

impl From<FrontendConfig> for Config {
    fn from(value: FrontendConfig) -> Self {
        Self {
            devops_config: AzureDevopsConfig::from(value.devops_config),
        }
    }
}

impl From<FrontendAzureDevopsConfig> for AzureDevopsConfig {
    fn from(value: FrontendAzureDevopsConfig) -> Self {
        Self {
            base_url: value.base_url,
            user: value.user,
            pat: value.pat,
            organization: value.organization,
            project: value.project,
            team: value.team,
            automatically_update_workitems: value
                .automatically_update_workitems
                .to_lowercase()
                .parse()
                .unwrap(),
        }
    }
}
