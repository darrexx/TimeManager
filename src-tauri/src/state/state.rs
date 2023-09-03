use tokio::sync::MutexGuard;

use crate::db::models::Activity;

use super::models::Timer;

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
