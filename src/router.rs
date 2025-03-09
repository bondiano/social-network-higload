use axum::{extract::Request, http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{api::health, AppState};

#[derive(OpenApi)]
#[openapi(
  tags((name = "Social Network", description = "Social Network operations"))
)]
pub struct ApiDoc;

#[tracing::instrument]
#[axum::debug_handler]
async fn handle_404(req: Request) -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "{\"message\":\"Not Found\"}")
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
  let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .nest(
      "/api",
      OpenApiRouter::new()
        .routes(routes!(health::health))
        .with_state(app_state),
    )
    .split_for_parts();

  router
    .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", api.clone()))
    .fallback(handle_404)
}
