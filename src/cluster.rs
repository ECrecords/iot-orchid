


#[derive(Deserialize)]
struct CreateCluster {
    id: String,
    region: String,
}

#[derive(Serialize)]
struct Cluster {
    id: String,
    region: String,
}

async fn create_cluster(extract::Json(payload): extract::Json<CreateCluster>) -> Json<Cluster> {
    Json(Cluster {
        id: payload.id,
        region: payload.region,
    })
}

async fn cluster_routes() -> Router {
    Router::new()
        .route("/clustert, get(get_clusters))
        .route("/clusters/:cluster_id/devices", get(get_devices))
        .route("/clusters/:cluster_id/devices/:device_token", get(get_devices_info))
        .route("/clusters", post(create_cluster))
}
