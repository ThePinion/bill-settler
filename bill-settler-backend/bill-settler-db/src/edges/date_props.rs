use std::time::SystemTime;

use gremlin_client::GValue;

use crate::{date::Date, db_client::PropPair, vertices::DbSavable};

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
        vec![(stringify!(date).to_string(), GValue::Date(self.date))]
    }
}
