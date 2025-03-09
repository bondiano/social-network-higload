use axum::{
  http::{header::InvalidHeaderValue, StatusCode},
  response::{IntoResponse, Response},
};

use miette::Diagnostic;
use thiserror::Error;
use tracing::error;
use tracing_subscriber::util::TryInitError;

use crate::dto::error::ErrorResponse;

/// System initialization errors
#[derive(Debug, Error, Diagnostic)]
pub enum InitError {
  #[error("Failed to initialize logger: {0}")]
  #[diagnostic(code(cas::errors::init::logger))]
  Logger(#[from] TryInitError),

  #[error("Invalid CORS origin: {0}")]
  #[diagnostic(code(cas::errors::init::cors))]
  CorsOrigin(InvalidHeaderValue),

  #[error("Failed to bind server to address: {0}")]
  #[diagnostic(code(cas::errors::init::bind))]
  Bind(std::io::Error),

  #[error("Failed to setup signal handlers")]
  #[diagnostic(code(cas::errors::init::signal))]
  SignalHandler,
}

/// Database related errors
#[derive(Debug, Error, Diagnostic)]
pub enum DatabaseError {}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

impl DatabaseError {
  /// Get the appropriate status code for this error
  pub fn status_code(&self) -> StatusCode {
    match self {
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl IntoResponse for DatabaseError {
  fn into_response(self) -> Response {
    error!("Database error: {}", self);

    let error_response =
      ErrorResponse::new("Database error occurred", &self.code().unwrap().to_string())
        .with_details(&self.to_string());

    (self.status_code(), error_response).into_response()
  }
}
