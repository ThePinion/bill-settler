use std::time::SystemTime;

use database::date::Date;
use database_macro::{DbEdge, DbLabel, DbSavable};

#[derive(Debug, DbLabel, DbSavable, DbEdge)]
pub struct Created {
    pub source_id: i64,
    pub target_id: i64,
    pub date_created: Date,
}

impl Created {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        Created {
            source_id: s_id,
            target_id: t_id,
            date_created: SystemTime::now().into(),
        }
    }
}
