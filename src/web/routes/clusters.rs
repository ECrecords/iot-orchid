#![allow(unused_imports)]
use std::ops::Add;

use crate::web::error::{Error, Result};

use serde::{Deserialize, Serialize};

use crate::context::UserContext;
use crate::model::cluster::ClusterBMC;
use crate::model::cluster::{GetClusterResponse, GetClusterDevicesResponse, AddDeviceRequest};
use crate::model::ModelManager;

#[derive(Deserialize)]
pub struct AddDeviceToClusterRequest {
    device_id: String,
}

use axum::extract::{Path, State, Json as AxumJson};
use axum::{Extension, Json};
use axum_jrpc::{JrpcResult, JsonRpcRequest, JsonRpcExtractor};
pub async fn get_clusters(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<UserContext>,
) -> Result<Json<Vec<GetClusterResponse>>> {
    Ok(Json(ClusterBMC::get_all(&ctx, &mm).await?))
}

pub async fn get_cluster(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<UserContext>,
    Path(id): Path<String>,
) -> Result<Json<GetClusterResponse>> {
    Ok(Json(ClusterBMC::get(&ctx, &mm, &id).await?))
}

pub async fn create_cluster(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<UserContext>,
    Path(_id): Path<String>,
) -> Result<Json<GetClusterResponse>> {
    todo!()
}

pub async fn add_device(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<UserContext>,
    Path(cluster_id): Path<String>,
    AxumJson(data): AxumJson<AddDeviceToClusterRequest>,
) -> Result<String> {
    let data = AddDeviceRequest {
        cluster_id: cluster_id,
        device_id: data.device_id,
    };
    
    ClusterBMC::add_device(&ctx, &mm, data).await?;

    Ok("test".to_string())
}

pub async fn get_cluster_devices(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<UserContext>,
    Path(id): Path<String>,
) -> Result<Json<Vec<GetClusterDevicesResponse>>> {
    Ok(Json(ClusterBMC::get_devices(&ctx, &mm, &id).await?))
}