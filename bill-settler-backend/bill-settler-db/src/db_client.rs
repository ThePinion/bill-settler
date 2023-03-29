use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

use crate::{edges::DbEdge, error::DbError, vertices::DbVertex};

pub type PropPair<'a> = (String, &'a str);

pub struct DbClient {
    pub traversal: GraphTraversalSource<SyncTerminator>,
}

impl DbClient {
    pub fn new(traversal: GraphTraversalSource<SyncTerminator>) -> Self {
        return Self { traversal };
    }
}

impl DbClient {
    pub fn add_vertex<T>(&self, props: Vec<PropPair>) -> Result<T, DbError>
    where
        T: DbVertex,
        DbError: From<<T as TryFrom<gremlin_client::Map>>::Error>,
    {
        let vertex = self
            .traversal
            .add_v(T::g_label())
            .property_many(props)
            .value_map(true)
            .next()?
            .ok_or(DbError::Unexpected)?;

        Ok(T::try_from(vertex)?)
    }

    pub fn add_edge<S: DbVertex, T: DbVertex, E: DbEdge<S, T>>(
        &self,
        edge: E,
    ) -> Result<(), DbError> {
        self.traversal
            .v(edge.source_id())
            .as_("s")
            .v(edge.target_id())
            .as_("t")
            .add_e(edge.label())
            .next()
            .unwrap();
        Ok(())
    }

    pub fn get_all<T>(&self) -> Result<Vec<T>, DbError>
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
