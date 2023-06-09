use std::time::SystemTime;

use database::date::Date;
use database_macro::{DbEdge, DbLabel, DbSavable};

#[derive(Debug, DbLabel, DbSavable, DbEdge)]
pub struct Trusts {
    pub source_id: i64,
    pub target_id: i64,
    pub date_trusted: Date,
}

impl Trusts {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        Trusts {
            source_id: s_id,
            target_id: t_id,
            date_trusted: SystemTime::now().into(),
        }
    }
}
