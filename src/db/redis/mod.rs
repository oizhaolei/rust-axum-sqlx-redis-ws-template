use crate::config::Config;
use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8::Pool;
use dotenv::dotenv;
use std::sync::Arc;

pub type Redis = Arc<Pool<RedisConnectionManager>>;

pub async fn redis_connect(config: &Config) -> Pool<RedisConnectionManager> {
    dotenv().ok();
    let manager = RedisConnectionManager::new(config.cache_url.as_str()).unwrap();
    Pool::builder().build(manager).await.unwrap()
}
