use std::time::SystemTime;

use crate::{
    date::Date,
    db_client::{IntoPropPair, PropPair},
    vertices::DbSavable,
};

pub struct DateProps {
    date: Date,
}

impl DateProps {
    pub fn now() -> Self {
        DateProps {
            date: Date::from(SystemTime::now()),
        }
    }
}

impl DbSavable for DateProps {
    fn g_props(&self) -> Vec<PropPair> {
        vec![(stringify!(date), self.date).into_pair()]
    }
}
