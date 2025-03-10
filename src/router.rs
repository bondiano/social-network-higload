use axum::{
  extract::Request,
  http::{header, StatusCode},
  middleware,
  response::IntoResponse,
  Router,
};
use std::sync::Arc;
use utoipa::{
  openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
  Modify, OpenApi,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
  api::{auth, health, me, users},
  middlewares::user_auth::{require_user_authentication, REFRESH_AUTH_HEADER},
  AppState,
};

#[derive(OpenApi)]
#[openapi(
  modifiers(&SecurityAddon),
  tags((name = "Social Network", description = "Social Network operations"))
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    if let Some(components) = openapi.components.as_mut() {
      components.add_security_schemes_from_iter([
        (
          "user_auth",
          SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(
            header::AUTHORIZATION.as_str(),
          ))),
        ),
        (
          "refresh_auth",
          SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(REFRESH_AUTH_HEADER))),
        ),
      ]);
    }
  }
}

#[tracing::instrument]
#[axum::debug_handler]
async fn handle_404(req: Request) -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "{\"message\":\"Not Found\"}")
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
  let user_router = OpenApiRouter::new()
    .routes(routes!(me::get_me))
    .route_layer(middleware::from_fn_with_state(
      app_state.clone(),
      require_user_authentication,
    ));

  let router = OpenApiRouter::new()
    .routes(routes!(users::get_user))
    .routes(routes!(auth::register))
    .routes(routes!(auth::login))
    .with_state(app_state.clone());

  let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .nest(
      "/api",
      OpenApiRouter::new()
        .routes(routes!(health::health))
        .merge(router)
        .merge(user_router)
        .with_state(app_state),
    )
    .split_for_parts();

  router
    .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", api.clone()))
    .fallback(handle_404)
}
