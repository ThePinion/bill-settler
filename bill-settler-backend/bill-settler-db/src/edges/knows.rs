use crate::vertices::user::User;

use super::DbEdge;

pub struct KnowsE {
    ids: (i64, i64),
}

impl KnowsE {
    pub fn new(source: &User, target: &User) -> Self {
        KnowsE {
            ids: (source.id, target.id),
        }
    }
}

impl DbEdge<User, User> for KnowsE {
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
