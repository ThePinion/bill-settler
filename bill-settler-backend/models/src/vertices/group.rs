use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::FromGMap;
use typescript_type_def::TypeDef;

#[derive(Debug, PartialEq, FromGMap, DbSavable, DbLabel, DbVertex, serde::Serialize, TypeDef)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl Group {
    pub fn new(name: String) -> Self {
        Group { id: 0, name: name }
    }
}
