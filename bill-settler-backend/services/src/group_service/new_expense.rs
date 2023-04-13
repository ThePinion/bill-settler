pub struct NewExpense {
    pub group_person_id: i64,
    pub creator_id: i64,
    pub amount: f32,
    pub schema: ExpenseSchema,
}

#[derive(Debug, serde::Deserialize)]
pub enum ExpenseSchema {
    EqualAll,
    EqualPersons(Vec<i64>),
    PayerOnly,
}
