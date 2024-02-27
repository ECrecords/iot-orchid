use model::ModelManager;

pub fn routes(model: ModelManager) -> axum::Router {
    let model = axum::extract::Extension::new(model);

    axum::Router::new()
        .route("/clusters", axum::routing::get(get_clusters))
        .route("/clusters/:cluster_id/devices", axum::routing::get(get_devices))
        .route("/clusters/:cluster_id/devices/:device_token", axum::routing::get(get_devices_info))
        .layer(axum::AddExtensionLayer::new(model))

}