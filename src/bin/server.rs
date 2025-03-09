use std::net::SocketAddr;

use clap::Parser;
use soc_net::{app, config::AppConfig, errors::common::InitError};

use tokio::signal;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> miette::Result<()> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(tracing_subscriber::EnvFilter::from_env("LOG_LEVEL"))
    .try_init()
    .map_err(InitError::Logger)?;

  let config = AppConfig::parse();
  debug!("Run with config: {:?}", config);

  let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
    .await
    .map_err(InitError::Bind)?;

  let server = axum::serve(
    listener,
    app(config)
      .await?
      .into_make_service_with_connect_info::<SocketAddr>(),
  )
  .with_graceful_shutdown(shutdown_signal());

  info!(
    "🚀 Server with API doc on http://{}/api-docs started successfully",
    server.local_addr().map_err(InitError::Bind)?
  );

  server.await.map_err(InitError::Bind)?;

  Ok(())
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }

  debug!("signal received, starting graceful shutdown");
}
