use std::sync::Arc;

use axum::{
  extract::{Request, State},
  http::header,
  middleware::Next,
  response::IntoResponse,
};

use crate::{
  app_state::AppState,
  dto::user::AuthTokens,
  errors::auth::AuthError,
  services::jwt::{JwtData, TokenType},
};

pub const REFRESH_AUTH_HEADER: &str = "authorization-refresh-token";

#[tracing::instrument(skip(app_state, req, next))]
#[axum::debug_middleware]
pub async fn require_user_authentication(
  State(app_state): State<Arc<AppState>>,
  mut req: Request,
  next: Next,
) -> Result<impl IntoResponse, AuthError> {
  let token = req
    .headers()
    .get(header::AUTHORIZATION)
    .and_then(|auth_header| auth_header.to_str().ok())
    .and_then(|auth_value| {
      auth_value
        .strip_prefix("Bearer ")
        .map(|stripped| stripped.to_owned())
    })
    .ok_or(AuthError::NoToken("user"))?;

  let refresh_token = req
    .headers()
    .get(REFRESH_AUTH_HEADER)
    .and_then(|auth_header| auth_header.to_str().ok())
    .and_then(|auth_value| {
      auth_value
        .strip_prefix("Bearer ")
        .map(|stripped| stripped.to_owned())
    })
    .ok_or(AuthError::NoRefreshToken("user"))?;

  let JwtData { user_id, kind } = app_state
    .jwt_service
    .decode(&token)
    .await
    .map_err(|e| AuthError::InvalidToken("jwt", e.to_string()))?;

  if kind != TokenType::Access {
    return Err(AuthError::InvalidToken(
      "jwt",
      "invalid token type".to_string(),
    ));
  }

  let user_id = user_id
    .parse::<i32>()
    .map_err(|_| AuthError::InvalidToken("jwt", "parse token error".to_string()))?;
  let user = app_state
    .user_service
    .get_by_id(user_id)
    .await
    .map_err(|_| AuthError::InvalidToken("user", "user not found".to_string()))?;

  let tokens = AuthTokens {
    access_token: token,
    refresh_token,
  };

  req.extensions_mut().insert(user);
  req.extensions_mut().insert(tokens);
  Ok(next.run(req).await)
}
