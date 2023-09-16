use crate::db::models::{Activity, ActivityTime, NewActivity};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use super::models::NewActivityTime;

pub fn create_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
    workitem_id: Option<i64>,
) -> i32 {
    use crate::schema::activities;
    use crate::schema::activities::dsl::id;

    let new_activity = NewActivity {
        name: activity_name,
        created_at: chrono::Utc::now().timestamp_millis(),
        last_modified: chrono::Utc::now().timestamp_millis(),
        duration: None,
        workitem_id,
    };

    diesel::insert_into(activities::table)
        .values(new_activity)
        .returning(id)
        .get_result(connection)
        .unwrap()
}

pub fn create_activity_time(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_id: i32,
) {
    use crate::schema::activity_times;

    let new_activity_time = NewActivityTime {
        activity_id,
        start_time: chrono::Utc::now().timestamp_millis(),
        end_time: None,
    };

    diesel::insert_into(activity_times::table)
        .values(new_activity_time)
        .execute(connection)
        .unwrap();
}

pub fn get_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
) -> Option<Activity> {
    use crate::schema::activities::dsl::*;

    let results = activities
        .filter(name.eq(activity_name))
        .select(Activity::as_select())
        .first(connection);

    match results {
        Ok(activity) => Some(activity),
        Err(diesel::NotFound) => None,
        Err(_) => panic!(),
    }
}

pub fn get_last_activities(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Option<Vec<Activity>> {
    use crate::schema::activities::dsl::*;

    let results = activities
        .order_by(last_modified.desc())
        .limit(5)
        .select(Activity::as_select())
        .load(connection);

    match results {
        Ok(result_activites) => Some(result_activites),
        Err(diesel::NotFound) => None,
        Err(_) => panic!(),
    }
}

pub fn get_last_activity_times(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Option<Vec<ActivityTime>> {
    use crate::schema::activity_times::dsl::*;

    let results = activity_times
        .order_by(id.desc())
        .limit(5)
        .select(ActivityTime::as_select())
        .load(connection);

    match results {
        Ok(result_activity_times) => Some(result_activity_times),
        Err(diesel::NotFound) => None,
        Err(_) => panic!(),
    }
}

pub fn get_all_activities(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Option<Vec<Activity>> {
    use crate::schema::activities::dsl::*;

    let results = activities
        .order_by(last_modified.desc())
        .select(Activity::as_select())
        .load(connection);

    match results {
        Ok(result_activites) => Some(result_activites),
        Err(diesel::NotFound) => None,
        Err(_) => panic!(),
    }
}

pub fn get_all_activity_times(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Option<Vec<ActivityTime>> {
    use crate::schema::activity_times::dsl::*;

    let results = activity_times
        .order_by(id.desc())
        .select(ActivityTime::as_select())
        .load(connection);

    match results {
        Ok(result_activites) => Some(result_activites),
        Err(diesel::NotFound) => None,
        Err(_) => panic!(),
    }
}

pub fn update_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
    activity_duration: chrono::Duration,
) -> Activity {
    use crate::schema::activities::dsl::*;
    let updated_activities = diesel::update(activities)
        .filter(name.eq(activity_name))
        .set((
            duration.eq(activity_duration.num_milliseconds()),
            last_modified.eq(chrono::Utc::now().timestamp_millis()),
        ))
        .get_results(connection)
        .unwrap();

    updated_activities.into_iter().next().unwrap()
}

pub fn update_activity_time(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity: Activity,
) -> ActivityTime {
    use crate::schema::activity_times::dsl::*;

    let time_to_update: ActivityTime = activity_times
        .order_by(id.desc())
        .select(ActivityTime::as_select())
        .first(connection)
        .unwrap();

    let updated_times = diesel::update(activity_times)
        .filter(id.eq(time_to_update.id))
        .set(end_time.eq(activity.last_modified))
        .get_results(connection)
        .unwrap();

    updated_times.into_iter().next().unwrap()
}
