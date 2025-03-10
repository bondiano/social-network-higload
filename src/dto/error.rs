use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct ErrorResponse {
  pub message: String,
  pub code: String,
  pub details: Option<String>,
  #[serde(skip_serializing)]
  status_code: StatusCode,
}

impl ErrorResponse {
  pub fn new(message: &str, code: &str) -> Self {
    Self {
      message: message.to_string(),
      code: code.to_string(),
      details: None,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  pub fn with_details(self, details: &str) -> Self {
    Self {
      details: Some(details.to_string()),
      ..self
    }
  }
}

impl<T> From<T> for ErrorResponse
where
  T: miette::Diagnostic + std::error::Error,
{
  fn from(error: T) -> Self {
    Self {
      message: error.to_string(),
      code: error
        .code()
        .unwrap_or_else(|| Box::new("social_network::errors::unknown"))
        .to_string(),
      details: None,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl IntoResponse for ErrorResponse {
  fn into_response(self) -> Response {
    // Create response with status code and JSON body
    (
      self.status_code,
      serde_json::to_string(&self).unwrap_or_else(|_| {
        String::from(
          r#"{"message":"Failed to serialize error","code":"social_network::errors::internal"}"#,
        )
      }),
    )
      .into_response()
  }
}
