use database::{db_client::DbClient, error::DbResult};
use models::{
    edges::{
        belongs_to::BelongsTo, created::Created, is::Is, owes_expense_schema::OwesExpenseSchema,
        paid_expense::PaidExpense,
    },
    vertices::{expense::Expense, group::Group, group_person::GroupPerson, user::User},
};

use self::{
    new_expense::NewExpense,
    new_group_person::{NewGroupPerson, NewGroupPersonAlias},
};

pub mod new_expense;
pub mod new_group_person;

#[derive(Clone)]
pub struct GroupService {
    client: DbClient,
}

impl GroupService {
    pub fn new(db_client: DbClient) -> Self {
        GroupService { client: db_client }
    }
    pub fn add_group(&self, user_id: i64, name: String) -> DbResult<Group> {
        let group = self.client.add_vertex_r(Group::new(name))?;
        self.client
            .add_edge_r::<_, User, Group>(Created::new(user_id, group.id))?;
        Ok(group)
    }
    pub fn add_group_person(&self, request: NewGroupPerson) -> DbResult<GroupPerson> {
        let person = match request.alias {
            NewGroupPersonAlias::User { user_id } => self.create_person_with_alias(user_id)?,
            NewGroupPersonAlias::NonUser { name } => {
                self.client.add_vertex_r(GroupPerson::new(name))?
            }
        };

        self.client
            .add_edge_r::<_, User, GroupPerson>(Created::new(request.creator_id, person.id))?;

        self.client
            .add_edge_r::<_, GroupPerson, Group>(BelongsTo::new(person.id, request.group_id))?;

        Ok(person)
    }

    fn create_person_with_alias(&self, user_id: i64) -> DbResult<GroupPerson> {
        let u: User = self.client.get_vertex(user_id)?;
        let p = self.client.add_vertex_r(GroupPerson::new(u.name))?;
        self.client
            .add_edge_r::<_, GroupPerson, User>(Is::new(p.id, user_id))?;
        Ok(p)
    }

    pub fn add_expense(&self, request: NewExpense) -> DbResult<Expense> {
        let expense = self.client.add_vertex_r(Expense::new(request.amount))?;
        self.client
            .add_edge_r::<_, User, Expense>(Created::new(request.creator_id, expense.id))?;
        self.client
            .add_edge_r::<_, GroupPerson, Expense>(PaidExpense::new(
                request.group_person_id,
                expense.id,
            ))?;

        match request.schema {
            //Stupid but simple so it's the only one implemented:))
            new_expense::ExpenseSchema::PayerOnly => {
                self.client.add_edge_r::<_, GroupPerson, Expense>(
                    OwesExpenseSchema::with_fraction(request.group_person_id, expense.id, 1.0),
                )?
            }
            _ => todo!("Not implemented!"),
        };

        Ok(expense)
    }
}
