use crate::web::error;

#[allow(unused_imports)]
pub use self::error::{Error, Result};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    username: String,
}

pub struct JWTBuilder {
    secret: String,
    username: String,
}

impl JWTBuilder {
    pub fn new() -> Result<Self> {
        let secret = std::env::var("JWT_SECRET")?;

        Ok(Self {
            secret: secret,
            username: "".to_string(),
        })
    }

    pub fn username(&mut self, username: &str) -> &Self {
        self.username = username.to_string();
        self
    }

    pub fn to_token(&self) -> Result<String> {
        let now = Utc::now();
        let expire = now + Duration::seconds(60);

        let claim = Claims {
            exp: expire.timestamp() as usize,
            iat: now.timestamp() as usize,
            username: self.username.clone(),
        };

        let key = EncodingKey::from_secret(self.secret.as_ref());

        Ok(encode(&Header::default(), &claim, &key)?)
    }
}

pub fn verify_jwt(token: &str) -> Result<()> {
    let secret = std::env::var("JWT_SECRET")?;
    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256))?;

    Ok(())
}