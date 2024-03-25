#![allow(unused_imports)]
use crate::web::error::{Error, Result};

use crate::ctx::Ctx;
use crate::model::cluster::ClusterBMC;
use crate::model::cluster::GetClustersResponse;
use crate::model::ModelManager;

use axum::extract::State;
use axum::{Extension, Json};

pub async fn get_clusters(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
) -> Result<Json<Vec<GetClustersResponse>>> {
    Ok(Json(ClusterBMC::get_all(&ctx, &mm).await?))
}