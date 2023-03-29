use crate::db_client::{PredicatePair, PropPair};

pub mod user;

pub trait DbLabel {
    fn g_label() -> &'static str;
}

pub trait DbVertex:
    TryFrom<gremlin_client::GValue> + TryFrom<gremlin_client::Map> + DbLabel
{
    fn id(&self) -> i64;
}

pub trait DbSavable<T: DbVertex>: DbLabel {
    fn g_props(&self) -> Vec<PropPair>;
    fn g_unique_props(&self) -> Vec<PredicatePair>;
}
