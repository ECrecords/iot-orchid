use axum::extract::State;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum::http::header::HeaderMap;
use crate::model::ModelManager;
use crate::web::jwt_auth::{create_jwt, verify_jwt};
use std::sync::Arc;

pub async fn guard(
    // State(model): State<Arc<ModelManager>>,
    headers: HeaderMap,
    request: Request, 
    next: Next) -> Result<Response, StatusCode> {
    
    let token = headers.get("Authorization");

    println!("Token: {:?}", token);

    let response = next.run(request).await;
    Ok(response)

}
