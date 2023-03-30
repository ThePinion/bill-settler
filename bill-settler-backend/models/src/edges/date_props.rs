use std::time::SystemTime;

use database::date::Date;
use database_macro::DbSavable;

#[derive(DbSavable)]
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
