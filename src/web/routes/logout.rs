#![allow(unused_imports)]
use crate::web::error::{Error, Result};

use axum::extract::{State, Extension};
use axum::Json;
use serde::Serialize;

use crate::{
    model::{user::UserBMC, ModelManager},
    auth,
    context::Ctx,
};

use axum_auth::AuthBearer;

// Struct to represent the logout responsetoken.
#[derive(Serialize, Debug)]
pub struct LogoutResponse {
    result: String,
}

// Asynchronous handler function for login requests.
pub async fn handler(
    State(model): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
) -> Result<Json<LogoutResponse>> {

    let token = ctx.jwt().to_string();

    match UserBMC::get_by_token(&model, &token).await? {
        Some(user) => {
            UserBMC::update_token(&model, &user.username, "").await?;
        }
        None => return Err(auth::Error::JWTValidationError.into()),
    };

    Ok(Json(LogoutResponse {
        result: String::from("success"),
    }))
}
