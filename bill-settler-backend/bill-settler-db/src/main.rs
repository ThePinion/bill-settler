use gremlin_client::{
    derive::{FromGMap, FromGValue},
    process::traversal::traversal,
    ConnectionOptions, GremlinClient, Vertex,
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

    let results = g
        .v(())
        .value_map(true)
        .iter()?
        .filter_map(Result::ok)
        .map(TestVertex::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:#?}", results);

    Ok(())
}
