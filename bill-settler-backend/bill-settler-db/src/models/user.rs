use gremlin_client::GremlinError;

use crate::db_client::{DbClient, DbError};

pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub handle: String,
}

pub struct PasswordUser {
    pub email: String,
    pub handle: String,
    pub password: String,
}

macro_rules! unique_fields {
    ($traversal:expr, $label:expr; $($x:expr),*) => {
        match $traversal
            .v(())
            .has_label($label)
            $(.has($x))*
            .count()
            .next()?
        {
            None | Some(0) => (),
            Some(_) => return Err(DbError::NotUnique),
        }
    };
}

impl DbClient {
    pub fn add_user(&self, user: PasswordUser) -> Result<User, DbError> {
        unique_fields!(
            self.traversal,
            stringify!(User);
            ("email", user.email),
            ("handle", user.handle)
        );

        todo!()

        // let count = self
        //     .traversal
        //     .v(())
        //     .has_label(stringify!(User))
        //     .has(("email", user.email))
        //     .count()
        //     .next()?
        //     .unwrap();
    }
}
