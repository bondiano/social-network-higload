use std::sync::Arc;

use crate::errors::common::DatabaseResult;
use redis::aio::MultiplexedConnection;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::Mutex;

pub type RedisClient = Arc<Mutex<MultiplexedConnection>>;

#[derive(Debug, Clone)]
pub struct DataSource {
  pub pg: PgPool,
  pub redis: RedisClient,
}

impl DataSource {
  pub async fn init(database_url: &str, redis_uri: &str) -> DatabaseResult<Self> {
    let pg = PgPoolOptions::new().connect(database_url).await?;

    let redis_client = redis::Client::open(redis_uri)?;
    let redis_connection = redis_client.get_multiplexed_async_connection().await?;

    Ok(Self {
      pg,
      redis: Arc::new(Mutex::new(redis_connection)),
    })
  }
}
