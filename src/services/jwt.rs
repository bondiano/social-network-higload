use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::config::AppConfigRc;
use crate::db::RedisClient;
use crate::dto::user::AuthTokens;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
  Access,
  Refresh,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JwtData {
  pub user_id: String,
  pub kind: TokenType,
}

impl JwtData {
  pub fn new(user_id: &str, kind: TokenType) -> Self {
    Self {
      user_id: user_id.to_string(),
      kind,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenClaims {
  pub user_id: String,
  pub kind: TokenType,
  pub iat: i64,
  pub exp: i64,
}

#[derive(Clone, Debug)]
pub struct JwtService {
  app_config: AppConfigRc,
  redis: RedisClient,
}

const BLACKLIST_PREFIX: &str = "jwt_blacklist:";

impl JwtService {
  pub fn new(app_config: AppConfigRc, redis: RedisClient) -> Self {
    Self { app_config, redis }
  }

  #[tracing::instrument(name = "build_access_token", skip(self))]
  pub fn build_access_token(&self, user_id: &str) -> Result<String, String> {
    let access_token_expiration = Duration::seconds(self.app_config.jwt_access_expiration);

    encode(
      &Header::default(),
      &TokenClaims {
        user_id: user_id.to_string(),
        kind: TokenType::Access,
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + access_token_expiration).timestamp(),
      },
      &EncodingKey::from_secret(self.app_config.jwt_secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
  }

  #[tracing::instrument(name = "build_refresh_token", skip(self))]
  pub fn build_refresh_token(&self, user_id: &str) -> Result<String, String> {
    let refresh_token_expiration = Duration::seconds(self.app_config.jwt_refresh_expiration);
    encode(
      &Header::default(),
      &TokenClaims {
        user_id: user_id.to_string(),
        kind: TokenType::Refresh,
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + refresh_token_expiration).timestamp(),
      },
      &EncodingKey::from_secret(self.app_config.jwt_secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
  }

  pub async fn build_tokens(&self, user_id: &str) -> Result<AuthTokens, String> {
    let (send, recv) = tokio::sync::oneshot::channel();
    let jwt_service = self.clone();
    let user_id = user_id.to_string();

    rayon::spawn(move || {
      let access_token = jwt_service.build_access_token(&user_id);
      let refresh_token = jwt_service.build_refresh_token(&user_id);

      match (access_token, refresh_token) {
        (Ok(access_token), Ok(refresh_token)) => {
          let _ = send.send(Ok(AuthTokens {
            access_token,
            refresh_token,
          }));
        }
        (Err(e), _) | (_, Err(e)) => {
          let _ = send.send(Err(e));
        }
      }
    });

    recv.await.map_err(|e| e.to_string())?
  }

  /// Validate a JWT token and return the user id and token type if valid
  #[tracing::instrument(name = "decode", skip(self))]
  pub async fn decode(&self, token: &str) -> Result<JwtData, String> {
    let token_data = decode::<TokenClaims>(
      token,
      &DecodingKey::from_secret(self.app_config.jwt_secret.as_bytes()),
      &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    if (Utc::now().timestamp() - token_data.claims.exp) > 0 {
      return Err("Token expired".to_string());
    }

    // Check if the token is blacklisted
    let key = format!("{}{}", BLACKLIST_PREFIX, token);
    let is_blacklisted = self
      .redis
      .lock()
      .await
      .get::<_, String>(key)
      .await
      .unwrap_or("false".to_string());
    if is_blacklisted == "true" {
      return Err("Token blacklisted".to_string());
    }

    Ok(JwtData {
      user_id: token_data.claims.user_id,
      kind: token_data.claims.kind,
    })
  }

  #[tracing::instrument(name = "invalidate", skip(self))]
  pub async fn invalidate(&self, token: &str) -> Result<(), String> {
    let key = format!("{}{}", BLACKLIST_PREFIX, token);
    let token_data = decode::<TokenClaims>(
      token,
      &DecodingKey::from_secret(self.app_config.jwt_secret.as_bytes()),
      &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    // Info from the token + 1 minute
    let expiry = token_data.claims.exp + 60;

    self
      .redis
      .lock()
      .await
      .set_ex::<_, _, ()>(key, "true", expiry as u64)
      .await
      .map_err(|e| e.to_string())?;

    Ok(())
  }
}
