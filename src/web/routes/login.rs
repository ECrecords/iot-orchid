use axum::{
    extract::{Json, State},
    response::Json as AxumJson,
};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::{
    auth::jwt::JWTBuilder,
    model::{user::UserBMC, ModelManager},
    web::error::{Error, Result},
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
) -> Result<AxumJson<LoginResponse>> {
    // Debug logging to indicate a POST request to /login.
    dbg!("POST /login");

    let (username, password) = (payload.username, payload.password);

    // Attempt to retrieve the user by username.
    let user = UserBMC::get_by_username(&model, &username).await?;

    // If the user is not found.
    let user = match user {
        Some(user) => user,
        None => return Err(Error::LoginFailUserNotFound),
    };

    // Verify the provided password.
    if !verify(&password, &user.pwd_hash).map_err(|_| Error::LoginFailPwdNotMatch)? {
        return Err(Error::LoginFailPwdNotMatch);
    }
    

    // Generate a JWT token for the authenticated user.
    let token = JWTBuilder::new()?.username(&username).to_token()?;

    // Update the user's token in the database.
    UserBMC::update_token(&model, &username, &token).await?;

    // Return the JWT token in the response.
    Ok(AxumJson(LoginResponse { token }))
}
