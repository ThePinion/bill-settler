use crate::entity::{DbLabel, DbSavable};

pub struct DbEdgeMap {
    pub source_id: i64,
    pub target_id: i64,
    pub properties: gremlin_client::Map,
}

impl DbEdgeMap {
    pub fn new(source_id: i64, target_id: i64, properties: gremlin_client::Map) -> DbEdgeMap {
        DbEdgeMap {
            source_id,
            target_id,
            properties,
        }
    }
}

pub trait DbSavableE: DbSavable {}

pub trait DbEdge: TryFrom<DbEdgeMap> + DbLabel + DbSavableE {
    fn source_id(&self) -> i64;
    fn target_id(&self) -> i64;
}
