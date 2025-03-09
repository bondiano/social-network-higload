use crate::{config::AppConfigRc, db::DataSource, errors::common::DatabaseError};

#[derive(Debug, Clone)]
pub struct AppState {
  pub ds: DataSource,
}

impl AppState {
  pub async fn init(ds: DataSource, _app_config: AppConfigRc) -> Result<Self, DatabaseError> {
    Ok(Self { ds })
  }
}
