use crate::entity::{DbLabel, DbSavable};

pub trait DbVertex:
    TryFrom<gremlin_client::GValue> + TryFrom<gremlin_client::Map> + DbLabel
{
    fn id(&self) -> i64;
}

pub trait DbRetrieveSavable<T: DbVertex>: DbSavable + DbLabel {}

impl<T> DbRetrieveSavable<T> for T
where
    T: DbVertex + DbSavable,
    Self: Sized,
{
}
