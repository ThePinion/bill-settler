mod routes;
mod shutdown_signal;

use std::net::SocketAddr;

use anyhow::Result;
use axum::{Router, routing};
use database::db_client::DbClient;
use services::{user_service::UserService, group_service::GroupService};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub group_service: GroupService,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        user_service: UserService::new(DbClient::with_config("bill-settler-janusgraph", 8182)),
        group_service: GroupService::new(DbClient::with_config("bill-settler-janusgraph", 8182)),
    };

    #[cfg(debug_assertions)]
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    #[cfg(not(debug_assertions))]
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    let app = Router::new()
        .route("/", routing::get(routes::index::root))
        .nest("/users", routes::users::router(state.clone()))
        .nest("/groups", routes::groups::router(state.clone()));

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal::listen())
        .await?;
    tracing::info!("Shutting down");

    Ok(())
}
