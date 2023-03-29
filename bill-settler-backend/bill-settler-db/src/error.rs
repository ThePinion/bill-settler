use gremlin_client::GremlinError;

use crate::db_client::PredicatePair;

pub type DbResult<T> = core::result::Result<T, DbError>;

#[derive(Debug)]
pub enum DbError {
    Unexpected,
    NotUnique(PredicatePair),
    Gremlin(GremlinError),
}

impl From<GremlinError> for DbError {
    fn from(e: GremlinError) -> Self {
        DbError::Gremlin(e)
    }
}
