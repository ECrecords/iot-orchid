use crate::web::error::{Error, Result};
use crate::auth;
use crate::model::user::UserBMC;
use crate::model::ModelManager;
use crate::auth::jwt::verify_jwt;

use axum::extract::State;
use axum::{extract::Request, middleware::Next, response::Response};

use axum_auth::AuthBearer;

pub async fn guard(
    State(model): State<ModelManager>,
    AuthBearer(token): AuthBearer,
    request: Request,
    next: Next,
) -> Result<Response> {

    let token = token.as_str();

    match UserBMC::get_by_token(&model, token).await? {
        Some(_user) => {
            verify_jwt(token)?;
        }
        None => return Err(Error::AuthError(auth::Error::JWTValidationError)),
    };

    Ok(next.run(request).await)
}