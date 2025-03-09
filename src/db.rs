use crate::errors::common::DatabaseResult;

#[derive(Debug, Clone)]
pub struct DataSource {}

impl DataSource {
  pub async fn init() -> DatabaseResult<Self> {
    Ok(Self {})
  }
}
