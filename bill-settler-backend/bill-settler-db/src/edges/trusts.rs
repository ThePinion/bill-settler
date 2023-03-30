use crate::{derive_label, vertices::user::User};

use super::{date_props::DateProps, DbEdge};

pub type TrustsEdge = DbEdge<User, DateProps, User, Trusts>;

derive_label!(Trusts);
pub struct Trusts {}

impl TrustsEdge {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        TrustsEdge::_new(s_id, t_id, DateProps::now())
    }
}
