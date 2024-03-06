use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as AxumJson,
};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::{
    model::{user::UserBMC, ModelManager},
    auth,
};

// Struct to represent the logout responsetoken.
#[derive(Serialize, Debug)]
pub struct LogoutResponse {
    message: String,
}

// Asynchronous handler function for login requests.
pub async fn handler(
    State(model): State<ModelManager>,
) -> Result<AxumJson<LogoutResponse>, StatusCode> {
    
    Ok(AxumJson(LogoutResponse {
        message: "TODO".to_string(),
    }))
}
