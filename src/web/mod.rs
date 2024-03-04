
mod error;

pub use self::error::{Error, Result};

use axum::{middleware, Extension};
use axum::routing::{get, post, put, delete};
use axum::Router;
mod guard;
mod jwt_auth;

use crate::model::ModelManager;

// pub fn routes(model: ModelManager) -> axum::Router {
pub async fn routes() -> Result<Router> {
    // let model = axum::extract::Extension::new(model);
    // let shared_model = Arc::new(ModelManager::new().await?);
    let routes = axum::Router::new()
        .route("/login", post(|| async { "Login" }))
        .route_layer(middleware::from_fn(guard::guard))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(Extension(ModelManager::new().await?));
        // .route("/clusters", axum::routing::get(get_clusters))
        // .route("/clusters/:cluster_id/devices", axum::routing::get(get_devices))
        // .route("/clusters/:cluster_id/devices/:device_token", axum::routing::get(get_devices_info))

    Ok(routes)
}
