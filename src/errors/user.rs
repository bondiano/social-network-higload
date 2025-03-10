use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use miette::Diagnostic;
use thiserror::Error;
use tracing::{error, warn};

use crate::dto::error::ErrorResponse;

#[derive(Debug, Error, Diagnostic)]
pub enum UserError {
  #[error("Failed to create user")]
  #[diagnostic(code(sn::errors::user::failed_to_create_user))]
  FailedToCreateUser(sqlx::Error),

  #[error("Failed to get user")]
  #[diagnostic(code(sn::errors::user::failed_to_find_user))]
  FailedToFindUser(sqlx::Error),

  #[error("User already exists")]
  #[diagnostic(code(sn::errors::user::user_already_exists))]
  UserAlreadyExists,

  #[error("User not found: {0}")]
  #[diagnostic(code(sn::errors::user::user_not_found))]
  UserNotFound(String),

  #[error("Failed to hash password")]
  #[diagnostic(code(sn::errors::user::password_hash_error))]
  PasswordHashError(String),

  #[error("Invalid password")]
  #[diagnostic(code(sn::errors::user::invalid_password))]
  InvalidPassword,

  #[error("Failed to build tokens: {0}")]
  #[diagnostic(code(sn::errors::user::failed_to_build_tokens))]
  FailedToBuildTokens(String),
}

pub type UserResult<T> = Result<T, UserError>;

impl UserError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Self::UserNotFound(_) => StatusCode::NOT_FOUND,
      Self::UserAlreadyExists => StatusCode::CONFLICT,
      _ => StatusCode::BAD_REQUEST,
    }
  }

  fn is_critical(&self) -> bool {
    matches!(self, Self::FailedToCreateUser(_))
  }
}

impl IntoResponse for UserError {
  fn into_response(self) -> Response {
    if self.is_critical() {
      error!("Critical user error: {:?}", self);
    } else {
      warn!("User error: {:?}", self);
    }

    let status = self.status_code();
    let error_response = match self {
      Self::FailedToFindUser(_) | Self::UserNotFound(_) => {
        ErrorResponse::new("User not found", "sn::errors::user::not_found")
      }

      Self::FailedToBuildTokens(_) => ErrorResponse::new(
        "Failed to build tokens",
        "sn::errors::user::failed_to_build_tokens",
      ),

      Self::FailedToCreateUser(_) => ErrorResponse::new(
        "Failed to create user",
        "sn::errors::user::failed_to_create_user",
      ),

      Self::PasswordHashError(_) => ErrorResponse::new(
        "Failed to hash password",
        "sn::errors::user::password_hash_error",
      ),

      Self::InvalidPassword => {
        ErrorResponse::new("Invalid password", "sn::errors::user::invalid_password")
      }

      Self::UserAlreadyExists => ErrorResponse::new(
        "User already exists",
        "sn::errors::user::user_already_exists",
      ),
    };

    (status, error_response).into_response()
  }
}
