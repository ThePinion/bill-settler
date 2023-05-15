use crate::Api;
use models::vertices::user::{PasswordUser, User};
use models::vertices::{expense::Expense, group::Group, group_person::GroupPerson};
use serde::Deserialize;
use services::group_service::new_expense::NewExpense;
use services::group_service::{
    new_expense::ExpenseSchema,
    new_group_person::{NewGroupPerson, NewGroupPersonAlias},
};
use typescript_type_def::TypeDef;
use yerpc::{rpc, Error};

#[derive(Debug, Deserialize, TypeDef, Clone)]
struct TrustUsersRequest {
    source_id: i64,
    target_id: i64,
}
#[derive(Debug, Deserialize, TypeDef, Clone)]
struct AddUserRequest {
    email: String,
    handle: String,
    password: String,
    name: String,
}

#[derive(Debug, Deserialize, TypeDef, Clone)]
struct LoginRequest {
    handle: String,
    password: String,
}

#[derive(Debug, serde::Deserialize, TypeDef)]
struct AddGroupRequest {
    user_id: i64,
    name: String,
}

#[derive(Debug, serde::Deserialize, TypeDef)]
struct AddGroupPersonRequest {
    pub creator_id: i64,
    pub group_id: i64,
    pub alias: NewGroupPersonAlias,
}

#[derive(Debug, serde::Deserialize, TypeDef)]
struct AddExpenseRequest {
    pub group_person_id: i64,
    pub creator_id: i64,
    pub amount: f32,
    pub schema: ExpenseSchema,
}

// ts_outdir = "../../../../bill-settler-app/src/lib"
#[rpc(all_positional)]
impl Api {
    async fn hello_world(&self) -> String {
        "Hello World!".to_string()
    }

    async fn add_user(&self, payload: AddUserRequest) -> Result<User, Error> {
        let user = self
            .user_service
            .add_user(PasswordUser::new(
                &payload.email,
                &payload.handle,
                &payload.password,
                &payload.name,
            ))
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(user)
    }

    async fn login(&self, payload: LoginRequest) -> Result<Option<User>, Error> {
        let user = self
            .user_service
            .login(payload.handle, payload.password)
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(user)
    }

    async fn trust_users(&self, payload: TrustUsersRequest) -> Result<(), Error> {
        self.user_service
            .trust_users(payload.source_id, payload.target_id)
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(())
    }

    async fn add_group(&self, payload: AddGroupRequest) -> Result<Group, Error> {
        let group = self
            .group_service
            .add_group(payload.user_id, payload.name)
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(group)
    }

    async fn add_group_person(&self, payload: AddGroupPersonRequest) -> Result<GroupPerson, Error> {
        let group_person = self
            .group_service
            .add_group_person(NewGroupPerson {
                creator_id: payload.creator_id,
                group_id: payload.group_id,
                alias: payload.alias,
            })
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(group_person)
    }

    async fn add_expense(&self, payload: AddExpenseRequest) -> Result<Expense, Error> {
        let expense = self
            .group_service
            .add_expense(NewExpense {
                group_person_id: payload.group_person_id,
                creator_id: payload.creator_id,
                amount: payload.amount,
                schema: payload.schema,
            })
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(expense)
    }
}
