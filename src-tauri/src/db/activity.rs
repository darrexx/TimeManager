use crate::db::models::{Activity, NewActivity};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

pub fn create_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
    workitem_id: Option<i64>,
) {
    use crate::schema::activities;

    let new_activity = NewActivity {
        name: activity_name,
        created_at: chrono::Utc::now().timestamp_millis(),
        last_modified: chrono::Utc::now().timestamp_millis(),
        duration: None,
        workitem_id,
    };

    diesel::insert_into(activities::table)
        .values(new_activity)
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

pub fn update_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
    activity_duration: chrono::Duration,
) {
    use crate::schema::activities::dsl::*;
    diesel::update(activities)
        .filter(name.eq(activity_name))
        .set((
            duration.eq(activity_duration.num_milliseconds()),
            last_modified.eq(chrono::Utc::now().timestamp_millis()),
        ))
        .execute(connection)
        .unwrap();
}
