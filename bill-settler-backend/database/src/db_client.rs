use gremlin_client::{
    process::traversal::{traversal, GraphTraversalSource, SyncTerminator},
    ConnectionOptions, GremlinClient,
};

use crate::{
    edge::{DbEdge, DbEdgeMap},
    error::{DbError, DbResult},
    vertex::{DbSavableV, DbVertex},
};

pub struct DbClient {
    pub traversal: GraphTraversalSource<SyncTerminator>,
}

impl DbClient {
    pub fn new(traversal: GraphTraversalSource<SyncTerminator>) -> Self {
        return Self { traversal };
    }

    pub fn with_config(host: &str, port: u16) -> Self {
        let options = ConnectionOptions::builder().host(host).port(port).build();

        let client = GremlinClient::connect(options).expect("Can connect");

        let g = traversal().with_remote(client);

        DbClient::new(g.clone())
    }
}

impl DbClient {
    pub fn add_vertex_r<D, T>(&self, vertex: D) -> DbResult<T>
    where
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
        D: DbSavableV<T>,
        T: DbVertex,
    {
        let vertex = self
            .traversal
            .add_v(D::g_label())
            .property_many(vertex.g_props())
            .value_map(true)
            .next()?
            .ok_or(DbError::Unexpected)?;

        Ok(T::try_from(vertex)?)
    }

    pub fn get_vertex<T>(&self, vertex_id: i64) -> DbResult<T>
    where
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
        T: DbVertex,
    {
        let vertex = self
            .traversal
            .v(vertex_id)
            .has_label(T::g_label())
            .value_map(true)
            .next()?
            .ok_or(DbError::Unexpected)?;

        Ok(T::try_from(vertex)?)
    }

    pub fn add_edge_r<T>(&self, edge: T) -> DbResult<T>
    where
        T: DbEdge,
        DbError: From<<T as TryFrom<DbEdgeMap>>::Error>,
    {
        const SOURCE: &str = "SOURCE";
        const TARGET: &str = "TARGET";
        let output = self
            .traversal
            .v(edge.source_id())
            .as_(SOURCE)
            .v(edge.target_id())
            .as_(TARGET)
            .add_e(T::g_label())
            .from(SOURCE)
            .to(TARGET)
            .property_many(edge.g_props())
            .value_map(())
            .next()?;
        match output {
            Some(map) => Ok(T::try_from(DbEdgeMap::new(
                edge.source_id(),
                edge.target_id(),
                map,
            ))?),
            None => Err(DbError::NotCreated),
        }
    }

    pub fn get_all_vertices<T>(&self) -> DbResult<Vec<T>>
    where
        T: DbVertex,
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
    {
        Ok(self
            .traversal
            .v(())
            .has_label(T::g_label())
            .value_map(true)
            .iter()?
            .filter_map(Result::ok)
            .map(T::try_from)
            .filter_map(Result::ok)
            .collect::<Vec<_>>())
    }
}
