use gremlin_client::{
    process::traversal::{traversal, GraphTraversalSource, SyncTerminator},
    ConnectionOptions, GremlinClient,
};

use crate::{
    edge::DbEdge,
    entity::{DbLabel, DbSavable},
    error::{DbError, DbResult},
    vertex::{DbRetrieveSavable, DbVertex},
};

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
    pub fn add_vertex_retrieve<D, T>(&self, vertex: D) -> DbResult<T>
    where
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
        D: DbRetrieveSavable<T>,
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
