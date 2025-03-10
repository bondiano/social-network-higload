#[derive(Clone, Debug)]
pub struct EncryptionService;

impl Default for EncryptionService {
  fn default() -> Self {
    Self::new()
  }
}

impl EncryptionService {
  pub fn new() -> Self {
    Self
  }

  /// Consume password value to make it unusable
  #[tracing::instrument(name = "hash_password", skip(self))]
  pub async fn hash_password(
    &self,
    password: String,
  ) -> Result<String, tokio::sync::oneshot::error::RecvError> {
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
      let hash = password_auth::generate_hash(password);
      let _ = send.send(hash);
    });

    recv.await
  }

  pub async fn verify_password(&self, password: &str, hash: &str) -> bool {
    let (send, recv) = tokio::sync::oneshot::channel();
    let password = password.to_string();
    let hash = hash.to_string();

    rayon::spawn(move || {
      let verify_result = password_auth::verify_password(password, &hash);

      let is_valid = verify_result.is_ok();
      let _ = send.send(is_valid);
    });

    recv.await.unwrap_or(false)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_hash_verify_password() {
    let encryption_service = EncryptionService::new();
    let hash = encryption_service
      .hash_password("password".to_string())
      .await
      .unwrap();

    let is_valid = encryption_service.verify_password("password", &hash).await;
    assert!(is_valid);
  }
}
