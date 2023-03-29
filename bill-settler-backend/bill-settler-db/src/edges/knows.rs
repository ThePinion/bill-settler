use crate::vertices::user::User;

use super::DbEdge;

pub struct KnowsEdge {
    ids: (i64, i64),
}

impl KnowsEdge {
    pub fn new(source: &User, target: &User) -> Self {
        KnowsEdge {
            ids: (source.id, target.id),
        }
    }
}

impl DbEdge<User, User> for KnowsEdge {
    fn label(&self) -> &'static str {
        "knows"
    }

    fn source_id(&self) -> i64 {
        self.ids.0
    }

    fn target_id(&self) -> i64 {
        self.ids.1
    }
}
