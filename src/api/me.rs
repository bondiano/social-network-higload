use axum::{response::IntoResponse, Extension, Json};

use crate::dto::{
  error::ErrorResponse,
  user::{UserDto, UserMeResponse},
};

#[utoipa::path(
  get,
  tags = ["Auth"],
  path = "/me",
  responses(
    (status = 200, description = "Current user info", body = UserMeResponse),
    (status = 400, description = "Bad request", body = ErrorResponse),
  ),
  security(
    ("user_auth" = []),
    ("refresh_auth" = []),
    ("partner_auth" = [])
  ),
)]
#[axum::debug_handler]
#[tracing::instrument]
pub async fn get_me(Extension(user): Extension<UserDto>) -> impl IntoResponse {
  Json(UserMeResponse::from(user))
}
