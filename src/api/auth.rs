use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use axum_valid::Valid;

use crate::{
  app_state::AppState,
  dto::{
    error::ErrorResponse,
    user::{LoginDto, SignUpDto, UserWithTokenResponse},
  },
  errors::common::WithValidationRejection,
  helpers::with_rejection::WithRejection,
};

#[utoipa::path(
  post,
  path = "/user/register",
  tags = ["Auth"],
  description = "Signup with login and password",
  responses(
    (status = 200, description = "User registered successfully", body = UserWithTokenResponse),
    (status = 400, description = "Bad request", body = ErrorResponse),
  ),
)]
#[axum::debug_handler]
pub async fn register(
  State(app_state): State<Arc<AppState>>,
  WithRejection(Valid(Json(signup_dto)), _): WithValidationRejection<Valid<Json<SignUpDto>>>,
) -> impl IntoResponse {
  app_state
    .user_service
    .sign_up(signup_dto)
    .await
    .map(|user_with_token_dto| Json(UserWithTokenResponse::from(user_with_token_dto)))
}

#[utoipa::path(
  post,
  path = "/login",
  tags = ["Auth"],
  description = "Login with login and password",
  responses(
    (status = 200, description = "User logged in successfully", body = UserWithTokenResponse),
    (status = 400, description = "Bad request", body = ErrorResponse),
  ),
)]
#[axum::debug_handler]
pub async fn login(
  State(app_state): State<Arc<AppState>>,
  WithRejection(Valid(Json(login_dto)), _): WithValidationRejection<Valid<Json<LoginDto>>>,
) -> impl IntoResponse {
  app_state
    .user_service
    .login(login_dto)
    .await
    .map(|user_with_token_dto| Json(UserWithTokenResponse::from(user_with_token_dto)))
}
