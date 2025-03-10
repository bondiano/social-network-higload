use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct AuthTokens {
  pub access_token: String,
  pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserDto {
  pub id: i32,

  pub email: String,
  pub password: String,

  pub first_name: Option<String>,
  pub second_name: Option<String>,
  pub birth_date: Option<NaiveDate>,
  pub gender: Option<String>,
  pub city: Option<String>,
  pub biography: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithTokenDto {
  pub user: UserDto,
  pub tokens: AuthTokens,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UserResponse {
  pub id: i32,

  pub first_name: Option<String>,
  pub second_name: Option<String>,
  pub birth_date: Option<String>,
  pub gender: Option<String>,
  pub biography: Option<String>,
  pub city: Option<String>,
}

impl From<UserDto> for UserResponse {
  fn from(user: UserDto) -> Self {
    Self {
      id: user.id,
      first_name: user.first_name,
      second_name: user.second_name,
      birth_date: user.birth_date.map(|date| date.to_string()),
      gender: user.gender,
      biography: user.biography,
      city: user.city,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserMeResponse {
  pub id: i32,

  pub email: String,

  pub first_name: Option<String>,
  pub second_name: Option<String>,
  pub birth_date: Option<String>,
  pub gender: Option<String>,
  pub city: Option<String>,
  pub biography: Option<String>,
}

impl From<UserDto> for UserMeResponse {
  fn from(user: UserDto) -> Self {
    Self {
      id: user.id,
      email: user.email,
      first_name: user.first_name,
      second_name: user.second_name,
      birth_date: user.birth_date.map(|date| date.to_string()),
      gender: user.gender,
      city: user.city,
      biography: user.biography,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserWithTokenResponse {
  pub user: UserMeResponse,
  pub tokens: AuthTokens,
}

impl From<UserWithTokenDto> for UserWithTokenResponse {
  fn from(create_user_dto: UserWithTokenDto) -> Self {
    Self {
      user: UserMeResponse::from(create_user_dto.user),
      tokens: create_user_dto.tokens,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct SignUpDto {
  #[validate(email)]
  #[schema(example = "user@example.com", required)]
  pub email: String,

  #[validate(length(min = 6, max = 255))]
  #[schema(example = "password123", minimum = 6, required)]
  pub password: String,

  #[validate(length(min = 2, max = 255))]
  #[schema(example = "John")]
  pub first_name: Option<String>,

  #[validate(length(min = 2, max = 255))]
  #[schema(example = "Doe")]
  pub second_name: Option<String>,

  #[validate(length(min = 2, max = 255))]
  #[schema(example = "Male")]
  pub gender: Option<String>,

  #[validate(length(min = 2, max = 255))]
  #[schema(example = "New York")]
  pub city: Option<String>,

  #[validate(length(min = 2, max = 255))]
  #[schema(example = "I am a software engineer")]
  pub biography: Option<String>,

  #[validate(length(min = 2, max = 255))]
  #[schema(example = "1990-01-01")]
  pub birth_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct LoginDto {
  #[validate(email)]
  #[schema(example = "user@example.com", required)]
  pub email: String,

  #[validate(length(min = 6, max = 255))]
  #[schema(example = "password123", minimum = 6, required)]
  pub password: String,
}
