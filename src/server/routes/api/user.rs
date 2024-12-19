use actix_web::{
  get, put,
  web::{Data, Json, Path, ReqData},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  auth::{password_hash::hash_password, user::AuthUser},
  db::users::{get_user_by_id, update_user, update_user_no_password},
  error::Result,
  models::{auth::UserInfo, user::PublicUser},
};

#[get("/user/self")]
async fn get_self(pool: Data<PgPool>, auth: ReqData<AuthUser>) -> Result<Json<PublicUser>> {
  let pool = &pool;
  let user = get_user_by_id(pool, auth.user_id).await?;
  Ok(Json(user.into()))
}

#[get("/user/{user_id}")]
async fn get_other(
  pool: Data<PgPool>,
  auth: ReqData<AuthUser>,
  user_id: Path<Uuid>,
) -> Result<Json<PublicUser>> {
  let pool = &pool;
  dbg!("TODO", &auth);
  let user = get_user_by_id(pool, user_id.into_inner()).await?;
  Ok(Json(user.into()))
}

#[put("/user/self")]
async fn update_self(
  pool: Data<PgPool>,
  auth: ReqData<AuthUser>,
  new_user: Json<UserInfo>,
) -> Result<Json<()>> {
  let pool = &pool;
  let UserInfo {
    name,
    surname,
    email,
    password,
  } = new_user.0;
  if password.is_empty() {
    update_user_no_password(pool, auth.user_id, &name, surname.as_deref(), &email).await?;
    return Ok(Json(()));
  }
  let password_hash = hash_password(&password)?;
  update_user(
    pool,
    auth.user_id,
    &name,
    surname.as_deref(),
    &email,
    &password_hash,
  )
  .await?;
  Ok(Json(()))
}

#[put("/user/{user_id}")]
async fn update_other(
  pool: Data<PgPool>,
  auth: ReqData<AuthUser>,
  user_id: Path<Uuid>,
  new_user: Json<UserInfo>,
) -> Result<Json<()>> {
  let pool = &pool;
  let user_id = user_id.into_inner();

  dbg!("TODO", &auth);

  let UserInfo {
    name,
    surname,
    email,
    password,
  } = new_user.0;
  let password_hash = hash_password(&password)?;
  update_user(
    pool,
    user_id,
    &name,
    surname.as_deref(),
    &email,
    &password_hash,
  )
  .await?;
  Ok(Json(()))
}
