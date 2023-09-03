use tokio::sync::MutexGuard;

use super::models::Frontend;

impl Default for Frontend {
    fn default() -> Self {
        Self {
            current_activity: None,
            popout_active: false,
        }
    }
}

impl From<&MutexGuard<'_, Frontend>> for Frontend {
    fn from(value: &MutexGuard<'_, Frontend>) -> Self {
        Self {
            current_activity: value.current_activity.clone(),
            popout_active: value.popout_active,
        }
    }
}
