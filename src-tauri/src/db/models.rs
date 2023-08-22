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
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::activities)]
pub struct NewActivity<'a> {
    pub name: &'a str,
    pub duration: Option<i64>,
    pub created_at: i64,
    pub last_modified: i64,
}