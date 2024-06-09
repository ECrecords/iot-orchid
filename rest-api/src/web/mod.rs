mod error;
#[allow(unused_imports)]
pub use self::error::{Error, Result};

mod guard;
mod routes;
mod rpc;

use crate::model::ModelManager;
use amqprs::connection::Connection;
use routes::{clusters, login, logout};

use axum::middleware;
#[allow(unused_imports)]
use axum::routing::{delete, get, post, put};
use axum::Router;

pub async fn initalize_app(conn: &Connection) -> Result<Router> {

    let model_manager = ModelManager::new(conn).await?;

    let routes = axum::Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/clusters", post(clusters::create_cluster))
                .route("/clusters/:id/devices", post(clusters::add_device))
                .route("/clusters/:id", get(clusters::get_cluster))
                .route("/clusters", get(clusters::get_clusters))
                .route("/clusters/:id/devices", get(clusters::get_cluster_devices))
                .route("/logout", post(logout::handler)),
        )
        .route("/rpc", post(rpc::rpc_handler))
        .route_layer(middleware::from_fn_with_state(
            model_manager.clone(),
            guard::guard,
        ))
        .nest("/api", Router::new().route("/login", post(login::handler)))
        .with_state(model_manager);

    Ok(routes)
}
