use std::{
    fmt::{Debug, Formatter},
    time::SystemTime,
};

use database::{db_client::DbClient, error::DbError};

use crate::{
    edges::trusts::TrustsEdge,
    vertices::user::{PasswordUser, User},
};

pub mod edges;
pub mod vertices;

fn main() -> Result<(), DbError> {
    let db_service = DbClient::new_use_config("localhost", 8182);

    let new_users = vec![
        PasswordUser::new("1@test.pl", "1", &format!("{:?}", SystemTime::now())),
        PasswordUser::new("2@test.pl", "2", &format!("{:?}", SystemTime::now())),
        PasswordUser::new("3@test.pl", "3", &format!("{:?}", SystemTime::now())),
    ];

    for user in new_users {
        if let Err(e) = db_service.add_vertex_retrieve(user) {
            println!("{:?}", e)
        }
    }

    let users = db_service.get_all_vertices::<User>()?;
    let edge = TrustsEdge::new(users[0].id, users[1].id);
    db_service.add_edge(edge.0)?;

    Ok(())
}
