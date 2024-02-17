use axum::{
    extract::{self, Path},
    Extension, Json, Router,
};

use axum::routing::{get, post};

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

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

async fn create_cluster(
    state: Extension<Pool<Postgres>>,
    extract::Json(cluster): extract::Json<CreateCluster>,
) -> Result<Json<Cluster>, Json<Cluster>> {
    let Extension(pool) = state;
    let query =
        sqlx::query("INSERT INTO clusters (id, region) VALUES ($1, $2) RETURNING id, region")
            .bind(&cluster.id)
            .bind(&cluster.region)
            .fetch_optional(&pool)
            .await;

    match query {
        Ok(query) => {
            let query = query.unwrap();

            Ok(Json(Cluster {
                id: query.get(0),
                region: query.get(1),
            }))
        }
        Err(_) => Err(Json(Cluster {
            id: "error".to_string(),
            region: "error".to_string(),
        })),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("Failed to create Postgres connection pool! URL: {}", url));

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => println!("Migrations failed: {}", e),
    }

    let app = Router::new()
        .nest("/api", app().await)
        .layer(Extension(pool))
        .into_make_service();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server started successfully"),
        Err(e) => println!("Server failed: {}", e),
    };
}

async fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/clusters", get(get_clusters))
        .route("/cluster", post(create_cluster))
        .route("/clusters/:id/devices", get(get_devices))
        .route("/clusters/:id/devices/:token", get(get_devices_info))
}
