use database::vertex::DbRetrieveSavable;
use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::{FromGMap, FromGValue};

#[derive(Debug, PartialEq, FromGValue, FromGMap, DbLabel, DbVertex)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub handle: String,
}

impl From<PasswordUser> for User {
    fn from(pu: PasswordUser) -> Self {
        User {
            id: 0,
            email: pu.email,
            handle: pu.handle,
        }
    }
}

#[derive(Clone, DbSavable, DbLabel)]
#[DbLabel = "User"]
pub struct PasswordUser {
    pub email: String,
    pub handle: String,
    pub password: String,
}

impl PasswordUser {
    pub fn new(email: &str, handle: &str, password: &str) -> Self {
        PasswordUser {
            email: email.into(),
            handle: handle.into(),
            password: password.into(),
        }
    }
}

impl DbRetrieveSavable<User> for PasswordUser {}
