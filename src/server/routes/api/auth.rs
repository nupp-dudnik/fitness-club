use actix_web::{
  get, post,
  web::{Data, Json, ReqData},
};
use sqlx::PgPool;

use crate::{
  auth::{
    claims::Claims,
    password_hash::{hash_password, verify_password},
    user::AuthUser,
  },
  db::users::{create_user, get_user_by_email},
  error::{Error, Result},
  models::auth::{Login, Token, UserInfo},
};

#[post("/login")]
async fn login(pool: Data<PgPool>, credentials: Json<Login>) -> Result<Json<Token>> {
  let pool: &PgPool = &pool;
  let Login { email, password } = credentials.0;
  let user = get_user_by_email(pool, &email)
    .await
    .map_err(|_| Error::UserDoesNotExist)?;

  verify_password(&password, &user.password).map_err(|_| Error::InvalidPassword)?;
  let token = Claims::jwt(user.id)?;

  Ok(Json(Token { token }))
}

#[post("/register")]
async fn register(pool: Data<PgPool>, credentials: Json<UserInfo>) -> Result<Json<Token>> {
  let pool: &PgPool = &pool;
  let UserInfo {
    name,
    surname,
    email,
    password,
  } = credentials.0;
  get_user_by_email(pool, email.trim())
    .await
    .is_err()
    .then_some(())
    .ok_or(Error::EmailUsed)?;
  let password_hash = hash_password(&password)?;
  let id = create_user(
    pool,
    &name,
    surname.as_deref(),
    email.trim(),
    &password_hash,
  )
  .await?;
  let token = Claims::jwt(id)?;
  Ok(Json(Token { token }))
}

#[get("/refresh")]
async fn refresh(auth: ReqData<AuthUser>) -> Result<Json<Token>> {
  let token = Claims::jwt(auth.user_id)?;
  Ok(Json(Token { token }))
}
