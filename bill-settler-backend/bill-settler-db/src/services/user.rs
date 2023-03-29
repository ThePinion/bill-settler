use crate::edges::knows::KnowsE;
use crate::error::DbError;
use crate::unique_fields;
use crate::{
    db_client::DbClient,
    vertices::user::{PasswordUser, User},
};

pub trait UserServiceT<'a> {
    fn add_user(&self, pu: PasswordUser) -> Result<User, DbError>;
    fn get_all_users(&self) -> Result<Vec<User>, DbError>;
    fn add_know_edge(&self, edge: KnowsE) -> Result<(), DbError>;
}

pub struct UserService<'a> {
    client: &'a DbClient,
}

impl<'a> UserService<'a> {
    pub fn new(client: &'a DbClient) -> Self {
        UserService { client }
    }
}

impl UserServiceT<'_> for UserService<'_> {
    fn add_user(&self, pu: PasswordUser) -> Result<User, DbError> {
        let user = User::from(pu.clone());

        unique_fields!(
            self.client.traversal, User;
            user.g_prop_email(),
            user.g_prop_handle()
        );

        self.client.add_vertex::<User>(vec![
            user.g_prop_email(),
            user.g_prop_handle(),
            pu.g_prop_password(),
        ])
    }

    fn get_all_users(&self) -> Result<Vec<User>, DbError> {
        self.client.get_all()
    }

    fn add_know_edge(&self, edge: KnowsE) -> Result<(), DbError> {
        self.client.add_edge(edge)
    }
}
