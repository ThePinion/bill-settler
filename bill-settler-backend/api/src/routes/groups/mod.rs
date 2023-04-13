use axum::{extract::{State, Json}, routing, Router};
use models::vertices::{group::Group, group_person::GroupPerson, expense::Expense};
use services::group_service::{new_group_person::{NewGroupPersonAlias, NewGroupPerson}, new_expense::{ExpenseSchema, NewExpense}};

use super::AppError;
use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::post(add_group))
        .route("/add", routing::post(add_group_person))
        .route("/expense", routing::post(add_expense))
        .with_state(state)
}

#[derive(Debug, serde::Deserialize)]
struct AddGroupRequest {
    user_id: i64,
    name: String,
}
async fn add_group(
    State(state): State<AppState>,
    Json(payload): Json<AddGroupRequest>,
) -> Result<Json<Group>, AppError> {
    let group = state
        .group_service
        .add_group(payload.user_id, payload.name)?;
    Ok(Json(group))
}

#[derive(Debug, serde::Deserialize)]
struct AddGroupPersonRequest{
    pub creator_id: i64,
    pub group_id: i64,
    pub alias: NewGroupPersonAlias,
}
async fn add_group_person(
    State(state): State<AppState>,
    Json(payload): Json<AddGroupPersonRequest>,
) -> Result<Json<GroupPerson>, AppError> {
    let group_person = state
        .group_service
        .add_group_person(NewGroupPerson {
            creator_id: payload.creator_id,
            group_id: payload.group_id,
            alias: payload.alias,
        })?;
    Ok(Json(group_person))
}

#[derive(Debug, serde::Deserialize)]
struct AddExpenseRequest {
    pub group_person_id: i64,
    pub creator_id: i64,
    pub amount: f32,
    pub schema: ExpenseSchema,
}
async fn add_expense(
    State(state): State<AppState>,
    Json(payload): Json<AddExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    let expense = state
        .group_service
        .add_expense(NewExpense {
            group_person_id: payload.group_person_id,
            creator_id: payload.creator_id,
            amount: payload.amount,
            schema: payload.schema,
        })?;
    Ok(Json(expense))
}
