use gremlin_client::derive::{FromGMap, FromGValue};

use crate::db_client::{DbClient, DbError};

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
    pub fn g_label() -> &'static str {
        stringify!(User)
    }

    pub fn g_prop_email(&self) -> (String, &str) {
        (stringify!(email).into(), &self.email)
    }

    pub fn g_prop_handle(&self) -> (String, &str) {
        (stringify!(handle).into(), &self.handle)
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

    pub fn g_prop_password(&self) -> (String, &str) {
        (stringify!(password).into(), &self.password)
    }
}

macro_rules! unique_fields {
    ($traversal:expr, $strct:ident; $($x:expr),*) => {
        $(match $traversal
            .v(())
            .has_label($strct::g_label())
            .has($x)
            .count()
            .next()?
        {
            None | Some(0) => (),
            Some(_) => return Err(DbError::NotUnique(($x.0, $x.1.into()))),
        })*
    };
}

macro_rules! add_vertex {
    ($traversal:expr, $strct:ident; $($x:expr),*) => {{
        let values = $traversal
            .add_v($strct::g_label())
            .property_many(vec![
                $($x,)*
            ])
            .value_map(true)
            .next()?
            .ok_or(DbError::Unexpected)?;
        $strct::try_from(values)?
    }};
}

impl DbClient {
    pub fn add_user(&self, pu: PasswordUser) -> Result<User, DbError> {
        let mut user = User::from(pu.clone());

        unique_fields!(
            self.traversal, User;
            user.g_prop_email(),
            user.g_prop_handle()
        );

        Ok(add_vertex!(
            self.traversal, User;
            user.g_prop_email(),
            user.g_prop_handle(),
            pu.g_prop_password()
        ))
    }
}
