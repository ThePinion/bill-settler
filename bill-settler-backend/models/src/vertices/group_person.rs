use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::FromGMap;
use typescript_type_def::TypeDef;

#[derive(Debug, PartialEq, FromGMap, DbLabel, DbVertex, DbSavable, serde::Serialize, TypeDef)]
pub struct GroupPerson {
    pub id: i64,
    pub name: String,
}

impl GroupPerson {
    pub fn new(name: String) -> Self {
        GroupPerson { id: 0, name }
    }
}
