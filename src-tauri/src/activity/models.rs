use serde::Serialize;

use crate::db::models::{Activity, ActivityTime};

#[derive(Serialize)]
pub struct ActivitiesAndTimes {
    pub activities: Vec<Activity>,
    pub activity_times: Vec<ActivityTime>,
}
