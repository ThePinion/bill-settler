use database_macro::{DbEdge, DbLabel, DbSavable, EnumGValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, DbLabel, DbSavable, DbEdge)]
pub struct OwesExpenseSchema {
    pub source_id: i64,
    pub target_id: i64,
    pub schema: ExpenseSchemaType,
}

impl OwesExpenseSchema {
    pub fn with_absolute(s_id: i64, t_id: i64, amount: f32) -> Self {
        OwesExpenseSchema {
            source_id: s_id,
            target_id: t_id,
            schema: ExpenseSchemaType::Absolute(amount),
        }
    }

    pub fn with_fraction(s_id: i64, t_id: i64, fraction: f32) -> Self {
        OwesExpenseSchema {
            source_id: s_id,
            target_id: t_id,
            schema: ExpenseSchemaType::Fraction(fraction),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumGValue)]
pub enum ExpenseSchemaType {
    Absolute(f32),
    Fraction(f32),
}
