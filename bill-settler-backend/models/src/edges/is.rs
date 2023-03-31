use database_macro::{DbEdge, DbLabel, DbSavable, EnumGValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, DbLabel, DbSavable, DbEdge)]
pub struct Is {
    pub source_id: i64,
    pub target_id: i64,
    test: Test,
}

impl Is {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        Is {
            source_id: s_id,
            target_id: t_id,
            test: Test::Siala(0.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumGValue)]
pub enum Test {
    Siala(f32),
    Mala(String),
}
