use crate::web::error::{Error, Result};
use crate::auth;
use crate::context::Ctx;
use crate::model::user::UserBMC;
use crate::model::ModelManager;
use crate::auth::jwt::verify_jwt;

use axum::extract::State;
use axum::{extract::Request, middleware::Next, response::Response};
use axum_auth::AuthBearer;

pub async fn guard(
    State(model): State<ModelManager>,
    AuthBearer(token): AuthBearer,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    // Convert the token to a string
    let token_str = token.as_str();

    let user = match UserBMC::get_by_token(&model, token_str).await? {
        Some(user) => user,
        None => return Err(Error::AuthError(auth::Error::JWTValidationError)),
    };
    
    // Verify the JWT
    if let Err(_) = verify_jwt(&token_str) {
        return Err(Error::AuthError(auth::Error::JWTValidationError));
    }

    // If the JWT is valid, insert the context into the request extensions
    request.extensions_mut().insert(Ctx {
        jwt: token_str.to_string(),
        username: user.username,
    });

    // Pass the request to the next middleware or handler
    Ok(next.run(request).await)
}