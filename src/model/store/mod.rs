mod error;

pub use self::error::{Error, Result};

use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

pub type Database = Pool<Postgres>;

pub async fn new_database_pool() -> Result<Pool<Postgres>> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        // FIXME: NEED A BETTER WAY TO HANDLE THIS
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .map_err(|e| Error::PoolCreationFailed(e.to_string()))?;

    Ok(pool)
}