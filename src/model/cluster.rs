#[allow(unused_imports)]
use crate::model::error::{Error, Result};
use crate::model::ModelManager;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::ctx::Ctx;
pub struct ClusterBMC {}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GetClustersResponse {
    pub id: String,
    pub region: Option<String>,
 
}

impl ClusterBMC {
    pub async fn get_all(ctx: &Ctx, model: &ModelManager) -> Result<Vec<GetClustersResponse>> {

        let clusters = sqlx::query_as!(
            GetClustersResponse,
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

}