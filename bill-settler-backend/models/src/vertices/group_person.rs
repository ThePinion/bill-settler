use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::FromGMap;

#[derive(Debug, PartialEq, FromGMap, DbLabel, DbVertex, DbSavable, serde::Serialize)]
pub struct GroupPerson {
    pub id: i64,
    pub name: String,
}

impl GroupPerson {
    pub fn new(name: String) -> Self {
        GroupPerson { id: 0, name }
    }
}
