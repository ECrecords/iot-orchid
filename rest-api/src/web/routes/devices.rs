use serde::{Deserialize, Serialize};
use axum::extract::{Path, State, Json as AxumJson, Extension};
use axum::Json;
use crate::context::UserContext;
use crate::model::cluster::ClusterBMC;
use crate::model::ModelManager;

use crate::web::error::{Error, Result};


#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub cluster_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDevice {
    pub cluster_id: String,
    pub id: String,
    pub device_type: String,
    pub connection_string: String,
}

pub async fn create_device(
    State(_mm): State<ModelManager>,
    Extension(_ctx): Extension<UserContext>,
    Path(_cluster_id): Path<String>,
    AxumJson(data): AxumJson<CreateDevice>,
) -> Result<Json<Device>> {
   todo!() 
}