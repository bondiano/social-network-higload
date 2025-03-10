use std::sync::Arc;

use axum::{
  extract::{Path, State},
  response::IntoResponse,
  Json,
};

use crate::{
  app_state::AppState,
  dto::{error::ErrorResponse, user::UserResponse},
};

#[utoipa::path(
  get,
  path = "/user/get/{id}",
  tags = ["User"],
  params(
    ("id" = String, Path, description = "User info by ID")
  ),
  responses(
    (status = 200, description = "User info", body = UserResponse),
    (status = 400, description = "Bad request", body = ErrorResponse),
  ),
)]
#[axum::debug_handler]
pub async fn get_user(
  State(app_state): State<Arc<AppState>>,
  Path(id): Path<i32>,
) -> impl IntoResponse {
  app_state
    .user_service
    .get_by_id(id)
    .await
    .map(|user| Json(UserResponse::from(user)))
}
