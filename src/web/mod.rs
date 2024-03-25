mod error;
mod routes;

#[allow(unused_imports)]
pub use self::error::{Error, Result};

use axum::middleware;

#[allow(unused_imports)]
use axum::routing::{delete, get, post, put};
use axum::Router;
mod guard;

use routes::{login, logout, clusters};

use crate::model::ModelManager;

pub async fn get_routes() -> Result<Router> {
    let mm = ModelManager::new().await?;

    let guarded_routes = axum::Router::new()
        
        .route("/clusters", get(clusters::get_clusters))
        .route("/logout", post(logout::handler))
        .route_layer(middleware::from_fn_with_state(mm.clone(), guard::guard))
        .with_state(mm.clone());

    let unguarded_routes = axum::Router::new()
        .route("/login", post(login::handler))
        .with_state(mm);

    let routes = axum::Router::new()
        .nest("/api", guarded_routes)
        .nest("/api", unguarded_routes);

    Ok(routes)
}
