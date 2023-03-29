use gremlin_client::{
    process::traversal::{GraphTraversalSource, SyncTerminator},
    GremlinError,
};

pub struct DbClient {
    pub traversal: GraphTraversalSource<SyncTerminator>,
}

impl DbClient {
    pub fn new(traversal: GraphTraversalSource<SyncTerminator>) -> Self {
        return Self { traversal };
    }
}

#[derive(Debug)]
pub enum DbError {
    Unexpected,
    NotUnique((String, String)),
    Gremlin(GremlinError),
}

impl From<GremlinError> for DbError {
    fn from(e: GremlinError) -> Self {
        DbError::Gremlin(e)
    }
}
