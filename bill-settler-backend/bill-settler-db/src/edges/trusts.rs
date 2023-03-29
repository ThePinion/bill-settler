use std::time::SystemTime;

use gremlin_client::GValue;

use crate::{date::Date, db_client::PropPair, vertices::user::User};

use super::DbEdge;

pub struct TrustsEdge {
    ids: (i64, i64),
    date: Date,
}

impl TrustsEdge {
    pub fn new(source: &User, target: &User) -> Self {
        TrustsEdge {
            ids: (source.id, target.id),
            date: Date::from(SystemTime::now()),
        }
    }
}

impl DbEdge<User, User> for TrustsEdge {
    fn g_label(&self) -> &'static str {
        "knows"
    }

    fn g_source_id(&self) -> i64 {
        self.ids.0
    }

    fn g_target_id(&self) -> i64 {
        self.ids.1
    }

    fn g_props(&self) -> Vec<PropPair> {
        vec![(stringify!(date).to_string(), GValue::Date(self.date))]
    }
}
