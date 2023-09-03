use super::models::ActivitiesAndTimes;
use crate::db::{
    activity::{
        get_all_activities, get_all_activity_times, get_last_activities, get_last_activity_times,
    },
    models::{Activity, ActivityTime},
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use tauri::State;

#[tauri::command]
pub fn get_history(db: State<Pool<ConnectionManager<SqliteConnection>>>) -> ActivitiesAndTimes {
    let connection = &mut db.get().unwrap();
    let activity_history = get_last_activities(connection);
    let activity_time_history = get_last_activity_times(connection);

    ActivitiesAndTimes {
        activities: match activity_history {
            Some(activities) => activities,
            None => vec![],
        },
        activity_times: match activity_time_history {
            Some(activity_times) => activity_times,
            None => vec![],
        },
    }
}

#[tauri::command]
pub fn get_activities(db: State<Pool<ConnectionManager<SqliteConnection>>>) -> Vec<Activity> {
    let connection = &mut db.get().unwrap();
    match get_all_activities(connection) {
        Some(activities) => activities,
        None => vec![],
    }
}

#[tauri::command]
pub fn get_activity_times(
    db: State<Pool<ConnectionManager<SqliteConnection>>>,
) -> Vec<ActivityTime> {
    let connection = &mut db.get().unwrap();
    match get_all_activity_times(connection) {
        Some(activities) => activities,
        None => vec![],
    }
}
