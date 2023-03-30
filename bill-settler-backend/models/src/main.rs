use std::time::SystemTime;

use database::{db_client::DbClient, error::DbError};
use edges::trusts::Trusts;

use crate::{
    edges::created::Created,
    vertices::{
        group::Group,
        user::{PasswordUser, User},
    },
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
        println!("{:?}", db_service.add_vertex_r(user))
    }

    let users = db_service.get_all_vertices::<User>()?;
    let group = db_service.add_vertex_r(Group::new("FirstGroup".into()))?;
    let _created = db_service.add_edge_r(Created::new(users[0].id, group.id));
    // TO BE CONTINUED
    let trusts = db_service.add_edge_r(Trusts::new(users[0].id, users[1].id))?;
    println!("{:?}", trusts);

    Ok(())
}
