use crate::{
  config::AppConfigRc,
  db::DataSource,
  errors::common::DatabaseError,
  services::{encryption::EncryptionService, jwt::JwtService, users::UserService},
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub ds: DataSource,
  pub user_service: UserService,
  pub jwt_service: JwtService,
}

impl AppState {
  pub async fn init(ds: DataSource, app_config: AppConfigRc) -> Result<Self, DatabaseError> {
    let jwt_service = JwtService::new(app_config.clone(), ds.redis.clone());
    let encryption_service = EncryptionService::new();
    let user_service = UserService::new(ds.pg.clone(), jwt_service.clone(), encryption_service);

    Ok(Self {
      ds,
      user_service,
      jwt_service,
    })
  }
}
