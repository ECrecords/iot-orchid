use axum::{
    extract::{self, Path},
    Extension, Json, Router,
};

use axum::routing::{delete, get, post, put};

use serde::{Deserialize, Serialize};
use sqlx::{pool, postgres::PgPoolOptions, query, Pool, Postgres, Row};

#[derive(Serialize)]
struct ClustersResponse {
    clusters: Vec<String>,
}

async fn get_clusters(
    state: Extension<Pool<Postgres>>,
) -> Result<Json<ClustersResponse>, Json<ClustersResponse>> {
    let Extension(pool) = state;

    let query = sqlx::query!("SELECT id FROM clusters")
        .fetch_all(&pool)
        .await;

    match query {
        Ok(query) => {
            let clusters: Vec<String> = query.iter().map(|row| row.id.clone()).collect();

            Ok(Json(ClustersResponse { clusters }))
        }
        Err(_) => Err(Json(ClustersResponse { clusters: vec![] })),
    }
}

async fn get_devices(Path(cluster_id): Path<String>) -> String {
    format!("List of devices in cluster {}", cluster_id)
}

#[derive(Serialize)]
struct DeviceInfo {
    token: String,
    cluster_id: String,
    topics: Vec<String>,
}

// two path arguments cluster_id and device_token
async fn get_devices_info(
    Path((cluster_id, device_token)): Path<(String, String)>,
) -> Json<DeviceInfo> {
    Json(DeviceInfo {
        token: device_token,
        cluster_id,
        topics: vec!["topic-1".to_string(), "topic-2".to_string()],
    })
}

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

#[tokio::main]
async fn main() {
    // build our application with a single route

    let api_routes = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/clusters", get(get_clusters))
        .route("/clusters", post(create_cluster))
        .route("/clusters/:id/devices", get(get_devices))
        .route("/clusters/:id/devices/:token", get(get_devices_info));

    let app = Router::new().nest("/api", api_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
