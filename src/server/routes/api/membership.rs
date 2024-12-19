use actix_web::{
  get, post,
  web::{Data, Json, Path, ReqData},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  auth::user::AuthUser,
  db::membership::{create_sub, get_all_subs, get_memberships_by_user_id},
  error::Result,
  models::member::{Membership, Subscription, SubscriptionPublic},
};

#[get("/membership/list")]
pub async fn get_available(
  pool: Data<PgPool>,
  _auth: ReqData<AuthUser>,
) -> Result<Json<Vec<Subscription>>> {
  Ok(Json(get_all_subs(&pool).await?))
}

#[get("/membership/self")]
pub async fn get_self(
  pool: Data<PgPool>,
  auth: ReqData<AuthUser>,
) -> Result<Json<Vec<Membership>>> {
  Ok(Json(get_memberships_by_user_id(&pool, auth.user_id).await?))
}

#[get("/membership/{user_id}")]
pub async fn get_other(
  pool: Data<PgPool>,
  _auth: ReqData<AuthUser>,
  user_id: Path<Uuid>,
) -> Result<Json<Vec<Membership>>> {
  Ok(Json(get_memberships_by_user_id(&pool, *user_id).await?))
}

#[post("/membership")]
pub async fn create_new(
  pool: Data<PgPool>,
  auth: ReqData<AuthUser>,
  sub: Json<SubscriptionPublic>,
) -> Result<Json<()>> {
  let SubscriptionPublic {
    access_level,
    price,
    period,
    custom_period,
  } = sub.0;
  dbg!("TODO", &auth);
  create_sub(&pool, access_level, price, period, custom_period.as_deref()).await?;
  Ok(Json(()))
}
