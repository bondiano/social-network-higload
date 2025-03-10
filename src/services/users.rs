use chrono::NaiveDate;
use sqlx::PgPool;

use crate::{
  dto::user::{LoginDto, SignUpDto, UserDto, UserWithTokenDto},
  errors::user::{UserError, UserResult},
  services::{encryption::EncryptionService, jwt::JwtService},
};

#[derive(Clone, Debug)]
pub struct UserService {
  db: PgPool,
  jwt_service: JwtService,
  encryption_service: EncryptionService,
}

impl UserService {
  pub fn new(db: PgPool, jwt_service: JwtService, encryption_service: EncryptionService) -> Self {
    Self {
      db,
      jwt_service,
      encryption_service,
    }
  }

  #[tracing::instrument(name = "get_by_id", skip(self))]
  pub async fn get_by_id(&self, id: i32) -> UserResult<UserDto> {
    sqlx::query_as!(UserDto, r#"SELECT * FROM users WHERE id = $1"#, id)
      .fetch_one(&self.db)
      .await
      .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::UserNotFound(id.to_string()),
        _ => UserError::FailedToFindUser(e),
      })
  }

  #[tracing::instrument(name = "get_by_email", skip(self))]
  pub async fn get_by_email(&self, email: &str) -> UserResult<UserDto> {
    sqlx::query_as!(UserDto, r#"SELECT * FROM users WHERE email = $1"#, email)
      .fetch_one(&self.db)
      .await
      .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::UserNotFound(email.to_string()),
        _ => UserError::FailedToFindUser(e),
      })
  }

  #[tracing::instrument(name = "sign_up", skip(self))]
  pub async fn sign_up(&self, signup_dto: SignUpDto) -> UserResult<UserWithTokenDto> {
    let hashed_password = self.hash_password(signup_dto.password).await?;
    let user = UserDto {
      email: signup_dto.email,
      password: hashed_password,
      first_name: signup_dto.first_name,
      second_name: signup_dto.second_name,
      gender: signup_dto.gender,
      city: signup_dto.city,
      biography: signup_dto.biography,
      birth_date: signup_dto
        .birth_date
        .map(|date| date.parse::<NaiveDate>().unwrap()),
      ..Default::default()
    };

    let user = sqlx::query_as!(
      UserDto,
      r#"INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *"#,
      user.email,
      user.password
    )
    .fetch_one(&self.db)
    .await
    .map_err(UserError::FailedToCreateUser)?;

    let tokens = self
      .jwt_service
      .build_tokens(&user.id.to_string())
      .await
      .map_err(UserError::FailedToBuildTokens)?;

    Ok(UserWithTokenDto { user, tokens })
  }

  #[tracing::instrument(name = "login", skip(self))]
  pub async fn login(&self, login_dto: LoginDto) -> UserResult<UserWithTokenDto> {
    let user = self.get_by_email(&login_dto.email).await?;
    let is_password_valid = self
      .encryption_service
      .verify_password(&login_dto.password, &user.password)
      .await;

    if !is_password_valid {
      return Err(UserError::InvalidPassword);
    }

    let tokens = self
      .jwt_service
      .build_tokens(&user.id.to_string())
      .await
      .map_err(UserError::FailedToBuildTokens)?;

    Ok(UserWithTokenDto { user, tokens })
  }

  #[tracing::instrument]
  async fn hash_password(&self, password: String) -> UserResult<String> {
    self
      .encryption_service
      .hash_password(password)
      .await
      .map_err(|e| UserError::PasswordHashError(e.to_string()))
  }
}
