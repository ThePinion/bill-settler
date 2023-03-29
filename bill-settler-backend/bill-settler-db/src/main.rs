use db_client::DbClient;
use edges::trusts::TrustsEdge;
use error::DbError;
use gremlin_client::derive::{FromGMap, FromGValue};
use vertices::user::User;

use crate::vertices::user::PasswordUser;

mod date;
mod db_client;
mod edges;
mod error;
mod utils;
mod vertices;

#[derive(Debug, PartialEq, FromGValue, FromGMap)]
struct TestVertex {
    id: i64,
    name: Option<String>,
}

fn main() -> Result<(), DbError> {
    let db_service = DbClient::new_use_config("localhost", 8182);

    let new_users = vec![
        PasswordUser::new("1@test.pl", "1", "secret"),
        PasswordUser::new("2@test.pl", "2", "secret"),
        PasswordUser::new("3@test.pl", "3", "secret"),
    ];

    for user in new_users {
        let _ = db_service.add_vertex::<PasswordUser, User>(user);
    }

    let users = db_service.get_all::<User>()?;
    let edge = TrustsEdge::new(&users[0], &users[1]);
    db_service.add_edge(edge)?;

    // println!("{:#?}", results);

    Ok(())
}
