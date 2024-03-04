use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    username: String,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = now + Duration::seconds(60);

    let claim = Claims {
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
        username: "test".to_string(),
    };

    let secret = "secret";
    let key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::default(), &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_jwt(token: &str) -> Result<(), StatusCode> {
    let secret = "secret";
    let key = DecodingKey::from_secret(secret.as_bytes());
    
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_jwt() {
        let token = create_jwt().unwrap();
        assert_eq!(verify_jwt(&token), Ok(()));
    }

    #[test]
    fn test_invalid_jwt() {
        let token = "invalid";
        assert_eq!(verify_jwt(token), Err(StatusCode::INTERNAL_SERVER_ERROR));
    }
}
