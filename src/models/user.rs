use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub surname: Option<String>,
  pub email: String,
  pub password: String,
  pub last_login: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicUser {
  pub id: Uuid,
  pub name: String,
  pub surname: Option<String>,
  pub email: String,
  pub last_login: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
  fn from(value: User) -> Self {
    let User {
      id,
      name,
      surname,
      email,
      password: _,
      last_login,
      created_at,
      updated_at,
    } = value;
    Self {
      id,
      name,
      surname,
      email,
      last_login,
      created_at,
      updated_at,
    }
  }
}
