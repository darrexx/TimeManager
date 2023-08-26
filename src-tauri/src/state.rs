use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;
use tokio::sync::MutexGuard;

use crate::{config::Config, db::models::Activity};

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

impl Default for Frontend {
    fn default() -> Self {
        Self {
            current_activity: None,
            popout_active: false,
        }
    }
}

pub type FrontendState = Mutex<Frontend>;

impl From<&MutexGuard<'_, Frontend>> for Frontend {
    fn from(value: &MutexGuard<'_, Frontend>) -> Self {
        Self {
            current_activity: value.current_activity.clone(),
            popout_active: value.popout_active,
        }
    }
}

pub type ConfigState = Mutex<Config>;

pub fn set_start_state(
    timer: &mut MutexGuard<'_, Timer>,
    activity_name: &String,
    existing_activity: &Option<Activity>,
) {
    timer.running = true;
    timer.activity_name = Some(activity_name.to_owned());
    timer.start_time = Some(chrono::Utc::now());

    timer.activity_duration = match existing_activity {
        Some(activity) => activity.duration.map(chrono::Duration::milliseconds),
        None => None,
    };
}
