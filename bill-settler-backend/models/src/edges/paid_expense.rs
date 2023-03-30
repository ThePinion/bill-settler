use std::time::SystemTime;

use database::date::Date;
use database_macro::{DbEdge, DbLabel, DbSavable};

#[derive(Debug, DbLabel, DbSavable, DbEdge)]
pub struct PaidExpense {
    pub source_id: i64,
    pub target_id: i64,
    pub date_paid: Date,
}

impl PaidExpense {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        PaidExpense {
            source_id: s_id,
            target_id: t_id,
            date_paid: SystemTime::now().into(),
        }
    }
}
