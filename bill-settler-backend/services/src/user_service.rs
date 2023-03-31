use database::{db_client::DbClient, error::DbResult};
use models::{
    edges::trusts::Trusts,
    vertices::user::{PasswordUser, User},
};

pub struct UserService<'a> {
    client: &'a DbClient,
}

impl<'a> UserService<'a> {
    pub fn new(db_client: &'a DbClient) -> Self {
        UserService { client: db_client }
    }
    pub fn add_user(&self, user: PasswordUser) -> DbResult<User> {
        self.client.add_vertex_r(user)
    }
    pub fn trust_users(&self, source_id: i64, target_id: i64) -> DbResult<Trusts> {
        Ok(self
            .client
            .add_edge_r::<_, User, User>(Trusts::new(source_id, target_id))?)
    }
}
