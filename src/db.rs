use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

impl DatabaseConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.url.trim().is_empty() {
            return Err("database.url must not be empty".to_string());
        }
        if self.pool_size == 0 {
            return Err("database.pool_size must be greater than 0".to_string());
        }
        if self.timeout_seconds == 0 {
            return Err("database.timeout_seconds must be greater than 0".to_string());
        }
        Ok(())
    }
}

pub async fn init_db(config: &DatabaseConfig) -> Result<PgPool, String> {
    let pool_options = PgPoolOptions::new()
        .max_connections(config.pool_size)
        .acquire_timeout(Duration::from_secs(config.timeout_seconds));

    let pool = pool_options
        .connect(&config.url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Verify pgvector is enabled
    let pgvector_check: Option<(i32,)> =
        sqlx::query_as("SELECT 1 FROM pg_extension WHERE extname = 'vector'")
            .fetch_optional(&pool)
            .await
            .map_err(|e| format!("Failed to query pg_extension: {}", e))?;

    if pgvector_check.is_none() {
        return Err("pgvector extension is not enabled in the database. Please ensure 'CREATE EXTENSION vector;' has been run.".to_string());
    }

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("Failed to run database migrations: {}", e))?;

    Ok(pool)
}
