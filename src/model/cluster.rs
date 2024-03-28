#[allow(unused_imports)]
use crate::model::error::{Error, Result};
use crate::context::Ctx;
use crate::model::ModelManager;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub struct ClusterBMC {}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GetClusterResponse {
    pub id: String,
    pub region: Option<String>,
 
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GetClusterDevicesResponse {
    pub id: String,
    // pub last_seen: Option<sqlx::types::chrono::NaiveDateTime>,
}

impl ClusterBMC {
    pub async fn get_all(ctx: &Ctx, model: &ModelManager) -> Result<Vec<GetClusterResponse>> {

        let clusters = sqlx::query_as!(
            GetClusterResponse,
            r#"
            SELECT clusters.id, clusters.region
            FROM clusters
            JOIN user_clusters ON clusters.id = user_clusters.cluster_id
            WHERE user_clusters.user_id = $1
            "#,
            ctx.username()
        )
        .fetch_all(&model.db)
        .await?;

        Ok(clusters)
    }

    pub async fn get(ctx: &Ctx, model: &ModelManager, id: &str) -> Result<GetClusterResponse> {
        let cluster = sqlx::query_as!(
            GetClusterResponse,
            r#"
            SELECT clusters.id, clusters.region
            FROM clusters
            JOIN user_clusters ON clusters.id = user_clusters.cluster_id
            WHERE user_clusters.user_id = $1 AND clusters.id = $2
            "#,
            ctx.username(),
            id
        )
        .fetch_one(&model.db)
        .await?;
    
        Ok(cluster)
    }

    pub async fn get_devices(ctx: &Ctx, model: &ModelManager, id: &str) -> Result<Vec<GetClusterDevicesResponse>> {
        let devices = sqlx::query_as!(
            GetClusterDevicesResponse,
            r#"
            SELECT cluster_devices.id
            FROM cluster_devices
            JOIN clusters ON cluster_devices.cluster_id = clusters.id
            JOIN user_clusters ON clusters.id = user_clusters.cluster_id
            WHERE user_clusters.user_id = $1 AND clusters.id = $2
            "#,
            ctx.username(),
            id
        )
        .fetch_all(&model.db)
        .await?;
    
        Ok(devices)
    }
}