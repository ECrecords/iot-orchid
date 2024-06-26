#[allow(unused_imports)]
use crate::model::error::{Error, Result};
use crate::context::UserContext;
use crate::model::ModelManager;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use paho_mqtt as mqtt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDeviceRequest {
    pub cluster_id: String,
    pub device_id: String,
}

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
    pub async fn get_all(ctx: &UserContext, model: &ModelManager) -> Result<Vec<GetClusterResponse>> {

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

    pub async fn get(ctx: &UserContext, model: &ModelManager, id: &str) -> Result<GetClusterResponse> {
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

    async fn has_access(ctx: &UserContext, model: &ModelManager, id: &str) -> Result<bool> {
        let cluster = sqlx::query!(
            r#"
            SELECT clusters.id
            FROM clusters
            JOIN user_clusters ON clusters.id = user_clusters.cluster_id
            WHERE user_clusters.user_id = $1 AND clusters.id = $2 AND clusters.token = $3
            "#,
            ctx.username(),
            id,
            ctx.jwt()
        )
        .fetch_optional(&model.db)
        .await?;
    
        Ok(cluster.is_some())
    }

    pub async fn get_devices(ctx: &UserContext, model: &ModelManager, id: &str) -> Result<Vec<GetClusterDevicesResponse>> {
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

    pub async fn ping_all(ctx: &UserContext, model: &ModelManager, id: &str) -> Result<Vec<String>> {

        let devices = Self::get_devices(&ctx, &model, &id).await?;
        let mut res: Vec<String> = vec![];
        for device in devices {
            model.mqtt().publish(mqtt::MessageBuilder::new()
                .topic(format!("cluster/{}/device/{}/ping", id, device.id))
                .payload("ping")
                .qos(1)
                .finalize()
            ).await.unwrap();
            let topic = format!("cluster/{}/device/{}", id, device.id);
            res.push(topic.clone());
        }
    
        Ok(res)
    }

    pub async fn add_device(ctx: &UserContext, model: &ModelManager, device: AddDeviceRequest) -> Result<()> {
        let device_id = device.device_id;
        let cluster_id = device.cluster_id;

        // check if user has access to cluster or if the cluster exists
        Self::has_access(&ctx, &model, &cluster_id).await?;

        sqlx::query!(
            r#"
            INSERT INTO cluster_devices (cluster_id, id)
            VALUES ($1, $2)
            "#,
            cluster_id,
            device_id
        )
        .execute(&model.db)
        .await?;
    
        Ok(())
    }


    pub async fn register_device(ctx: &UserContext, model: &ModelManager, id: &str) -> Result<()> {
        todo!()
    }
}