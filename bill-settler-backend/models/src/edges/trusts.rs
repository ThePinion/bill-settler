use database::edge::DbEdge;
use database_macro::DbLabel;

use crate::vertices::user::User;

use super::date_props::DateProps;

pub struct TrustsEdge(pub DbEdge<User, DateProps, User, Trusts>);

#[derive(Debug, DbLabel)]
pub struct Trusts {}

impl TrustsEdge {
    pub fn new(s_id: i64, t_id: i64) -> Self {
        TrustsEdge(DbEdge::_new(s_id, t_id, DateProps::now()))
    }
}
