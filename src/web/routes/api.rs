use axum::Router;



pub fn routes() -> Router {
    Router::new()
        .route("/clusters", todo!())
        .route("/clusters/:cluster_id/devices", todo!())
        .route("/clusters/:cluster_id/devices/:device_token", todo!())
} 