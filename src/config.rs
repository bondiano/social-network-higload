use std::sync::Arc;

use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Clone, ValueEnum, Debug, Serialize, PartialEq, Eq, Default, Copy)]
pub enum Environment {
  #[default]
  Development,
  Test,
  Production,
}

impl Environment {
  pub fn is_dev(&self) -> bool {
    matches!(self, Environment::Development | Environment::Test)
  }
}

#[derive(Parser, Debug, Clone, Default)]
#[clap(author, about, long_about = None)]
pub struct AppConfig {
  /// Set environment
  #[clap(short, long, env, default_value = "development")]
  pub environment: Environment,

  /// Set server host
  #[clap(long, env, default_value = "0.0.0.0")]
  pub host: String,

  /// Set server port
  /// Default: 4238
  #[clap(long, env, default_value = "4238")]
  pub port: u16,

  /// Set database url
  #[clap(long, env)]
  pub database_url: String,

  /// Set redis url
  #[clap(long, env)]
  pub redis_url: String,

  /// Set secret key
  #[clap(long, env)]
  pub jwt_secret: String,

  /// Set JWT expiration time
  #[clap(long, env, default_value = "3600")] // 1 hour
  pub jwt_access_expiration: i64,

  /// Set JWT refresh expiration time in seconds
  #[clap(long, env, default_value = "259200")] // 3 days
  pub jwt_refresh_expiration: i64,
}

pub type AppConfigRc = Arc<AppConfig>;
