use crate::db_client::{PredicatePair, PropPair};

use gremlin_client::{
    derive::{FromGMap, FromGValue},
    GValue,
};

use super::{DbLabel, DbSavable, DbVertex};

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

impl DbLabel for User {
    fn g_label() -> &'static str {
        return stringify!(User).into();
    }
}

impl DbVertex for User {
    fn id(&self) -> i64 {
        return self.id;
    }
}

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

impl DbLabel for PasswordUser {
    fn g_label() -> &'static str {
        return stringify!(User).into();
    }
}

impl DbSavable<User> for PasswordUser {
    fn g_props(&self) -> Vec<PropPair> {
        vec![
            (stringify!(email).into(), GValue::String(self.email.clone())),
            (
                stringify!(handle).into(),
                GValue::String(self.handle.clone()),
            ),
            (
                stringify!(password).into(),
                GValue::String(self.password.clone()),
            ),
        ]
    }

    fn g_unique_props(&self) -> Vec<PredicatePair> {
        vec![
            (stringify!(email).into(), self.email.clone().into()),
            (stringify!(handle).into(), self.handle.clone().into()),
        ]
    }
}
