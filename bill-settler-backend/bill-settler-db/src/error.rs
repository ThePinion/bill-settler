use gremlin_client::GremlinError;

pub type DbResult<T> = core::result::Result<T, DbError>;

#[derive(Debug)]
pub enum DbError {
    Unexpected,
    Gremlin(GremlinError),
}

impl From<GremlinError> for DbError {
    fn from(e: GremlinError) -> Self {
        DbError::Gremlin(e)
    }
}
