use crate::model::error::{Error, Result};
use crate::model::ModelManager;
use serde::{Deserialize, Serialize};
// use sqlx::types::Uuid;
use sqlx::{error, FromRow};

#[derive(Debug)]
pub struct UserBMC {}

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct User {
    pub username: String,
    pub cluster_id: Option<String>,
    pub pwd_salt: String,
    pub pwd_hash: String,
    pub token: Option<String>,
}

impl UserBMC {
    pub async fn get_all(model: &ModelManager) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&model.db)
            .await?;

        Ok(users)
    }

    pub async fn get_by_username(model: &ModelManager, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&model.db)
            .await?;

        Ok(user)
    }

    pub async fn get_by_token(model: &ModelManager, token: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE token = $1", token)
            .fetch_optional(&model.db)
            .await?;

        Ok(user)
    }

    pub async fn update_token(model: &ModelManager, username: &str, token: &str) -> Result<()> {
        let _ = sqlx::query!(
            "UPDATE users SET token = $1 WHERE username = $2",
            token,
            username
        )
        .execute(&model.db)
        .await?;

        Ok(())
    }
}
