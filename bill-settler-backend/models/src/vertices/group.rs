use database::{entity::DbSavable, prop::PropPair};
use gremlin_client::derive::{FromGMap, FromGValue};

#[derive(Debug, PartialEq, FromGValue, FromGMap)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl DbSavable for Group {
    fn g_props(&self) -> Vec<PropPair> {
        vec![database::prop::IntoPropPair::into_pair((
            stringify!(name),
            &self.name,
        ))]
    }
}
