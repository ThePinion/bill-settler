use database_macro::DbSavable;
use gremlin_client::derive::FromGMap;

#[derive(Debug, PartialEq, FromGMap, DbSavable)]
pub struct Expense {
    pub id: i64,
    pub amount: f32,
}
