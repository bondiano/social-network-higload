use axum::{http::StatusCode, response::IntoResponse};

use crate::dto::error::ErrorResponse;

#[utoipa::path(
  get,
  tags = ["Internal"],
  path = "/health",
  responses(
    (status = 200, description = "Health check"),
    (status = 400, description = "Bad request", body = ErrorResponse),
  ),
)]
#[axum::debug_handler]
pub async fn health() -> impl IntoResponse {
  StatusCode::OK
}
