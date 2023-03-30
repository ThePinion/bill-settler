use std::fmt::Debug;

use gremlin_client::{
    process::traversal::{traversal, GraphTraversalSource, SyncTerminator},
    ConnectionOptions, GValue, GremlinClient, ToGValue,
};

use crate::{
    edges::DbEdge,
    error::{DbError, DbResult},
    vertices::{DbLabel, DbRetrieveSavable, DbSavable, DbVertex},
};

pub type PropPair = (String, GValue);

pub trait IntoPropPair {
    fn into_pair(self) -> PropPair;
}

impl<T> IntoPropPair for (&'static str, T)
where
    GValue: From<T>,
{
    fn into_pair(self) -> PropPair {
        (self.0.into(), self.1.into())
    }
}

#[derive(Clone)]
pub struct GValueHolder(GValue);

//This is stupid but it suposedly has to be that way
impl ToGValue for GValueHolder {
    fn to_gvalue(&self) -> GValue {
        self.0.clone()
    }
}

impl<T> From<T> for GValueHolder
where
    GValue: From<T>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl Debug for GValueHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.0.fmt(f);
    }
}

pub struct DbClient {
    pub traversal: GraphTraversalSource<SyncTerminator>,
}

impl DbClient {
    pub fn new(traversal: GraphTraversalSource<SyncTerminator>) -> Self {
        return Self { traversal };
    }

    pub fn new_use_config(host: &str, port: u16) -> Self {
        let options = ConnectionOptions::builder().host(host).port(port).build();

        let client = GremlinClient::connect(options).expect("Can connect");

        let g = traversal().with_remote(client);

        DbClient::new(g.clone())
    }
}

impl DbClient {
    pub fn add_vertex<D, T>(&self, vertex: D) -> DbResult<T>
    where
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
        D: DbRetrieveSavable<T>,
        T: DbVertex,
    {
        let vertex = self
            .traversal
            .add_v(T::g_label())
            .property_many(vertex.g_props())
            .value_map(true)
            .next()?
            .ok_or(DbError::Unexpected)?;

        Ok(T::try_from(vertex)?)
    }

    pub fn add_edge<S, P, T, L>(&self, edge: DbEdge<S, P, T, L>) -> DbResult<()>
    where
        S: DbVertex,
        T: DbVertex,
        P: DbSavable,
        L: DbLabel,
    {
        self.traversal
            .v(edge.source_id)
            .as_("s")
            .v(edge.target_id)
            .as_("t")
            .add_e(DbEdge::<S, P, T, L>::g_label())
            .property_many(edge.props.g_props())
            .next()
            .unwrap();
        Ok(())
    }

    pub fn get_all<T>(&self) -> DbResult<Vec<T>>
    where
        T: DbVertex,
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
    {
        Ok(self
            .traversal
            .v(())
            .value_map(true)
            .iter()?
            .filter_map(Result::ok)
            .map(T::try_from)
            .filter_map(Result::ok)
            .collect::<Vec<_>>())
    }
}
