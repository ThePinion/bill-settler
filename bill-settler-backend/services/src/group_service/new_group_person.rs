#[derive(Debug, serde::Deserialize)]
pub enum NewGroupPersonAlias {
    User { user_id: i64 },
    NonUser { name: String },
}

pub struct NewGroupPerson {
    pub creator_id: i64,
    pub group_id: i64,
    pub alias: NewGroupPersonAlias,
}

impl NewGroupPerson {
    pub fn with_user(creator_id: i64, group_id: i64, user_id: i64) -> Self {
        NewGroupPerson {
            creator_id,
            group_id,
            alias: NewGroupPersonAlias::User { user_id },
        }
    }
    pub fn with_name(creator_id: i64, group_id: i64, name: String) -> Self {
        NewGroupPerson {
            creator_id,
            group_id,
            alias: NewGroupPersonAlias::NonUser { name },
        }
    }
}
