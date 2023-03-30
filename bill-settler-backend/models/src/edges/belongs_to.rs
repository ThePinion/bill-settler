use database_macro::{DbEdge, DbLabel, DbSavable};

#[derive(Debug, DbLabel, DbSavable, DbEdge)]
pub struct BelongsTo {
    pub source_id: i64,
    pub target_id: i64,
}

impl BelongsTo {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        BelongsTo {
            source_id: s_id,
            target_id: t_id,
        }
    }
}
