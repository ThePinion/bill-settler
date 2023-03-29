use crate::{db_client::PropPair, derive_entity};

use gremlin_client::derive::{FromGMap, FromGValue};

derive_entity!(User);
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

impl User {
    pub fn g_prop_email(&self) -> PropPair {
        (stringify!(email).into(), self.email.clone())
    }

    pub fn g_prop_handle(&self) -> PropPair {
        (stringify!(handle).into(), self.handle.clone())
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

    pub fn g_prop_password(&self) -> PropPair {
        (stringify!(password).into(), self.password.clone())
    }

    pub fn g_props(&self) -> Vec<PropPair> {
        let user = User::from(self.clone());
        vec![
            user.g_prop_email().clone(),
            user.g_prop_handle().clone(),
            self.g_prop_password(),
        ]
    }
}
