mod error;
#[allow(unused_imports)]
pub use self::error::{Error, Result};

mod guard;
mod rpc;
mod routes;

use crate::model::{ModelManager, ModelChannel};
use routes::{clusters, login, logout};

#[allow(unused_imports)]
use axum::routing::{delete, get, post, put};
use axum::middleware;
use axum::Router;

pub async fn get_routes(model_channel: ModelChannel) -> Result<Router> {

    let mm = ModelManager::new(model_channel).await?;

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
        .route_layer(middleware::from_fn_with_state(mm.clone(), guard::guard))
        .nest("/api", Router::new().route("/login", post(login::handler)))
        .with_state(mm);

    Ok(routes)
}