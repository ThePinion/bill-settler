use edges::knows::KnowsE;
use error::DbError;
use gremlin_client::derive::{FromGMap, FromGValue};

use crate::{
    services::{user::UserServiceT, DbService, DbServiceT},
    vertices::user::PasswordUser,
};

mod db_client;
mod edges;
mod error;
mod services;
mod utils;
mod vertices;

#[derive(Debug, PartialEq, FromGValue, FromGMap)]
struct TestVertex {
    id: i64,
    name: Option<String>,
}

fn main() -> Result<(), DbError> {
    let db_service = DbService::new_use_config("localhost", 8182);

    let user_service = db_service.user_service();
    user_service.add_user(PasswordUser::new("1@test.pl", "1", "secret"))?;
    user_service.add_user(PasswordUser::new("2@test.pl", "2", "secret"))?;
    user_service.add_user(PasswordUser::new("3@test.pl", "3", "secret"))?;

    let users = user_service.get_all_users()?;
    let edge = KnowsE::new(&users[0], &users[1]);
    user_service.add_know_edge(edge)?;

    // println!("{:#?}", results);

    Ok(())
}
