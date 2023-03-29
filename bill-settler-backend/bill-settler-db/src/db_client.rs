use std::fmt::Debug;

use gremlin_client::{
    process::traversal::{traversal, GraphTraversalSource, SyncTerminator},
    ConnectionOptions, GValue, GremlinClient, ToGValue,
};

use crate::{
    edges::DbEdge,
    error::{DbError, DbResult},
    vertices::{DbSavable, DbVertex},
};

#[derive(Debug, Clone)]
pub struct GValueHolder(GValue);

pub type PropPair = (String, GValue);
pub type PredicatePair = (String, GValueHolder);

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
        D: DbSavable<T>,
        T: DbVertex,
    {
        for prop in vertex.g_unique_props() {
            match self
                .traversal
                .v(())
                .has_label(T::g_label())
                .has(prop.clone())
                .count()
                .next()?
            {
                None | Some(0) => (),
                Some(_) => return Err(DbError::NotUnique(prop)),
            }
        }

        let vertex = self
            .traversal
            .add_v(T::g_label())
            .property_many(vertex.g_props())
            .value_map(true)
            .next()?
            .ok_or(DbError::Unexpected)?;

        Ok(T::try_from(vertex)?)
    }

    pub fn add_edge<S: DbVertex, T: DbVertex, E: DbEdge<S, T>>(&self, edge: E) -> DbResult<()> {
        self.traversal
            .v(edge.g_source_id())
            .as_("s")
            .v(edge.g_target_id())
            .as_("t")
            .add_e(edge.g_label())
            .property_many(edge.g_props())
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
