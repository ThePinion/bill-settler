use crate::{db_client::PropPair, vertices::DbVertex};

pub mod trusts;

pub trait DbEdge<S: DbVertex, T: DbVertex> {
    fn g_label(&self) -> &'static str;
    fn g_source_id(&self) -> i64;
    fn g_target_id(&self) -> i64;
    fn g_props(&self) -> Vec<PropPair> {
        vec![]
    }
}
