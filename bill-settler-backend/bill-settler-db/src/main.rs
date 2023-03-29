use gremlin_client::{
    derive::{FromGMap, FromGValue},
    process::traversal::traversal,
    ConnectionOptions, GremlinClient,
};

use crate::{
    db_client::DbClient,
    models::user::{PasswordUser, User},
};

mod db_client;
mod models;

#[derive(Debug, PartialEq, FromGValue, FromGMap)]
struct TestVertex {
    id: i64,
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ConnectionOptions::builder()
        .host("localhost")
        .port(8182)
        .build();

    let client = GremlinClient::connect(options).expect("Can connect");

    let g = traversal().with_remote(client);

    let client = DbClient::new(g.clone());

    let new_user = client.add_user(PasswordUser::new("2@test.pl", "2", "secret"));

    println!("{:?}", new_user);

    let results = g
        .v(())
        .value_map(true)
        .iter()?
        .filter_map(Result::ok)
        .map(User::try_from)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    println!("{:#?}", results);

    Ok(())
}
