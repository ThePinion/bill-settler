use std::{error::Error, fmt::Display};

use gremlin_client::GremlinError;

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone)]
pub enum DbError {
    Unexpected,

    Gremlin(String),

    NotCreated,
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Unexpected => write!(f, "An unexpected error occurred!"),
            DbError::Gremlin(msg) => write!(f, "A Gremlin error occurred: {msg}"),
            DbError::NotCreated => write!(f, "The entity was not created!"),
        }
    }
}

impl Error for DbError {}

impl From<GremlinError> for DbError {
    fn from(e: GremlinError) -> Self {
        DbError::Gremlin(e.to_string())
    }
}
