use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::FromGMap;

#[derive(Debug, PartialEq, FromGMap, DbSavable, DbLabel, DbVertex)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl Group {
    pub fn new(name: String) -> Self {
        Group { id: 0, name: name }
    }
}
