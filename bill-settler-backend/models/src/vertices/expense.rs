use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::FromGMap;

#[derive(Debug, PartialEq, FromGMap, DbLabel, DbVertex, DbSavable)]
pub struct Expense {
    pub id: i64,
    pub amount: f32,
}

impl Expense {
    pub fn new(amount: f32) -> Self {
        Expense { id: 0, amount }
    }
}
