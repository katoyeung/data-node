use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use redis::Client;
use std::env;
use std::sync::Arc;
use std::time::Duration;

pub async fn create_redis_pool() -> Arc<Pool<RedisConnectionManager>> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).expect("Failed to create Redis client");
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder()
        .max_open(20) // Maximum number of connections in the pool
        .max_idle_lifetime(Some(Duration::from_secs(30))) // Correct method name for setting max idle lifetime
        .build(manager); // Directly returns a Pool instance without needing .expect()

    Arc::new(pool)
}
