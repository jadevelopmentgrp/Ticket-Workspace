use crate::Config;
use deadpool::managed::PoolConfig;
use deadpool::Runtime;
use deadpool_redis::{Config as RedisConfig, Pool};
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

/// panics on err
#[tracing::instrument(skip(config))]
pub fn build_redis(config: &Config) -> Pool {
    let mut cfg = RedisConfig::from_url(config.get_redis_uri());
    cfg.pool = Some(PoolConfig::new(config.redis_threads));

    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool")
}
