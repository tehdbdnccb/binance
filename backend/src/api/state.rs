use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    // High-performance kill switch matrix (Agent ID -> Is Suspended)
    pub agent_status: Arc<RwLock<HashMap<Uuid, bool>>>,
}