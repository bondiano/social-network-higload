use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use miette::Diagnostic;
use thiserror::Error;
use tracing::{error, warn};

use crate::dto::error::ErrorResponse;

#[derive(Debug, Error, Diagnostic)]
pub enum AuthError {
  #[error("Authentication required: {0}")]
  #[diagnostic(code(sn::errors::auth::no_token))]
  NoToken(&'static str),

  #[error("Refresh token required for {0}")]
  #[diagnostic(code(sn::errors::auth::no_refresh_token))]
  NoRefreshToken(&'static str),

  #[error("Invalid {0} token: {1}")]
  #[diagnostic(code(sn::errors::auth::invalid_token))]
  InvalidToken(&'static str, String),

  #[error("Invalid authentication method")]
  #[diagnostic(code(sn::errors::auth::invalid_auth_method))]
  InvalidAuthMethod,

  #[error("No authenticated user found")]
  #[diagnostic(code(sn::errors::auth::no_authenticated_user))]
  NoAuthenticatedUser,

  #[error("Two-factor authentication required")]
  #[diagnostic(code(sn::errors::auth::otp_required))]
  OtpRequired,

  #[error("Permission denied: {0}")]
  #[diagnostic(code(sn::errors::auth::permission_denied))]
  PermissionDenied(String),

  #[error("Session expired")]
  #[diagnostic(code(sn::errors::auth::session_expired))]
  SessionExpired,
}

impl AuthError {
  /// Helper to create an InvalidToken error
  pub fn invalid_token(token_type: &'static str, details: impl Into<String>) -> Self {
    Self::InvalidToken(token_type, details.into())
  }

  /// Helper to create a PermissionDenied error
  pub fn permission_denied(msg: impl Into<String>) -> Self {
    Self::PermissionDenied(msg.into())
  }

  /// Get the appropriate status code for this error
  pub fn status_code(&self) -> StatusCode {
    match self {
      Self::NoToken(_)
      | Self::NoRefreshToken(_)
      | Self::InvalidToken(_, _)
      | Self::NoAuthenticatedUser
      | Self::SessionExpired => StatusCode::UNAUTHORIZED,

      Self::InvalidAuthMethod | Self::OtpRequired => StatusCode::BAD_REQUEST,

      Self::PermissionDenied(_) => StatusCode::FORBIDDEN,
    }
  }

  /// Returns true if the error should trigger a client logout
  pub fn should_logout(&self) -> bool {
    matches!(
      self,
      Self::SessionExpired | Self::InvalidToken(_, _) | Self::NoAuthenticatedUser
    )
  }
}

impl IntoResponse for AuthError {
  fn into_response(self) -> Response {
    match &self {
      Self::InvalidToken(_, _) | Self::SessionExpired => {
        warn!("Authentication error: {:?}", self)
      }
      Self::PermissionDenied(_) => {
        error!("Authorization error: {:?}", self)
      }
      _ => warn!("Auth error: {:?}", self),
    }

    let status = self.status_code();
    let error_response = match &self {
      AuthError::InvalidToken(token_type, details) => ErrorResponse::new(
        &format!("Invalid {} token", token_type),
        "sn::errors::auth::invalid_token",
      )
      .with_details(details),

      AuthError::PermissionDenied(msg) => ErrorResponse::new(
        &format!("Permission denied: {}", msg),
        "sn::errors::auth::permission_denied",
      ),

      _ => ErrorResponse::new(&self.to_string(), &self.code().unwrap().to_string()),
    };

    (status, error_response).into_response()
  }
}
