use database::vertex::DbSavableV;
use database_macro::{DbLabel, DbSavable, DbVertex};
use gremlin_client::derive::{FromGMap, FromGValue};

#[derive(Debug, PartialEq, FromGValue, FromGMap, DbLabel, DbVertex, serde::Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub handle: String,
    pub name: String,
}

impl From<PasswordUser> for User {
    fn from(pu: PasswordUser) -> Self {
        User {
            id: 0,
            email: pu.email,
            handle: pu.handle,
            name: pu.name,
        }
    }
}

#[derive(Clone, DbSavable, DbLabel)]
#[DbLabel = "User"]
pub struct PasswordUser {
    pub email: String,
    pub handle: String,
    pub password: String,
    pub name: String,
}

impl PasswordUser {
    pub fn new(email: &str, handle: &str, password: &str, name: &str) -> Self {
        PasswordUser {
            email: email.into(),
            handle: handle.into(),
            password: password.into(),
            name: name.into(),
        }
    }
}

impl DbSavableV<User> for PasswordUser {}
