pub mod group_service;
pub mod user_service;

// fn main() -> Result<(), DbError> {
//     let db_client = DbClient::with_config("localhost", 8182);

//     let user_service = UserService::new(&db_client);
//     let group_service = GroupService::new(&db_client);
//     let users = vec![
//         PasswordUser::new("0@test.pl", "0", "secret", "Janusz Kowalski"),
//         PasswordUser::new("1@test.pl", "1", "secret", "Robert Lewandowski"),
//     ]
//     .into_iter()
//     .map(|u| user_service.add_user(u).unwrap())
//     .collect::<Vec<_>>();

//     user_service.trust_users(users[0].id, users[1].id)?;

//     let group = group_service.add_group(users[0].id, "First Group".into())?;
//     let person = group_service.add_group_person(NewGroupPerson::with_user(
//         users[0].id,
//         group.id,
//         users[1].id,
//     ))?;
//     let expense = group_service.add_expense(NewExpense {
//         group_person_id: person.id,
//         creator_id: users[0].id,
//         amount: 15.0,
//         schema: ExpenseSchema::PayerOnly,
//     })?;
//     println!("{:?}", expense);

//     Ok(())
// }
