use std::time::SystemTime;

use database::{db_client::DbClient, error::DbError};
use models::vertices::user::PasswordUser;

use crate::{
    group_service::{new_expense::NewExpense, GroupService},
    user_service::UserService,
};

pub mod group_service;
pub mod user_service;

fn main() -> Result<(), DbError> {
    let db_client = DbClient::with_config("localhost", 8182);

    let user_service = UserService::new(&db_client);
    let group_service = GroupService::new(&db_client);
    let users = vec![
        PasswordUser::new("0@test.pl", "0", &format!("{:?}", SystemTime::now())),
        PasswordUser::new("1@test.pl", "1", &format!("{:?}", SystemTime::now())),
    ]
    .into_iter()
    .map(|u| user_service.add_user(u).unwrap())
    .collect::<Vec<_>>();

    user_service.trust_users(users[0].id, users[1].id)?;

    let group = group_service.add_group(users[0].id, "First Group".into())?;
    let expense = group_service.add_expense(NewExpense {
        group_id: group.id,
        payer_id: users[1].id,
        creator_id: users[0].id,
        amount: 15.0,
    })?;
    println!("{:?}", expense);

    Ok(())
}
