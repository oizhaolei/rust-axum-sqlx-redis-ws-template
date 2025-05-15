use crate::config::Config;
use crate::db::redis::{Redis, redis_connect};
use axum::Extension;
use std::sync::Arc;

pub type CacheExt = Extension<Arc<CacheImpl>>;

pub async fn create_cache(config: &Config) -> CacheImpl {
    let redis_pool = Arc::new(redis_connect(config).await);
    CacheImpl::new(redis_pool)
}
pub struct CacheImpl {
    pub redis_pool: Redis,
}
impl CacheImpl {
    pub fn new(pool: Redis) -> Self {
        Self { redis_pool: pool }
    }
}
