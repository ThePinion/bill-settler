use gremlin_client::GremlinError;
use thiserror::Error;

pub type DbResult<T> = core::result::Result<T, DbError>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Unexpected error")]
    Unexpected,
    #[error("Gremlin error: {0}")]
    Gremlin(GremlinError),
    #[error("Not created error")]
    NotCreated,
}

impl From<GremlinError> for DbError {
    fn from(e: GremlinError) -> Self {
        DbError::Gremlin(e)
    }
}
