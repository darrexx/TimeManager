use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::db::models::{Activity, NewActivity};

pub fn create_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
) {
    use crate::schema::activities;

    let new_activity = NewActivity {
        name: activity_name,
        created_at: chrono::Utc::now().timestamp_millis(),
        last_modified: chrono::Utc::now().timestamp_millis(),
        duration: None,
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

pub fn update_activity(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    activity_name: &String,
    activity_duration: chrono::Duration,
) {
    use crate::schema::activities::dsl::*;
    diesel::update(activities)
        .filter(name.eq(activity_name))
        .set(duration.eq(activity_duration.num_milliseconds()))
        .execute(connection)
        .unwrap();
}
