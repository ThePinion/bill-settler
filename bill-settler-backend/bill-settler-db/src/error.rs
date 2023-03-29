use gremlin_client::GremlinError;

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
