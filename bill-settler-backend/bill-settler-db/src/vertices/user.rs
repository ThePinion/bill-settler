use crate::{
    db_client::{IntoPropPair, PropPair},
    derive_label, derive_vertex,
};

use gremlin_client::derive::{FromGMap, FromGValue};

use super::{DbRetrieveSavable, DbSavable};

derive_vertex!(User);
#[derive(Debug, PartialEq, FromGValue, FromGMap)]
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

derive_label!(PasswordUser, User);
#[derive(Clone)]
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

impl DbSavable for PasswordUser {
    fn g_props(&self) -> Vec<PropPair> {
        vec![
            (stringify!(email), &self.email).into_pair(),
            (stringify!(handle), &self.handle).into_pair(),
            (stringify!(password), &self.password).into_pair(),
        ]
    }
}

impl DbRetrieveSavable<User> for PasswordUser {}
