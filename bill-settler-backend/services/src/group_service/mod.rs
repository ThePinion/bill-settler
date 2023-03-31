use database::{db_client::DbClient, error::DbResult};
use models::{
    edges::{belongs_to::BelongsTo, created::Created, is::Is, paid_expense::PaidExpense},
    vertices::{expense::Expense, group::Group, group_person::GroupPerson, user::User},
};

use self::{
    new_expense::NewExpense,
    new_group_person::{NewGroupPerson, NewGroupPersonAlias},
};

pub mod new_expense;
pub mod new_group_person;

pub struct GroupService<'a> {
    client: &'a DbClient,
}

impl<'a> GroupService<'a> {
    pub fn new(db_client: &'a DbClient) -> Self {
        GroupService { client: db_client }
    }
    pub fn add_group(&self, user_id: i64, name: String) -> DbResult<Group> {
        let group = self.client.add_vertex_r(Group::new(name))?;
        self.client.add_edge_r(Created::new(user_id, group.id))?;
        Ok(group)
    }
    pub fn add_group_person(&self, request: NewGroupPerson) -> DbResult<GroupPerson> {
        let person = match request.alias {
            NewGroupPersonAlias::User { user_id } => {
                let u: User = self.client.get_vertex(user_id)?;
                let p = self.client.add_vertex_r(GroupPerson::new(u.name))?;
                self.client.add_edge_r(Is::new(p.id, user_id))?;
                p
            }
            NewGroupPersonAlias::NonUser { name } => {
                self.client.add_vertex_r(GroupPerson::new(name))?
            }
        };

        self.client
            .add_edge_r(Created::new(request.creator_id, person.id))?;

        self.client
            .add_edge_r(BelongsTo::new(person.id, request.group_id))?;

        Ok(person)
    }
    pub fn add_expense(&self, request: NewExpense) -> DbResult<Expense> {
        let expense = self.client.add_vertex_r(Expense::new(request.amount))?;
        self.client
            .add_edge_r(Created::new(request.creator_id, expense.id))?;
        self.client
            .add_edge_r(PaidExpense::new(request.group_person_id, expense.id))?;

        Ok(expense)
    }
}
