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
  /// Default: 4228
  #[clap(long, env, default_value = "4228")]
  pub port: u16,
}

pub type AppConfigRc = Arc<AppConfig>;
