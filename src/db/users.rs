use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<User> {
  let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(pool)
    .await?;
  Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User> {
  let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
    .fetch_one(pool)
    .await?;
  Ok(user)
}

pub async fn create_user(
  pool: &PgPool,
  name: &str,
  surname: Option<&str>,
  email: &str,
  password_hash: &str,
) -> Result<Uuid> {
  let id = Uuid::new_v4();
  let now = chrono::Utc::now();
  sqlx::query!("
    INSERT INTO users (id, name, surname, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
      id,
      name,
      surname,
      email,
      password_hash,
      now,
      now
  )
    .execute(pool)
    .await?;
  Ok(id)
}

pub async fn update_user_no_password(
  pool: &PgPool,
  user_id: Uuid,
  name: &str,
  surname: Option<&str>,
  email: &str,
) -> Result<()> {
  let now = chrono::Utc::now();
  sqlx::query!(
    "
    UPDATE users SET name = $1, surname = $2, email = $3, updated_at = $4 WHERE id = $5",
    name,
    surname,
    email,
    now,
    user_id
  )
  .execute(pool)
  .await?;
  Ok(())
}

pub async fn update_user(
  pool: &PgPool,
  user_id: Uuid,
  name: &str,
  surname: Option<&str>,
  email: &str,
  password_hash: &str,
) -> Result<()> {
  let now = chrono::Utc::now();
  sqlx::query!(
    "
    UPDATE users SET name = $1, surname = $2, email = $3, password = $4, updated_at = $5 WHERE id = $6",
    name,
    surname,
    email,
    password_hash,
    now,
    user_id
  )
  .execute(pool)
  .await?;
  Ok(())
}
