mod error;
mod routes;

#[allow(unused_imports)]
pub use self::error::{Error, Result};

use axum::{extract::State, middleware};

#[allow(unused_imports)]
use axum::routing::{delete, get, post, put};
use axum::Router;
mod guard;
mod jwt_auth;

use paho_mqtt::async_client;
use routes::{api, login, logout};

use crate::model::ModelManager;

pub async fn get_routes() -> Result<Router> {
    let mm = ModelManager::new().await?;

    let routes = axum::Router::new()
        .route("/logout", post(logout::handler))
        .route_layer(middleware::from_fn_with_state(mm.clone(), guard::guard))
        .route("/login", post(login::handler))
        .with_state(mm);

    // .route("/clusters", axum::routing::get(get_clusters))
    // .route("/clusters/:cluster_id/devices", axum::routing::get(get_devices))
    // .route("/clusters/:cluster_id/devices/:device_token", axum::routing::get(get_devices_info))

    Ok(routes)
}
