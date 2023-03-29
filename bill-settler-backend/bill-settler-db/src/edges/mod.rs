use crate::vertices::DbVertex;

pub mod knows;

pub trait DbEdge<S: DbVertex, T: DbVertex> {
    fn label(&self) -> &'static str;
    fn source_id(&self) -> i64;
    fn target_id(&self) -> i64;
}
