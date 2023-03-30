use crate::vertices::{user::User, DbLabel};

use super::{date_props::DateProps, DbEdge};

pub type TrustsEdge = DbEdge<User, DateProps, User, Trusts>;

pub struct Trusts {}

impl DbLabel for Trusts {
    fn g_label() -> &'static str {
        stringify!(Trusts)
    }
}

impl TrustsEdge {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        TrustsEdge::_new(s_id, t_id, DateProps::now())
    }
}
