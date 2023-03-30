use crate::entity::{DbLabel, DbSavable};

pub trait DbVertex: TryFrom<gremlin_client::Map> + DbLabel {
    fn id(&self) -> i64;
}

pub trait DbSavableV<T: DbVertex>: DbSavable + DbLabel {}

impl<T> DbSavableV<T> for T
where
    T: DbVertex + DbSavable,
    Self: Sized,
{
}
