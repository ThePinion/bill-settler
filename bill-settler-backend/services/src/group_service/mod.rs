use database::{db_client::DbClient, error::DbResult};
use models::{
    edges::{belongs_to::BelongsTo, created::Created, paid_expense::PaidExpense},
    vertices::{expense::Expense, group::Group},
};

use self::new_expense::NewExpense;

pub mod new_expense;

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
    pub fn add_expense(&self, request: NewExpense) -> DbResult<Expense> {
        let expense = self.client.add_vertex_r(Expense::new(request.amount))?;
        self.client
            .add_edge_r(Created::new(request.creator_id, expense.id))?;
        self.client
            .add_edge_r(PaidExpense::new(request.payer_id, expense.id))?;
        self.client
            .add_edge_r(BelongsTo::new(expense.id, request.group_id))?;
        Ok(expense)
    }
}
