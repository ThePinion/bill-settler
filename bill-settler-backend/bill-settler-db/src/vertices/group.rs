use gremlin_client::derive::{FromGMap, FromGValue};

use crate::db_client::{IntoPropPair, PropPair};

use super::{DbLabel, DbSavable, DbVertex};

#[derive(Debug, PartialEq, FromGValue, FromGMap)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl DbLabel for Group {
    fn g_label() -> &'static str {
        stringify!(Group)
    }
}

impl DbVertex for Group {
    fn id(&self) -> i64 {
        self.id
    }
}

impl DbSavable for Group {
    fn g_props(&self) -> Vec<PropPair> {
        vec![(stringify!(name), &self.name.clone()).into_pair()]
    }
}
