use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workitem {
    pub id: i64,
    pub name: String,
}
