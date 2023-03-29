use gremlin_client::{process::traversal::traversal, ConnectionOptions, GremlinClient};

use crate::db_client::DbClient;

use self::user::{UserService, UserServiceT};

pub mod user;

pub struct DbService {
    client: DbClient,
}

pub trait DbServiceT<'a, UserS>
where
    UserS: UserServiceT<'a>,
{
    fn user_service(&'a self) -> UserS;
}

impl DbService {
    pub fn new(client: DbClient) -> Self {
        DbService { client: client }
    }

    pub fn new_use_config(host: &str, port: u16) -> Self {
        let options = ConnectionOptions::builder().host(host).port(port).build();

        let client = GremlinClient::connect(options).expect("Can connect");

        let g = traversal().with_remote(client);

        let client = DbClient::new(g.clone());

        Self::new(client)
    }
}

impl<'a> DbServiceT<'a, UserService<'a>> for DbService {
    fn user_service(&'a self) -> UserService<'a> {
        UserService::new(&self.client)
    }
}
