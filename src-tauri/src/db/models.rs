use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::activities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Activity {
    pub id: i32,
    pub name: String,
    pub duration: Option<i64>,
    pub created_at: i64,
    pub last_modified: i64,
    pub workitem_id: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::activities)]
pub struct NewActivity<'a> {
    pub name: &'a str,
    pub duration: Option<i64>,
    pub created_at: i64,
    pub last_modified: i64,
    pub workitem_id: Option<i64>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Activity))]
#[diesel(table_name = crate::schema::activity_times)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityTime {
    pub id: i32,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub activity_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::activity_times)]
pub struct NewActivityTime {
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub activity_id: i32,
}
