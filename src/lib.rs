use app_state::AppState;
use axum::{
  extract::MatchedPath,
  http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderName, HeaderValue, Method, Request,
  },
  Router,
};
use config::AppConfig;
use db::DataSource;
use errors::common::InitError;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
  cors::CorsLayer,
  request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
  trace::{self, TraceLayer},
};
use tracing::{info_span, Level};

pub mod app_state;
pub mod config;
pub mod db;
pub mod errors;

mod api;
mod dto;
mod helpers;
mod middlewares;
mod router;
mod services;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn app(app_config: AppConfig) -> miette::Result<Router> {
  let cors = CorsLayer::new()
    .allow_origin(
      app_config
        .host
        .parse::<HeaderValue>()
        .map_err(InitError::CorsOrigin)?,
    )
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

  let middleware_stack = ServiceBuilder::new()
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
          // Log the request id as generated.
          let request_id = match request.headers().get(REQUEST_ID_HEADER) {
            Some(request_id) => request_id,
            None => &HeaderValue::from_static("no request id"),
          };

          let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);
          info_span!(
              "http_request",
              matched_path,
              path =? request.uri(),
              method =? request.method(),
              request_id =? request_id,
          )
        })
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    )
    .layer(cors)
    .layer(PropagateRequestIdLayer::new(x_request_id));

  let db = DataSource::init(&app_config.database_url, &app_config.redis_url).await?;
  let app_state = AppState::init(db, Arc::new(app_config)).await?;

  let app = router::create_router(Arc::new(app_state)).layer(middleware_stack);

  Ok(app)
}
