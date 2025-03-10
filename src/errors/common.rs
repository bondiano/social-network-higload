use axum::{
  http::{header::InvalidHeaderValue, StatusCode},
  response::{IntoResponse, Response},
};

use miette::Diagnostic;
use thiserror::Error;
use tracing::{error, warn};
use tracing_subscriber::util::TryInitError;

use crate::{dto::error::ErrorResponse, helpers::with_rejection::WithRejection};

/// System initialization errors
#[derive(Debug, Error, Diagnostic)]
pub enum InitError {
  #[error("Failed to initialize logger: {0}")]
  #[diagnostic(code(sn::errors::init::logger))]
  Logger(#[from] TryInitError),

  #[error("Invalid CORS origin: {0}")]
  #[diagnostic(code(sn::errors::init::cors))]
  CorsOrigin(InvalidHeaderValue),

  #[error("Failed to bind server to address: {0}")]
  #[diagnostic(code(sn::errors::init::bind))]
  Bind(std::io::Error),

  #[error("Failed to setup signal handlers")]
  #[diagnostic(code(sn::errors::init::signal))]
  SignalHandler,
}

/// Database related errors
#[derive(Debug, Error, Diagnostic)]
pub enum DatabaseError {
  #[error("Failed to connect to database: {0}")]
  #[diagnostic(code(sn::errors::database::connect))]
  PostgresConnect(#[from] sqlx::Error),

  #[error("Failed to connect to redis: {0}")]
  #[diagnostic(code(sn::errors::database::connect))]
  RedisConnect(#[from] redis::RedisError),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

impl DatabaseError {
  /// Get the appropriate status code for this error
  pub fn status_code(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
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

/// Validation related errors
#[derive(Debug, Error, Diagnostic)]
pub enum ValidationError {
  #[error("{0}")]
  #[diagnostic(code(sn::errors::validation::invalid))]
  Invalid(String),

  #[error("Missing required field: {0}")]
  #[diagnostic(code(sn::errors::validation::missing_field))]
  MissingField(String),

  #[error("Invalid format for {field}: {message}")]
  #[diagnostic(code(sn::errors::validation::format))]
  InvalidFormat { field: String, message: String },

  #[error("Value out of range for {field}: {message}")]
  #[diagnostic(code(sn::errors::validation::range))]
  OutOfRange { field: String, message: String },
}

impl ValidationError {
  pub fn invalid(msg: impl Into<String>) -> Self {
    Self::Invalid(msg.into())
  }

  pub fn missing_field(field: impl Into<String>) -> Self {
    Self::MissingField(field.into())
  }

  pub fn invalid_format(field: impl Into<String>, message: impl Into<String>) -> Self {
    Self::InvalidFormat {
      field: field.into(),
      message: message.into(),
    }
  }

  pub fn status_code(&self) -> StatusCode {
    StatusCode::BAD_REQUEST
  }
}

impl<E> From<axum_valid::ValidRejection<E>> for ValidationError
where
  E: std::error::Error,
{
  fn from(error: axum_valid::ValidRejection<E>) -> Self {
    ValidationError::Invalid(error.to_string())
  }
}

impl IntoResponse for ValidationError {
  fn into_response(self) -> Response {
    warn!("Validation error: {}", self);

    let error_response = match &self {
      ValidationError::Invalid(msg) => {
        ErrorResponse::new("Validation error", "sn::errors::validation::invalid").with_details(msg)
      }

      ValidationError::MissingField(field) => ErrorResponse::new(
        &format!("Missing required field: {}", field),
        "sn::errors::validation::missing_field",
      ),

      ValidationError::InvalidFormat { field, message } => ErrorResponse::new(
        &format!("Invalid format for field: {}", field),
        "sn::errors::validation::format",
      )
      .with_details(message),

      ValidationError::OutOfRange { field, message } => ErrorResponse::new(
        &format!("Value out of range for field: {}", field),
        "sn::errors::validation::range",
      )
      .with_details(message),
    };

    (self.status_code(), error_response).into_response()
  }
}

pub type WithValidationRejection<E> = WithRejection<E, ValidationError>;
