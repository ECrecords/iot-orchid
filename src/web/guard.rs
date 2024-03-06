use crate::model::user::UserBMC;
use crate::model::ModelManager;
// use crate::web::jwt_auth::verify_jwt;
use crate::auth::jwt::verify_jwt;

use axum::extract::State;
use axum::http::header::HeaderMap;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn guard(
    State(model): State<ModelManager>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract the Authorization header and the token
    let auth_header = headers
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = auth_header.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match UserBMC::get_by_token(&model, token).await {
        Ok(Some(_user)) => {
            verify_jwt(token);
        }
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(next.run(request).await)
}
