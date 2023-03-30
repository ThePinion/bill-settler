use std::marker::PhantomData;

use crate::vertices::{DbLabel, DbSavable, DbVertex};

pub mod date_props;
pub mod trusts;

pub struct DbEdge<S, P, T, L>
where
    S: DbVertex,
    T: DbVertex,
    P: DbSavable,
    L: DbLabel,
{
    pub source_id: i64,
    pub target_id: i64,
    pub props: P,
    _source_phantom: PhantomData<S>,
    _target_phantom: PhantomData<T>,
    _label_phantom: PhantomData<L>,
}

impl<S, P, T, L> DbEdge<S, P, T, L>
where
    S: DbVertex,
    T: DbVertex,
    P: DbSavable,
    L: DbLabel,
{
    pub fn _new(s_id: i64, t_id: i64, props: P) -> Self {
        DbEdge {
            source_id: s_id,
            target_id: t_id,
            props: props,
            _source_phantom: PhantomData,
            _target_phantom: PhantomData,
            _label_phantom: PhantomData,
        }
    }
}

impl<S, P, T, L> DbLabel for DbEdge<S, P, T, L>
where
    S: DbVertex,
    T: DbVertex,
    P: DbSavable,
    L: DbLabel,
{
    fn g_label() -> &'static str {
        L::g_label()
    }
}
