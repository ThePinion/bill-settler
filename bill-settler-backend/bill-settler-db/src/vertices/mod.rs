use crate::db_client::PropPair;

pub mod group;
pub mod user;

pub trait DbLabel {
    fn g_label() -> &'static str;
}

pub trait DbVertex:
    TryFrom<gremlin_client::GValue> + TryFrom<gremlin_client::Map> + DbLabel
{
    fn id(&self) -> i64;
}

pub trait DbRetrieveSavable<T: DbVertex>: DbSavable + DbLabel {}

pub trait DbSavable {
    fn g_props(&self) -> Vec<PropPair>;
}

impl<T> DbRetrieveSavable<T> for T
where
    T: DbVertex + DbSavable,
    Self: Sized,
{
}
