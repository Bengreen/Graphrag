use crate::config::AppConfig;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<AppConfig>,
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}
