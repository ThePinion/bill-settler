mod routes;
mod shutdown_signal;

use anyhow::Result;
use axum::Extension;
use axum::{extract::ws::WebSocketUpgrade, response::Response, routing::get, Router};
use database::db_client::DbClient;
use models::vertices::user::PasswordUser;
use services::group_service::GroupService;
use services::user_service::UserService;
use std::net::SocketAddr;
use yerpc::axum::handle_ws_rpc;
use yerpc::{RpcClient, RpcSession};

#[derive(Clone)]
pub struct Api {
    pub user_service: UserService,
    pub group_service: GroupService,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_client = DbClient::with_config("localhost", 8182);
    let api = Api {
        user_service: UserService::new(db_client.clone()),
        group_service: GroupService::new(db_client),
    };

    //TODO: Only for demonstration purposes
    api.user_service.add_user(PasswordUser {
        email: "test@test.pl".to_string(),
        handle: "test".to_string(),
        password: "test".to_string(),
        name: "Test1".to_string(),
    })?;

    let app = Router::new()
        .route("/rpc", get(handler))
        .layer(Extension(api));

    #[cfg(debug_assertions)]
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    #[cfg(not(debug_assertions))]
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    // let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal::listen())
        .await?;
    tracing::info!("Shutting down");

    Ok(())
}

async fn handler(ws: WebSocketUpgrade, Extension(api): Extension<Api>) -> Response {
    let (client, out_channel) = RpcClient::new();
    let session = RpcSession::new(client, api);
    handle_ws_rpc(ws, out_channel, session).await
}
