#![allow(unused_imports)]
use crate::web::error::{Error, Result};

use crate::context::Ctx;
use crate::model::cluster::ClusterBMC;
use crate::model::cluster::{GetClusterResponse, GetClusterDevicesResponse};
use crate::model::ModelManager;

use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum_jrpc::{JrpcResult, JsonRpcRequest, JsonRpcExtractor};

pub async fn get_clusters(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
) -> Result<Json<Vec<GetClusterResponse>>> {
    Ok(Json(ClusterBMC::get_all(&ctx, &mm).await?))
}

pub async fn get_cluster(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
    Path(id): Path<String>,
) -> Result<Json<GetClusterResponse>> {
    Ok(Json(ClusterBMC::get(&ctx, &mm, &id).await?))
}

pub async fn create_cluster(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
    Path(_id): Path<String>,
) -> Result<Json<GetClusterResponse>> {
    todo!()
}

pub async fn update_cluster(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
    Path(_id): Path<String>,
    // add json body
) -> JrpcResult {
    todo!()
}

pub async fn delete_cluster(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
) -> JrpcResult {
    todo!()
}

pub async fn get_cluster_devices(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
    Path(id): Path<String>,
) -> Result<Json<Vec<GetClusterDevicesResponse>>> {
    Ok(Json(ClusterBMC::get_devices(&ctx, &mm, &id).await?))
}