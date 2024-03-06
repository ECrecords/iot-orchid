use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as AxumJson,
};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::{
    model::{user::UserBMC, ModelManager},
    auth::jwt::JWTBuilder,
};

// Struct to represent the login response, containing only the JWT token.
#[derive(Serialize, Debug)]
pub struct LoginResponse {
    token: String,
}

// Struct to deserialize the login request payload.
#[derive(Deserialize, Debug)]
pub struct LoginInPayload {
    username: String,
    password: String,
}

// Asynchronous handler function for login requests.
pub async fn handler(
    State(model): State<ModelManager>,
    Json(payload): Json<LoginInPayload>,
) -> Result<AxumJson<LoginResponse>, StatusCode> {
    // Debug logging to indicate a POST request to /login.
    dbg!("POST /login");

    let (username, password) = (payload.username, payload.password);

    // Attempt to retrieve the user by username.
    let user_result = UserBMC::get_by_username(&model, &username).await;

    // Check the result of the user retrieval.
    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED), // User not found
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR), // Database or query error
    };

    // Verify the provided password against the stored hash.
    let verified = verify(&password, &user.pwd_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !verified {
        return Err(StatusCode::UNAUTHORIZED); // Password does not match
    }

    // Generate a JWT token for the authenticated user.
    let token = JWTBuilder::new()?
        .username(&username)
        .to_token()?;

    // Update the user's token in the database.
    UserBMC::update_token(&model, &username, &token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the JWT token in the response.
    Ok(AxumJson(LoginResponse { token }))
}
