use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::config::models::Config;

pub struct Timer {
    pub running: bool,
    pub start_time: Option<DateTime<chrono::Utc>>,
    pub pause_start_time: Option<DateTime<chrono::Utc>>,
    pub activity_name: Option<String>,
    pub activity_duration: Option<chrono::Duration>,
}

pub type TimerState = Mutex<Timer>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Frontend {
    pub current_activity: Option<String>,
    pub popout_active: bool,
}

pub type FrontendState = Mutex<Frontend>;

pub type ConfigState = Mutex<Config>;
