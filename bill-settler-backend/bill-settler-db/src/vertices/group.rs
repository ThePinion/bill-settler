use gremlin_client::{
    derive::{FromGMap, FromGValue},
    GValue,
};

use crate::db_client::{PredicatePair, PropPair};

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
        vec![(stringify!(name).into(), GValue::String(self.name.clone()))]
    }

    fn g_unique_props(&self) -> Vec<PredicatePair> {
        vec![(
            stringify!(name).into(),
            GValue::String(self.name.clone()).into(),
        )]
    }
}
